use std::io::{self, BufRead, BufReader, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::thread;

use anyhow::{Context, Result};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use portable_pty::{native_pty_system, ChildKiller, CommandBuilder, PtySize, PtySystem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessCommand {
    program: String,
    args: Vec<String>,
    cwd: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtyRunResult {
    pub status: &'static str,
    pub exit_code: Option<i32>,
}

pub struct DetachedPtySession {
    pub input: Box<dyn Write + Send>,
    pub killer: Box<dyn ChildKiller + Send + Sync>,
}

pub struct DetachedPtyObserver {
    pub on_output: Box<dyn FnMut(Vec<u8>) + Send + 'static>,
    pub on_exit: Box<dyn FnOnce(PtyRunResult) + Send + 'static>,
}

impl HarnessCommand {
    pub fn program(&self) -> &str {
        &self.program
    }

    #[cfg(test)]
    pub fn args(&self) -> &[String] {
        &self.args
    }

    #[cfg(test)]
    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    fn to_command_builder(&self) -> CommandBuilder {
        let mut builder = CommandBuilder::new(&self.program);
        builder.args(&self.args);
        builder.cwd(self.cwd.as_os_str());
        builder
    }
}

pub fn build_harness_command(
    harness_id: &str,
    prompt: &str,
    cwd: &Path,
    mode: crate::harness::HarnessLaunchMode,
) -> Result<HarnessCommand> {
    build_harness_command_with_conversation(harness_id, prompt, cwd, mode, None)
}

pub fn build_harness_command_with_conversation(
    harness_id: &str,
    prompt: &str,
    cwd: &Path,
    mode: crate::harness::HarnessLaunchMode,
    conversation: Option<&crate::harness::ConversationHint>,
) -> Result<HarnessCommand> {
    let (program, args) = crate::harness::command_parts_for_harness_with_conversation(
        harness_id,
        prompt,
        mode,
        conversation,
    )?;

    Ok(HarnessCommand {
        program: program.to_string(),
        args,
        cwd: cwd.to_path_buf(),
    })
}

pub fn run_attached(command: &HarnessCommand) -> Result<PtyRunResult> {
    let pty_system = native_pty_system();
    run_attached_with_pty_system(command, pty_system.as_ref())
}

#[allow(dead_code)]
pub fn spawn_detached(command: &HarnessCommand) -> Result<DetachedPtySession> {
    spawn_detached_with_observer(command, None)
}

/// Handle returned by `spawn_piped_with_observer`. The child is wrapped in
/// an `Arc<Mutex<Option<...>>>` so the drain thread can take it for `wait`
/// while the daemon's killer holds a separate clone for `kill`.
pub struct PipedSession {
    pub input: Box<dyn Write + Send>,
    pub child: std::sync::Arc<std::sync::Mutex<Option<std::process::Child>>>,
}

/// Spawn `command` as a plain piped child process (no PTY) and stream its
/// stdout to `observer`. Used by stream-mode harness launches where the
/// child reads newline-delimited JSON from stdin and writes
/// newline-delimited JSON to stdout — wrapping in a PTY would add ANSI
/// escapes the child wouldn't otherwise emit. Lifecycle mirrors
/// `spawn_detached_with_observer`: a background thread drains stdout and
/// fires `on_exit` when the child finishes.
pub fn spawn_piped_with_observer(
    command: &HarnessCommand,
    observer: Option<DetachedPtyObserver>,
) -> Result<PipedSession> {
    use std::process::Command as StdCommand;
    use std::sync::{Arc, Mutex as StdMutex};

    let mut std_command = StdCommand::new(&command.program);
    std_command.args(&command.args);
    std_command.current_dir(&command.cwd);
    std_command.stdin(Stdio::piped());
    std_command.stdout(Stdio::piped());
    std_command.stderr(Stdio::piped());

    let mut child = std_command.spawn().with_context(|| {
        format!(
            "failed to spawn harness `{}` in piped mode",
            command.program
        )
    })?;

    let stdin = child
        .stdin
        .take()
        .context("failed to take child stdin in piped mode")?;
    let stdout = child
        .stdout
        .take()
        .context("failed to take child stdout in piped mode")?;
    let stderr = child
        .stderr
        .take()
        .context("failed to take child stderr in piped mode")?;

    // Drain stderr to keep the buffer from filling and the child from
    // blocking. Stream-json harnesses surface auth/setup errors there.
    // `map_while(Result::ok)` stops at the first read error instead of
    // looping forever on EBADF (clippy::lines_filter_map_ok).
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            eprintln!("[stream-stderr] {line}");
        }
    });

    let shared_child = Arc::new(StdMutex::new(Some(child)));
    let shared_for_wait = Arc::clone(&shared_child);
    thread::spawn(move || {
        let mut reader = stdout;
        let mut observer = observer;
        drain_detached_output(
            &mut reader,
            observer.as_mut().map(|observer| &mut observer.on_output),
        );
        let result = wait_for_piped_child(&shared_for_wait);
        if let Some(observer) = observer {
            (observer.on_exit)(result);
        }
    });

    Ok(PipedSession {
        input: Box::new(stdin),
        child: shared_child,
    })
}

