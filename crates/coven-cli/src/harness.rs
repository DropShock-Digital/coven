use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

const EXTERNAL_ADAPTER_SCHEMA: &str = "coven.harness-adapter.v1";
const EXTERNAL_ADAPTER_DIR: &str = "harness-adapters";
const MAX_ARG_COUNT: usize = 32;
const MAX_ARG_LEN: usize = 256;
const MAX_LABEL_LEN: usize = 80;
const MAX_HINT_LEN: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessAdapterSource {
    /// Compatibility adapters Coven ships as the v0 default set.
    BuiltInDefault,
    /// Data-backed adapters loaded from the external-adapter manifest path.
    /// These are routable by explicit id, but they are not default fallbacks
    /// and should not grow daemon/TUI/client special cases.
    ExternalManifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessSummary {
    pub id: String,
    pub label: String,
    pub executable: String,
    pub available: bool,
    pub install_hint: String,
    pub source: HarnessAdapterSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessLaunchMode {
    Interactive,
    NonInteractive,
    /// Long-lived stream-json process: stdin reads newline-delimited JSON
    /// messages, stdout writes newline-delimited JSON events. Only
    /// `claude` supports this today (`-p --input-format stream-json
    /// --output-format stream-json --verbose`).
    ///
    /// Capability is enforced at two layers:
    /// - `command_parts_for_harness_with_conversation` (the offline arg
    ///   builder): codex's `stream_args` returns `None`, so the builder
    ///   falls back to non-interactive args. This makes the function
    ///   safe to call standalone.
    /// - `daemon::LiveSessionRuntime::launch_session` (the live runtime):
    ///   explicitly rejects stream-mode launches when
    ///   `harness_supports_stream_mode(harness)` is false, returning a
    ///   structured `500 launch_failed` so the client sees the actual
    ///   constraint instead of a silently-downgraded behavior. The
    ///   chat layer is the only caller that requests Stream today and
    ///   already gates on `harness_supports_stream_mode` before doing so.
    Stream,
}

/// Whether the harness CLI has a long-lived JSON-streaming mode the daemon
/// can keep alive across chat turns. Claude does (`stream-json`); codex
/// doesn't (only one-shot `codex exec`). Gated to Unix today because the
/// daemon's stream-mode kill path relies on Unix process-group semantics
/// (`setsid()` at spawn, then `kill(-pid, SIGKILL)` to tear down the
/// harness plus any subprocesses it spawned in one syscall). A Windows
/// process-tree termination path would let this widen. See
/// `docs/chat-persistence.md`.
#[cfg(unix)]
pub fn harness_supports_stream_mode(harness_id: &str) -> bool {
    built_in_harness_spec(harness_id)
        .map(|spec| spec.supports_stream_mode)
        .unwrap_or(false)
}

#[cfg(not(unix))]
pub fn harness_supports_stream_mode(_harness_id: &str) -> bool {
    false
}

/// Hint passed when a chat turn wants to participate in a multi-turn
/// conversation by reusing the underlying harness CLI's session-resume
/// mechanism. Consulted in `NonInteractive` mode (each turn cold-starts
/// claude/codex with `--resume`/`exec resume`) and in `Stream` mode (the
/// long-lived claude process is launched with `--session-id`/`--resume`
/// up front).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationHint {
    /// First turn of a conversation. The harness should create a session
    /// claimed under this id so later turns can resume it.
    Init { id: String },
    /// Subsequent turn. The harness should resume the session at this id and
    /// append the new prompt to its history.
    Resume { id: String },
}

impl ConversationHint {
    pub fn id(&self) -> &str {
        match self {
            ConversationHint::Init { id } | ConversationHint::Resume { id } => id,
        }
    }
}

/// Whether the harness CLI lets the caller pre-assign a session id at launch
/// time (e.g. `claude --session-id <uuid>`). Harnesses that auto-generate
/// session ids (e.g. codex) return `false`; the chat app captures the id from
/// the first turn's output instead. See `docs/chat-persistence.md`.
pub fn harness_supports_preassigned_session_id(harness_id: &str) -> bool {
    built_in_harness_spec(harness_id)
        .map(|spec| spec.supports_preassigned_session_id)
        .unwrap_or(false)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessCommandSpec {
    pub id: String,
    pub label: String,
    pub executable: String,
    pub interactive_prompt_prefix_args: Vec<String>,
    pub non_interactive_prompt_prefix_args: Vec<String>,
    pub install_hint: String,
    /// CLI flag name to pass a system-prompt string (e.g. `Some("--system-prompt")`
    /// for Claude). `None` means the harness has no such flag and identity
    /// should be injected by prepending a preamble to the prompt instead.
    pub system_prompt_flag: Option<String>,
    pub supports_stream_mode: bool,
    pub supports_preassigned_session_id: bool,
    pub source: HarnessAdapterSource,
    pub compatibility_notes: Option<String>,
}

impl HarnessCommandSpec {
    pub fn prompt_args(&self, prompt: &str, mode: HarnessLaunchMode) -> Vec<String> {
        let prefix_args = match mode {
            HarnessLaunchMode::Interactive => &self.interactive_prompt_prefix_args,
            HarnessLaunchMode::NonInteractive => &self.non_interactive_prompt_prefix_args,
            // Stream mode bypasses `prompt_args` entirely (no trailing
            // prompt; messages arrive on stdin). Fall back to
            // non-interactive args if a caller somehow lands here.
            HarnessLaunchMode::Stream => &self.non_interactive_prompt_prefix_args,
        };

        prefix_args
            .iter()
            .cloned()
            .chain(std::iter::once(prompt.to_string()))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
struct ExternalHarnessAdapterToml {
    schema: String,
    id: String,
    label: String,
    executable: String,
    interactive_prompt_prefix_args: Vec<String>,
    non_interactive_prompt_prefix_args: Vec<String>,
    install_hint: Option<String>,
    system_prompt_flag: Option<String>,
    supports_stream_mode: Option<bool>,
    supports_preassigned_session_id: Option<bool>,
    compatibility_notes: Option<String>,
}

pub fn built_in_harnesses() -> Vec<HarnessSummary> {
    summaries_for_specs(built_in_harness_specs())
}

pub fn external_adapter_dir(coven_home: &Path) -> PathBuf {
    coven_home.join(EXTERNAL_ADAPTER_DIR)
}

pub fn external_harnesses(coven_home: &Path) -> Result<Vec<HarnessSummary>> {
    Ok(summaries_for_specs(load_external_harness_specs(
        coven_home,
    )?))
}

pub fn registered_harnesses(coven_home: &Path) -> Result<Vec<HarnessSummary>> {
    Ok(summaries_for_specs(registered_harness_specs(coven_home)?))
}

pub fn registered_harness_specs(coven_home: &Path) -> Result<Vec<HarnessCommandSpec>> {
    let mut specs = built_in_harness_specs();
    let built_in_ids = specs.iter().map(|spec| spec.id.clone()).collect::<Vec<_>>();
    for external in load_external_harness_specs(coven_home)? {
        if specs.iter().any(|spec| spec.id == external.id) {
            anyhow::bail!(
                "external adapter `{}` conflicts with an existing harness id",
                external.id
            );
        }
        if built_in_ids.iter().any(|id| id == &external.id) {
            anyhow::bail!(
                "external adapter `{}` cannot override a built-in harness",
                external.id
            );
        }
        specs.push(external);
    }
    Ok(specs)
}

pub fn registered_harness_spec(
    coven_home: &Path,
    harness_id: &str,
) -> Result<Option<HarnessCommandSpec>> {
    Ok(registered_harness_specs(coven_home)?
        .into_iter()
        .find(|spec| spec.id == harness_id))
}

fn built_in_harness_spec(harness_id: &str) -> Option<HarnessCommandSpec> {
    built_in_harness_specs()
        .into_iter()
        .find(|spec| spec.id == harness_id)
}

fn summaries_for_specs(specs: Vec<HarnessCommandSpec>) -> Vec<HarnessSummary> {
    specs
        .into_iter()
        .map(|spec| HarnessSummary {
            id: spec.id,
            label: spec.label,
            executable: spec.executable.clone(),
            available: executable_exists(&spec.executable),
            install_hint: spec.install_hint,
            source: spec.source,
        })
        .collect()
}

/// Familiar identity context passed down from `coven run --familiar`.
/// Each harness spec decides how to surface this to the underlying CLI
/// (prompt prefix, `--system-prompt` flag, env var, etc.) so the
/// integration layer is harness-agnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamiliarContext {
    /// Canonical familiar id (e.g. `"charm"`).
    pub id: String,
    /// Human display name (e.g. `"Charm"`).
    pub display_name: String,
    /// Short role/theme description (e.g. `"Voice, Social, and Presence Familiar"`).
    pub role: Option<String>,
}

impl FamiliarContext {
    /// Render a concise identity preamble suitable for prepending to a prompt
    /// or injecting as a system-prompt block. Kept intentionally short so it
    /// doesn't crowd the actual task.
    pub fn identity_preamble(&self) -> String {
        match &self.role {
            Some(role) => format!(
                "[Identity: You are {name}, a {role}. Respond as {name}, not as the underlying tool.]",
                name = self.display_name,
                role = role,
            ),
            None => format!(
                "[Identity: You are {name}. Respond as {name}, not as the underlying tool.]",
                name = self.display_name,
            ),
        }
    }
}

pub fn built_in_harness_specs() -> Vec<HarnessCommandSpec> {
    vec![
        HarnessCommandSpec {
            id: "codex".to_string(),
            label: "Codex".to_string(),
            executable: "codex".to_string(),
            interactive_prompt_prefix_args: Vec::new(),
            non_interactive_prompt_prefix_args: vec![
                "exec".to_string(),
                "--skip-git-repo-check".to_string(),
                "--color".to_string(),
                "never".to_string(),
            ],
            install_hint: "Install Codex with `npm install -g @openai/codex` or `brew install --cask codex`; if it is already installed, make sure `codex` is on PATH and run `codex login` or `codex` once to authenticate, then retry `coven doctor`.".to_string(),
            // Codex has no --system-prompt flag; identity is injected as a
            // bracketed preamble prepended to the prompt.
            system_prompt_flag: None,
            supports_stream_mode: false,
            supports_preassigned_session_id: false,
            source: HarnessAdapterSource::BuiltInDefault,
            compatibility_notes: None,
        },
        HarnessCommandSpec {
            id: "claude".to_string(),
            label: "Claude Code".to_string(),
            executable: "claude".to_string(),
            interactive_prompt_prefix_args: Vec::new(),
            non_interactive_prompt_prefix_args: vec!["--print".to_string()],
            install_hint: "Install Claude Code with `npm install -g @anthropic-ai/claude-code`; if it is already installed, make sure `claude` is on PATH and run `claude doctor` to finish local auth/setup, then retry `coven doctor`.".to_string(),
            system_prompt_flag: Some("--system-prompt".to_string()),
            supports_stream_mode: true,
            supports_preassigned_session_id: true,
            source: HarnessAdapterSource::BuiltInDefault,
            compatibility_notes: None,
        },
    ]
}

pub fn load_external_harness_specs(coven_home: &Path) -> Result<Vec<HarnessCommandSpec>> {
    let dir = external_adapter_dir(coven_home);
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(err).with_context(|| {
                format!(
                    "failed to read external harness adapter directory {}",
                    dir.display()
                )
            })
        }
    };

    let mut specs = Vec::new();
    for entry in entries {
        let entry = entry.with_context(|| format!("failed to read entry in {}", dir.display()))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }
        let raw = fs::read_to_string(&path).with_context(|| {
            format!(
                "failed to read external adapter manifest {}",
                path.display()
            )
        })?;
        let manifest: ExternalHarnessAdapterToml = toml::from_str(&raw).with_context(|| {
            format!(
                "failed to parse external adapter manifest {}",
                path.display()
            )
        })?;
        specs.push(validate_external_manifest(manifest, &path)?);
    }
    specs.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(specs)
}