fn wait_for_piped_child(
    child: &std::sync::Arc<std::sync::Mutex<Option<std::process::Child>>>,
) -> PtyRunResult {
    let mut guard = match child.lock() {
        Ok(guard) => guard,
        Err(_) => {
            return PtyRunResult {
                status: "failed",
                exit_code: None,
            };
        }
    };
    let Some(child) = guard.as_mut() else {
        return PtyRunResult {
            status: "completed",
            exit_code: None,
        };
    };
    match child.wait() {
        Ok(status) => {
            let exit_code = status.code();
            let status_label = if status.success() {
                "completed"
            } else {
                "failed"
            };
            PtyRunResult {
                status: status_label,
                exit_code,
            }
        }
        Err(_) => PtyRunResult {
            status: "failed",
            exit_code: None,
        },
    }
}

pub fn spawn_detached_with_observer(
    command: &HarnessCommand,
    observer: Option<DetachedPtyObserver>,
) -> Result<DetachedPtySession> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(terminal_size())
        .context("failed to open PTY")?;
    let mut child = pair
        .slave
        .spawn_command(command.to_command_builder())
        .with_context(|| format!("failed to spawn harness `{}`", command.program()))?;
    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .context("failed to clone PTY reader")?;
    let input = pair
        .master
        .take_writer()
        .context("failed to open PTY writer")?;
    let killer = child.clone_killer();

    thread::spawn(move || {
        let mut observer = observer;
        drain_detached_output(
            &mut reader,
            observer.as_mut().map(|observer| &mut observer.on_output),
        );
        let result = wait_for_child(&mut child);
        if let Some(observer) = observer {
            (observer.on_exit)(result);
        }
    });

    Ok(DetachedPtySession { input, killer })
}

fn drain_detached_output(
    reader: &mut dyn Read,
    mut on_output: Option<&mut Box<dyn FnMut(Vec<u8>) + Send + 'static>>,
) {
    let mut buffer = [0_u8; 8192];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                if let Some(callback) = on_output.as_deref_mut() {
                    callback(buffer[..bytes_read].to_vec());
                }
            }
            Err(_) => break,
        }
    }
}

fn wait_for_child(child: &mut Box<dyn portable_pty::Child + Send + Sync>) -> PtyRunResult {
    match child.wait() {
        Ok(exit_status) => {
            let exit_code = i32::try_from(exit_status.exit_code()).unwrap_or(i32::MAX);
            let status = if exit_status.success() {
                "completed"
            } else {
                "failed"
            };
            PtyRunResult {
                status,
                exit_code: Some(exit_code),
            }
        }
        Err(_) => PtyRunResult {
            status: "failed",
            exit_code: None,
        },
    }
}