fn validate_external_manifest(
    manifest: ExternalHarnessAdapterToml,
    path: &Path,
) -> Result<HarnessCommandSpec> {
    if manifest.schema != EXTERNAL_ADAPTER_SCHEMA {
        anyhow::bail!(
            "external adapter manifest {} has unsupported schema `{}`; expected `{}`",
            path.display(),
            manifest.schema,
            EXTERNAL_ADAPTER_SCHEMA
        );
    }
    validate_harness_id(&manifest.id, path)?;
    if built_in_harness_specs()
        .iter()
        .any(|spec| spec.id == manifest.id)
    {
        anyhow::bail!(
            "external adapter manifest {} attempts to override built-in harness `{}`",
            path.display(),
            manifest.id
        );
    }
    validate_short_string("label", &manifest.label, MAX_LABEL_LEN, path)?;
    validate_executable(&manifest.executable, path)?;
    validate_args(
        "interactive_prompt_prefix_args",
        &manifest.interactive_prompt_prefix_args,
        path,
    )?;
    validate_args(
        "non_interactive_prompt_prefix_args",
        &manifest.non_interactive_prompt_prefix_args,
        path,
    )?;
    if let Some(flag) = &manifest.system_prompt_flag {
        validate_short_string("system_prompt_flag", flag, MAX_ARG_LEN, path)?;
    }
    if let Some(hint) = &manifest.install_hint {
        validate_short_string("install_hint", hint, MAX_HINT_LEN, path)?;
    }
    if let Some(notes) = &manifest.compatibility_notes {
        validate_short_string("compatibility_notes", notes, MAX_HINT_LEN, path)?;
    }
    if manifest.supports_stream_mode.unwrap_or(false) {
        anyhow::bail!(
            "external adapter manifest {} requests stream mode, but generic external adapters only support one-shot prompt argv today",
            path.display()
        );
    }
    if manifest.supports_preassigned_session_id.unwrap_or(false) {
        anyhow::bail!(
            "external adapter manifest {} requests preassigned session ids, but generic external adapters only support one-shot prompt argv today",
            path.display()
        );
    }

    Ok(HarnessCommandSpec {
        id: manifest.id,
        label: manifest.label,
        executable: manifest.executable,
        interactive_prompt_prefix_args: manifest.interactive_prompt_prefix_args,
        non_interactive_prompt_prefix_args: manifest.non_interactive_prompt_prefix_args,
        install_hint: manifest.install_hint.unwrap_or_else(|| {
            "Install the external harness CLI and make sure its executable is on PATH.".to_string()
        }),
        system_prompt_flag: manifest.system_prompt_flag,
        supports_stream_mode: false,
        supports_preassigned_session_id: false,
        source: HarnessAdapterSource::ExternalManifest,
        compatibility_notes: manifest.compatibility_notes,
    })
}