fn run_attached_with_pty_system(
    command: &HarnessCommand,
    pty_system: &(dyn PtySystem + Send),
) -> Result<PtyRunResult> {
    let pair = pty_system
        .openpty(terminal_size())
        .context("failed to open PTY")?;
    let mut child = pair
        .slave
        .spawn_command(command.to_command_builder())
        .with_context(|| format!("failed to spawn harness `{}`", command.program()))?;

    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .context("failed to clone PTY reader")?;
    let mut writer = pair
        .master
        .take_writer()
        .context("failed to open PTY writer")?;
    let _raw_mode =
        RawModeGuard::enable_if_terminal().context("failed to enable raw terminal mode")?;

    let output_thread = thread::spawn(move || {
        let mut stdout = io::stdout().lock();
        io::copy(&mut reader, &mut stdout)?;
        stdout.flush()
    });

    thread::spawn(move || {
        let mut stdin = io::stdin().lock();
        let _ = io::copy(&mut stdin, &mut writer);
    });

    let exit_status = child.wait().context("failed to wait for harness process")?;
    let _ = output_thread.join();
    let exit_code = i32::try_from(exit_status.exit_code()).unwrap_or(i32::MAX);
    let status = if exit_status.success() {
        "completed"
    } else {
        "failed"
    };

    Ok(PtyRunResult {
        status,
        exit_code: Some(exit_code),
    })
}

struct RawModeGuard {
    enabled: bool,
}

impl RawModeGuard {
    fn enable_if_terminal() -> Result<Self> {
        let enabled = io::stdin().is_terminal() && io::stdout().is_terminal();
        if enabled {
            enable_raw_mode()?;
        }
        Ok(Self { enabled })
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        if self.enabled {
            let _ = disable_raw_mode();
        }
    }
}

fn terminal_size() -> PtySize {
    PtySize {
        rows: env_u16("LINES").unwrap_or(24),
        cols: env_u16("COLUMNS").unwrap_or(80),
        pixel_width: 0,
        pixel_height: 0,
    }
}

fn env_u16(name: &str) -> Option<u16> {
    std::env::var(name)
        .ok()?
        .parse()
        .ok()
        .filter(|value| *value > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_codex_command_without_shell_interpolation() {
        let cwd = Path::new("/tmp/coven project");
        let command = build_harness_command(
            "codex",
            "hello; rm -rf /",
            cwd,
            crate::harness::HarnessLaunchMode::Interactive,
        )
        .unwrap();

        assert_eq!(command.program(), "codex");
        assert_eq!(command.args(), &["hello; rm -rf /"]);
        assert_eq!(command.cwd(), cwd);
    }

    #[test]
    fn spawn_detached_starts_pty_and_returns_input_and_kill_handles() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let command = HarnessCommand {
            program: "cat".to_string(),
            args: vec![],
            cwd: temp_dir.path().to_path_buf(),
        };

        let mut session = spawn_detached(&command)?;
        session.input.write_all(b"hello detached pty\n")?;
        session.input.flush()?;
        session.killer.kill()?;
        Ok(())
    }

    #[test]
    fn detached_output_drain_invokes_callback_for_bytes() {
        let captured = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let captured_for_callback = captured.clone();
        let mut callback: Box<dyn FnMut(Vec<u8>) + Send + 'static> = Box::new(move |chunk| {
            captured_for_callback
                .lock()
                .unwrap()
                .extend_from_slice(&chunk);
        });
        let mut reader: &[u8] = b"hello coven";

        drain_detached_output(&mut reader, Some(&mut callback));

        assert_eq!(captured.lock().unwrap().as_slice(), b"hello coven");
    }

    #[test]
    fn builds_claude_command_without_shell_interpolation() {
        let cwd = Path::new("/tmp/coven-project");
        let command = build_harness_command(
            "claude",
            "explain && exit",
            cwd,
            crate::harness::HarnessLaunchMode::Interactive,
        )
        .unwrap();

        assert_eq!(command.program(), "claude");
        assert_eq!(command.args(), &["explain && exit"]);
        assert_eq!(command.cwd(), cwd);
    }
}