fn validate_harness_id(id: &str, path: &Path) -> Result<()> {
    if id.is_empty()
        || id.len() > 64
        || !id
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
        || !id
            .chars()
            .next()
            .map(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
            .unwrap_or(false)
    {
        anyhow::bail!(
            "external adapter manifest {} has invalid id `{}`; expected [a-z0-9][a-z0-9_-]{{0,63}}",
            path.display(),
            id
        );
    }
    Ok(())
}

fn validate_executable(executable: &str, path: &Path) -> Result<()> {
    validate_short_string("executable", executable, MAX_ARG_LEN, path)?;
    if executable.contains('/') || executable.contains('\\') {
        anyhow::bail!(
            "external adapter manifest {} executable `{}` must be a PATH command name, not a path",
            path.display(),
            executable
        );
    }
    Ok(())
}

fn validate_args(field: &str, args: &[String], path: &Path) -> Result<()> {
    if args.len() > MAX_ARG_COUNT {
        anyhow::bail!(
            "external adapter manifest {} field `{}` has too many args; max is {}",
            path.display(),
            field,
            MAX_ARG_COUNT
        );
    }
    for arg in args {
        validate_short_string(field, arg, MAX_ARG_LEN, path)?;
    }
    Ok(())
}

fn validate_short_string(field: &str, value: &str, max_len: usize, path: &Path) -> Result<()> {
    if value.trim().is_empty() || value.len() > max_len {
        anyhow::bail!(
            "external adapter manifest {} field `{}` must be non-empty and <= {} bytes",
            path.display(),
            field,
            max_len
        );
    }
    if value.contains('\0') {
        anyhow::bail!(
            "external adapter manifest {} field `{}` contains a NUL byte",
            path.display(),
            field
        );
    }
    Ok(())
}

#[cfg(test)]
pub fn command_parts_for_harness(
    harness_id: &str,
    prompt: &str,
    mode: HarnessLaunchMode,
) -> Result<(String, Vec<String>)> {
    command_parts_for_harness_with_conversation(harness_id, prompt, mode, None, None)
}

#[cfg(test)]
pub fn command_parts_for_registered_harness(
    coven_home: &Path,
    harness_id: &str,
    prompt: &str,
    mode: HarnessLaunchMode,
) -> Result<(String, Vec<String>)> {
    command_parts_for_registered_harness_with_conversation(
        coven_home, harness_id, prompt, mode, None, None,
    )
}

/// Build a built-in harness command line, optionally injecting
/// session-continuity flags so the harness CLI resumes a prior conversation.
pub fn command_parts_for_harness_with_conversation(
    harness_id: &str,
    prompt: &str,
    mode: HarnessLaunchMode,
    hint: Option<&ConversationHint>,
    familiar: Option<&FamiliarContext>,
) -> Result<(String, Vec<String>)> {
    let spec = built_in_harness_spec(harness_id)
        .ok_or_else(|| anyhow!("unsupported built-in harness `{harness_id}`"))?;
    command_parts_from_spec(&spec, prompt, mode, hint, familiar)
}

/// Build a registered harness command line. This includes Codex/Claude built-ins
/// plus explicit external-adapter manifests loaded from `COVEN_HOME/harness-adapters/*.toml`.
pub fn command_parts_for_registered_harness_with_conversation(
    coven_home: &Path,
    harness_id: &str,
    prompt: &str,
    mode: HarnessLaunchMode,
    hint: Option<&ConversationHint>,
    familiar: Option<&FamiliarContext>,
) -> Result<(String, Vec<String>)> {
    let spec = registered_harness_spec(coven_home, harness_id)?
        .ok_or_else(|| anyhow!("unsupported registered harness `{harness_id}`"))?;
    command_parts_from_spec(&spec, prompt, mode, hint, familiar)
}

fn command_parts_from_spec(
    spec: &HarnessCommandSpec,
    prompt: &str,
    mode: HarnessLaunchMode,
    hint: Option<&ConversationHint>,
    familiar: Option<&FamiliarContext>,
) -> Result<(String, Vec<String>)> {
    // Resolve effective prompt: inject familiar identity preamble when present.
    // Harnesses with a dedicated --system-prompt flag get identity there instead,
    // keeping the task prompt clean.
    let has_system_prompt_flag = spec.system_prompt_flag.is_some();
    let effective_prompt = match familiar {
        Some(f) if !has_system_prompt_flag => {
            format!("{preamble}\n\n{prompt}", preamble = f.identity_preamble())
        }
        _ => prompt.to_string(),
    };

    // Stream mode reads prompts from stdin as JSON messages, so the prompt
    // argument is not appended. The continuity hint (claude resume / init)
    // still maps to a CLI flag; codex falls back to one-shot.
    if mode == HarnessLaunchMode::Stream {
        if let Some(mut args) = stream_args(spec, hint) {
            // Claude stream mode: inject identity via --system-prompt flag.
            if let (Some(flag), Some(f)) = (spec.system_prompt_flag.as_deref(), familiar) {
                args.insert(0, f.identity_preamble());
                args.insert(0, flag.to_string());
            }
            return Ok((spec.executable.clone(), args));
        }
        // Harness doesn't support stream: fall through to non-interactive.
        return Ok((
            spec.executable.clone(),
            spec.prompt_args(&effective_prompt, HarnessLaunchMode::NonInteractive),
        ));
    }

    if let Some(hint) = hint {
        if let Some(mut args) = continuity_args(spec, mode, hint) {
            // Inject identity via --system-prompt for harnesses that support it.
            if let (Some(flag), Some(f)) = (spec.system_prompt_flag.as_deref(), familiar) {
                args.insert(0, f.identity_preamble());
                args.insert(0, flag.to_string());
            }
            return Ok((
                spec.executable.clone(),
                args.into_iter()
                    .chain(std::iter::once(effective_prompt))
                    .collect(),
            ));
        }
    }

    let mut args = spec.prompt_args(&effective_prompt, mode);
    // Inject identity via --system-prompt for harnesses that support it,
    // prepending before the prompt args.
    if let (Some(flag), Some(f)) = (spec.system_prompt_flag.as_deref(), familiar) {
        args.insert(0, f.identity_preamble());
        args.insert(0, flag.to_string());
    }
    Ok((spec.executable.clone(), args))
}

/// Per-harness translation of stream-mode launch into CLI args. Stream-mode
/// processes are long-lived: stdin is a stream of newline-delimited JSON
/// messages and stdout is a stream of newline-delimited JSON events.
/// Returns `None` for harnesses that don't support stream mode so the
/// caller can fall back to a one-shot launch.
fn stream_args(spec: &HarnessCommandSpec, hint: Option<&ConversationHint>) -> Option<Vec<String>> {
    match spec.id.as_str() {
        "claude" if spec.supports_stream_mode => {
            let mut args: Vec<String> = vec![
                "--print".to_string(),
                "--input-format".to_string(),
                "stream-json".to_string(),
                "--output-format".to_string(),
                "stream-json".to_string(),
                "--verbose".to_string(),
            ];
            if let Some(hint) = hint {
                let flag = match hint {
                    ConversationHint::Init { .. } => "--session-id",
                    ConversationHint::Resume { .. } => "--resume",
                };
                args.push(flag.to_string());
                args.push(hint.id().to_string());
            }
            Some(args)
        }
        _ => None,
    }
}

/// Per-harness translation of a `ConversationHint` into CLI args that precede
/// the prompt. Returns `None` when the harness has no resume support (or when
/// the launch mode doesn't support it) so the caller falls back to defaults.
fn continuity_args(
    spec: &HarnessCommandSpec,
    mode: HarnessLaunchMode,
    hint: &ConversationHint,
) -> Option<Vec<String>> {
    // Continuity only makes sense in non-interactive mode today. Interactive
    // mode launches the harness TUI, which has its own resume picker.
    if mode != HarnessLaunchMode::NonInteractive {
        return None;
    }
    match spec.id.as_str() {
        "claude" if spec.supports_stream_mode => {
            let flag = match hint {
                ConversationHint::Init { .. } => "--session-id",
                ConversationHint::Resume { .. } => "--resume",
            };
            Some(vec![
                "--print".to_string(),
                flag.to_string(),
                hint.id().to_string(),
            ])
        }
        "codex" => match hint {
            // Codex auto-assigns the session id on the first turn; we capture
            // it from output and feed it back on subsequent turns.
            ConversationHint::Init { .. } => None,
            ConversationHint::Resume { id } => {
                let mut args: Vec<String> = spec.non_interactive_prompt_prefix_args.clone();
                args.push("resume".to_string());
                args.push(id.clone());
                Some(args)
            }
        },
        _ => None,
    }
}

fn executable_exists(executable: &str) -> bool {
    env::var_os("PATH")
        .map(|paths| executable_exists_in_paths(executable, env::split_paths(&paths)))
        .unwrap_or(false)
}

fn executable_exists_in_paths<I>(executable: &str, paths: I) -> bool
where
    I: IntoIterator<Item = PathBuf>,
{
    if executable.contains('/') || executable.contains('\\') {
        return false;
    }

    paths.into_iter().any(|path| {
        executable_candidates(&path, executable)
            .any(|candidate| candidate_is_executable(&candidate))
    })
}

#[cfg(unix)]
fn candidate_is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.metadata()
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn candidate_is_executable(path: &Path) -> bool {
    path.is_file()
}

#[cfg(windows)]
fn executable_candidates<'a>(
    path: &'a Path,
    executable: &'a str,
) -> impl Iterator<Item = PathBuf> + 'a {
    let extensions = env::var_os("PATHEXT")
        .map(|value| {
            env::split_paths(&value)
                .map(|path| path.to_string_lossy().into_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![".COM".into(), ".EXE".into(), ".BAT".into(), ".CMD".into()]);

    let base = path.join(executable);
    let has_extension = Path::new(executable).extension().is_some();
    std::iter::once(base.clone()).chain(extensions.into_iter().filter_map(move |extension| {
        if has_extension {
            None
        } else {
            Some(path.join(format!("{executable}{extension}")))
        }
    }))
}

#[cfg(not(windows))]
fn executable_candidates<'a>(
    path: &'a Path,
    executable: &'a str,
) -> impl Iterator<Item = PathBuf> + 'a {
    std::iter::once(path.join(executable))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    fn write_manifest(coven_home: &Path, name: &str, manifest: &str) -> Result<()> {
        let dir = external_adapter_dir(coven_home);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join(name), manifest)?;
        Ok(())
    }

    fn hermes_manifest() -> &'static str {
        r#"schema = "coven.harness-adapter.v1"
id = "hermes"
label = "Hermes Agent"
executable = "hermes"
interactive_prompt_prefix_args = ["chat", "--source", "coven", "-q"]
non_interactive_prompt_prefix_args = ["chat", "--source", "coven", "-Q", "-q"]
install_hint = "Install Hermes Agent and run hermes doctor."
compatibility_notes = "One-shot argv mode only."
"#
    }

    #[test]
    fn executable_exists_in_paths_finds_matching_file() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let executable = temp_dir.path().join("codex");
        fs::write(&executable, "")?;
        make_executable(&executable)?;

        assert!(executable_exists_in_paths(
            "codex",
            vec![temp_dir.path().to_path_buf()]
        ));
        Ok(())
    }

    #[test]
    fn executable_exists_in_paths_returns_false_when_missing() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        assert!(!executable_exists_in_paths(
            "claude",
            vec![temp_dir.path().to_path_buf()]
        ));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn executable_exists_in_paths_rejects_non_executable_file() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        fs::write(temp_dir.path().join("codex"), "")?;

        assert!(!executable_exists_in_paths(
            "codex",
            vec![temp_dir.path().to_path_buf()]
        ));
        Ok(())
    }

    #[test]
    fn executable_exists_in_paths_rejects_paths() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let executable = temp_dir.path().join("codex");
        fs::write(&executable, "")?;
        make_executable(&executable)?;

        assert!(!executable_exists_in_paths(
            temp_dir.path().join("codex").to_string_lossy().as_ref(),
            vec![temp_dir.path().to_path_buf()]
        ));
        Ok(())
    }

    #[test]
    fn built_in_harnesses_returns_codex_and_claude_only() {
        let harnesses = built_in_harnesses();

        assert_eq!(harnesses.len(), 2);
        assert_eq!(harnesses[0].id, "codex");
        assert_eq!(harnesses[0].label, "Codex");
        assert_eq!(harnesses[0].executable, "codex");
        assert_eq!(harnesses[0].source, HarnessAdapterSource::BuiltInDefault);
        assert_eq!(harnesses[1].id, "claude");
        assert_eq!(harnesses[1].label, "Claude Code");
        assert_eq!(harnesses[1].executable, "claude");
        assert_eq!(harnesses[1].source, HarnessAdapterSource::BuiltInDefault);
        assert!(!harnesses.iter().any(|harness| harness.id == "hermes"));
    }

    #[test]
    fn external_adapter_dir_is_under_coven_home() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;

        assert_eq!(
            external_adapter_dir(temp_dir.path()),
            temp_dir.path().join("harness-adapters")
        );
        Ok(())
    }

    #[test]
    fn load_external_harness_specs_reads_trusted_manifest_dir() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        write_manifest(temp_dir.path(), "hermes.toml", hermes_manifest())?;

        let external = load_external_harness_specs(temp_dir.path())?;
        let hermes = external
            .iter()
            .find(|harness| harness.id == "hermes")
            .expect("hermes manifest should be loaded from external manifest dir");

        assert_eq!(hermes.label, "Hermes Agent");
        assert_eq!(hermes.executable, "hermes");
        assert_eq!(hermes.source, HarnessAdapterSource::ExternalManifest);
        assert_eq!(
            hermes.compatibility_notes.as_deref(),
            Some("One-shot argv mode only.")
        );
        Ok(())
    }

    #[test]
    fn registered_harnesses_include_external_manifests_without_changing_built_ins() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        write_manifest(temp_dir.path(), "hermes.toml", hermes_manifest())?;

        let registered = registered_harnesses(temp_dir.path())?;

        assert!(registered.iter().any(|harness| harness.id == "codex"));
        assert!(registered.iter().any(|harness| harness.id == "claude"));
        assert!(registered.iter().any(|harness| {
            harness.id == "hermes" && harness.source == HarnessAdapterSource::ExternalManifest
        }));
        assert!(!built_in_harnesses()
            .iter()
            .any(|harness| harness.id == "hermes"));
        Ok(())
    }

    #[test]
    fn external_manifest_rejects_built_in_overrides() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        write_manifest(
            temp_dir.path(),
            "codex.toml",
            r#"schema = "coven.harness-adapter.v1"
id = "codex"
label = "Fake Codex"
executable = "fake-codex"
interactive_prompt_prefix_args = []
non_interactive_prompt_prefix_args = []
"#,
        )?;

        let err = load_external_harness_specs(temp_dir.path()).unwrap_err();
        assert!(err.to_string().contains("override built-in harness"));
        Ok(())
    }

    #[test]
    fn external_manifest_rejects_executable_paths() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        write_manifest(
            temp_dir.path(),
            "bad.toml",
            r#"schema = "coven.harness-adapter.v1"
id = "bad"
label = "Bad"
executable = "/tmp/bad"
interactive_prompt_prefix_args = []
non_interactive_prompt_prefix_args = []
"#,
        )?;

        let err = load_external_harness_specs(temp_dir.path()).unwrap_err();
        assert!(err.to_string().contains("must be a PATH command name"));
        Ok(())
    }

    #[test]
    fn external_manifest_rejects_stream_claims_until_generic_stream_contract_exists() -> Result<()>
    {
        let temp_dir = tempfile::tempdir()?;
        write_manifest(
            temp_dir.path(),
            "streamy.toml",
            r#"schema = "coven.harness-adapter.v1"
id = "streamy"
label = "Streamy"
executable = "streamy"
interactive_prompt_prefix_args = []
non_interactive_prompt_prefix_args = []
supports_stream_mode = true
"#,
        )?;

        let err = load_external_harness_specs(temp_dir.path()).unwrap_err();
        assert!(err
            .to_string()
            .contains("generic external adapters only support one-shot"));
        Ok(())
    }

    #[test]
    fn built_in_harnesses_include_first_run_recovery_commands() {
        let harnesses = built_in_harnesses();
        let codex = harnesses
            .iter()
            .find(|harness| harness.id == "codex")
            .expect("codex harness should exist");
        let claude = harnesses
            .iter()
            .find(|harness| harness.id == "claude")
            .expect("claude harness should exist");

        assert!(codex.install_hint.contains("npm install -g @openai/codex"));
        assert!(codex.install_hint.contains("brew install --cask codex"));
        assert!(codex.install_hint.contains("codex"));
        assert!(codex.install_hint.contains("PATH"));

        assert!(claude
            .install_hint
            .contains("npm install -g @anthropic-ai/claude-code"));
        assert!(claude.install_hint.contains("claude doctor"));
        assert!(claude.install_hint.contains("claude"));
        assert!(claude.install_hint.contains("PATH"));
    }

    #[test]
    fn command_parts_for_known_harnesses_append_interactive_prompt() -> Result<()> {
        assert_eq!(
            command_parts_for_harness("codex", "fix tests", HarnessLaunchMode::Interactive)?,
            ("codex".to_string(), vec!["fix tests".to_string()])
        );
        assert_eq!(
            command_parts_for_harness("claude", "polish ui", HarnessLaunchMode::Interactive)?,
            ("claude".to_string(), vec!["polish ui".to_string()])
        );
        Ok(())
    }

    #[test]
    fn command_parts_for_known_harnesses_use_noninteractive_entrypoints() -> Result<()> {
        assert_eq!(
            command_parts_for_harness("codex", "fix tests", HarnessLaunchMode::NonInteractive)?,
            (
                "codex".to_string(),
                vec![
                    "exec".to_string(),
                    "--skip-git-repo-check".to_string(),
                    "--color".to_string(),
                    "never".to_string(),
                    "fix tests".to_string(),
                ]
            )
        );
        assert_eq!(
            command_parts_for_harness("claude", "polish ui", HarnessLaunchMode::NonInteractive)?,
            (
                "claude".to_string(),
                vec!["--print".to_string(), "polish ui".to_string()]
            )
        );
        Ok(())
    }

    #[test]
    fn command_spec_supports_prefix_args_for_future_harnesses() {
        let spec = HarnessCommandSpec {
            id: "future".to_string(),
            label: "Future Harness".to_string(),
            executable: "future".to_string(),
            interactive_prompt_prefix_args: vec!["chat".to_string()],
            non_interactive_prompt_prefix_args: vec!["exec".to_string(), "-q".to_string()],
            install_hint: "Install the future harness.".to_string(),
            system_prompt_flag: None,
            supports_stream_mode: false,
            supports_preassigned_session_id: false,
            source: HarnessAdapterSource::ExternalManifest,
            compatibility_notes: None,
        };

        assert_eq!(
            spec.prompt_args("hello", HarnessLaunchMode::Interactive),
            vec!["chat".to_string(), "hello".to_string()]
        );
        assert_eq!(
            spec.prompt_args("hello", HarnessLaunchMode::NonInteractive),
            vec!["exec".to_string(), "-q".to_string(), "hello".to_string()]
        );
    }

    #[test]
    fn command_parts_for_external_manifest_harness_use_manifest_args() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        write_manifest(temp_dir.path(), "hermes.toml", hermes_manifest())?;

        assert_eq!(
            command_parts_for_registered_harness(
                temp_dir.path(),
                "hermes",
                "hello; rm -rf /",
                HarnessLaunchMode::Interactive,
            )?,
            (
                "hermes".to_string(),
                vec![
                    "chat".to_string(),
                    "--source".to_string(),
                    "coven".to_string(),
                    "-q".to_string(),
                    "hello; rm -rf /".to_string(),
                ]
            )
        );
        assert_eq!(
            command_parts_for_registered_harness(
                temp_dir.path(),
                "hermes",
                "hello",
                HarnessLaunchMode::NonInteractive,
            )?,
            (
                "hermes".to_string(),
                vec![
                    "chat".to_string(),
                    "--source".to_string(),
                    "coven".to_string(),
                    "-Q".to_string(),
                    "-q".to_string(),
                    "hello".to_string(),
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn external_manifest_harnesses_do_not_inherit_stream_or_preassigned_sessions() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        write_manifest(temp_dir.path(), "hermes.toml", hermes_manifest())?;
        let hermes = registered_harness_spec(temp_dir.path(), "hermes")?
            .expect("hermes external manifest should be registered");

        assert!(!hermes.supports_stream_mode);
        assert!(!hermes.supports_preassigned_session_id);
        assert!(!harness_supports_stream_mode("hermes"));
        assert!(!harness_supports_preassigned_session_id("hermes"));
        Ok(())
    }

    #[test]
    fn command_parts_reject_unknown_harnesses() {
        assert!(command_parts_for_harness(
            "attacker-harness",
            "hello",
            HarnessLaunchMode::Interactive
        )
        .unwrap_err()
        .to_string()
        .contains("unsupported built-in harness"));
    }

    #[test]
    fn claude_init_hint_attaches_session_id_flag_in_print_mode() -> Result<()> {
        let hint = ConversationHint::Init {
            id: "abc-123".to_string(),
        };
        let parts = command_parts_for_harness_with_conversation(
            "claude",
            "hello",
            HarnessLaunchMode::NonInteractive,
            Some(&hint),
            None,
        )?;
        assert_eq!(
            parts,
            (
                "claude".to_string(),
                vec![
                    "--print".to_string(),
                    "--session-id".to_string(),
                    "abc-123".to_string(),
                    "hello".to_string(),
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn claude_resume_hint_attaches_resume_flag_in_print_mode() -> Result<()> {
        let hint = ConversationHint::Resume {
            id: "abc-123".to_string(),
        };
        let parts = command_parts_for_harness_with_conversation(
            "claude",
            "follow up",
            HarnessLaunchMode::NonInteractive,
            Some(&hint),
            None,
        )?;
        assert_eq!(
            parts,
            (
                "claude".to_string(),
                vec![
                    "--print".to_string(),
                    "--resume".to_string(),
                    "abc-123".to_string(),
                    "follow up".to_string(),
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn interactive_mode_ignores_conversation_hint() -> Result<()> {
        let hint = ConversationHint::Init {
            id: "abc-123".to_string(),
        };
        let parts = command_parts_for_harness_with_conversation(
            "claude",
            "hello",
            HarnessLaunchMode::Interactive,
            Some(&hint),
            None,
        );
        assert_eq!(
            parts.unwrap(),
            ("claude".to_string(), vec!["hello".to_string()])
        );
        Ok(())
    }

    #[test]
    fn codex_init_hint_falls_through_to_default_args_so_codex_can_assign_its_own_id() -> Result<()>
    {
        let hint = ConversationHint::Init {
            id: "abc-123".to_string(),
        };
        let parts = command_parts_for_harness_with_conversation(
            "codex",
            "fix tests",
            HarnessLaunchMode::NonInteractive,
            Some(&hint),
            None,
        )?;
        assert_eq!(
            parts,
            (
                "codex".to_string(),
                vec![
                    "exec".to_string(),
                    "--skip-git-repo-check".to_string(),
                    "--color".to_string(),
                    "never".to_string(),
                    "fix tests".to_string(),
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn codex_resume_hint_uses_exec_resume_subcommand_with_id() -> Result<()> {
        let hint = ConversationHint::Resume {
            id: "019e5998-7130-7872-8d96-a6b67c5b6406".to_string(),
        };
        let parts = command_parts_for_harness_with_conversation(
            "codex",
            "follow up",
            HarnessLaunchMode::NonInteractive,
            Some(&hint),
            None,
        )?;
        assert_eq!(
            parts,
            (
                "codex".to_string(),
                vec![
                    "exec".to_string(),
                    "--skip-git-repo-check".to_string(),
                    "--color".to_string(),
                    "never".to_string(),
                    "resume".to_string(),
                    "019e5998-7130-7872-8d96-a6b67c5b6406".to_string(),
                    "follow up".to_string(),
                ]
            )
        );
        Ok(())
    }

    #[test]
    fn preassigned_session_id_support_is_per_harness() {
        assert!(harness_supports_preassigned_session_id("claude"));
        assert!(!harness_supports_preassigned_session_id("codex"));
        assert!(!harness_supports_preassigned_session_id("unknown"));
    }

    #[test]
    fn none_hint_matches_legacy_command_parts() -> Result<()> {
        let with_none = command_parts_for_harness_with_conversation(
            "claude",
            "hello",
            HarnessLaunchMode::NonInteractive,
            None,
            None,
        )?;
        let legacy =
            command_parts_for_harness("claude", "hello", HarnessLaunchMode::NonInteractive)?;
        assert_eq!(with_none, legacy);
        Ok(())
    }

    #[cfg(unix)]
    fn make_executable(path: &Path) -> Result<()> {
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }

    #[cfg(not(unix))]
    fn make_executable(_path: &Path) -> Result<()> {
        Ok(())
    }
}
