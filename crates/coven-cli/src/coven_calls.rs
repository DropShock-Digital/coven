// coven_calls.rs — daemon-side writer for the Coven Calls delegation log.
//
// Every time one familiar delegates work to another (via `sessions_spawn`,
// `sessions_send`, or the `/delegate` cast code), the daemon appends a
// record here so the Coven Calls graph in coven-cave has data to render.
//
// Data contract (matches coven-cave `src/lib/coven-calls-types.ts`):
//   {
//     "version": 1,
//     "calls": [
//       {
//         "id": "<uuid>",
//         "callerFamiliarId": "nova",
//         "calleeFamiliarId": "sage",
//         "request": "<first 300 chars of prompt>",
//         "status": "running" | "completed" | "failed" | "cancelled",
//         "createdAt": "<ISO 8601>",
//         "endedAt": "<ISO 8601>",     // omitted until terminal
//         "sessionId": "<uuid>"        // omitted when unknown
//       }
//     ]
//   }
//
// Writes are atomic (write-to-tmp → rename) so a crash never leaves a
// corrupt JSON file on disk.
//
// Author: Kitty 🐾  · 2026-06-05

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Public types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CovenCallStatus {
    Completed,
    Failed,
    Cancelled,
}

impl CovenCallStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CovenCall {
    pub id: String,
    pub caller_familiar_id: String,
    pub callee_familiar_id: String,
    /// Truncated to 300 chars to keep the file small.
    pub request: String,
    pub status: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
}

// ── Internal file schema ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CallsFile {
    version: u32,
    calls: Vec<CovenCall>,
}

impl Default for CallsFile {
    fn default() -> Self {
        Self { version: 1, calls: Vec::new() }
    }
}

// ── Path ─────────────────────────────────────────────────────────────────────

fn calls_file_path(coven_home: &Path) -> PathBuf {
    coven_home.join("cave-coven-calls.json")
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Load all call records. Returns an empty vec when the file doesn't exist yet.
pub fn load_calls(coven_home: &Path) -> Result<Vec<CovenCall>> {
    let file = load_calls_file(&calls_file_path(coven_home))?;
    Ok(file.calls)
}

/// Append a new `"running"` delegation record.  
/// Returns the new call's `id` (used to close the record via `emit_terminal`).
///
/// Called from `launch_session` in `api.rs` when both `familiar_id` and
/// `caller_familiar_id` are set on the launch payload.
pub fn record_call(
    coven_home: &Path,
    caller_id: &str,
    callee_id: &str,
    request: &str,
    session_id: &str,
) -> Result<CovenCall> {
    let path = calls_file_path(coven_home);
    let mut file = load_calls_file(&path)?;

    let call = CovenCall {
        id: Uuid::new_v4().to_string(),
        caller_familiar_id: caller_id.to_string(),
        callee_familiar_id: callee_id.to_string(),
        request: truncate_request(request, 300),
        status: "running".to_string(),
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        ended_at: None,
        session_id: Some(session_id.to_string()),
        artifact: None,
    };

    file.calls.push(call.clone());
    save_calls_file(&path, &file)?;

    Ok(call)
}

/// Emit a `"running"` record for an inline cast delegation (where the callee
/// is resolved by the cast runner rather than the launch path).  
/// Returns the new call id string for later closure via `emit_terminal`.
pub fn emit_running(
    coven_home: &Path,
    caller_id: &str,
    callee_id: &str,
    request: &str,
    session_id: Option<&str>,
) -> Result<String> {
    let path = calls_file_path(coven_home);
    let mut file = load_calls_file(&path)?;

    let call = CovenCall {
        id: Uuid::new_v4().to_string(),
        caller_familiar_id: caller_id.to_string(),
        callee_familiar_id: callee_id.to_string(),
        request: truncate_request(request, 300),
        status: "running".to_string(),
        created_at: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        ended_at: None,
        session_id: session_id.map(str::to_string),
        artifact: None,
    };

    let call_id = call.id.clone();
    file.calls.push(call);
    save_calls_file(&path, &file)?;

    Ok(call_id)
}

/// Update an existing call record to a terminal status (`completed`, `failed`,
/// or `cancelled`). No-ops gracefully when the `call_id` is not found (e.g.
/// if the file was rotated or the call was never recorded).
pub fn emit_terminal(
    coven_home: &Path,
    call_id: &str,
    status: CovenCallStatus,
    ended_at: &str,
    artifact: Option<&str>,
) -> Result<()> {
    let path = calls_file_path(coven_home);
    let mut file = load_calls_file(&path)?;

    if let Some(call) = file.calls.iter_mut().find(|c| c.id == call_id) {
        call.status = status.as_str().to_string();
        call.ended_at = Some(ended_at.to_string());
        if let Some(a) = artifact {
            call.artifact = Some(a.to_string());
        }
        save_calls_file(&path, &file)?;
    }
    // Silently no-op if call_id not found — non-fatal.
    Ok(())
}

// ── Private helpers ───────────────────────────────────────────────────────────

fn load_calls_file(path: &Path) -> Result<CallsFile> {
    match fs::read_to_string(path) {
        Ok(raw) => serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse {}", path.display())),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(CallsFile::default()),
        Err(err) => Err(err).with_context(|| format!("failed to read {}", path.display())),
    }
}

fn save_calls_file(path: &Path, file: &CallsFile) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create dir {}", parent.display()))?;
    }
    let tmp = path.with_extension(format!("{}.tmp", std::process::id()));
    let json = serde_json::to_string_pretty(file).context("failed to serialize calls file")?;
    fs::write(&tmp, &json)
        .with_context(|| format!("failed to write tmp {}", tmp.display()))?;
    fs::rename(&tmp, path)
        .with_context(|| format!("failed to rename {} → {}", tmp.display(), path.display()))?;
    Ok(())
}

fn truncate_request(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{truncated}…")
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_call_creates_file_on_first_write() {
        let tmp = tempfile::tempdir().unwrap();
        let call = record_call(tmp.path(), "nova", "sage", "Research this topic", "sess-1").unwrap();
        assert_eq!(call.caller_familiar_id, "nova");
        assert_eq!(call.callee_familiar_id, "sage");
        assert_eq!(call.status, "running");
        assert_eq!(call.session_id.as_deref(), Some("sess-1"));

        let calls = load_calls(tmp.path()).unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].id, call.id);
    }

    #[test]
    fn record_call_appends_to_existing_file() {
        let tmp = tempfile::tempdir().unwrap();
        record_call(tmp.path(), "nova", "sage", "Task A", "sess-1").unwrap();
        record_call(tmp.path(), "nova", "cody", "Task B", "sess-2").unwrap();
        record_call(tmp.path(), "charm", "sage", "Task C", "sess-3").unwrap();

        let calls = load_calls(tmp.path()).unwrap();
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[2].caller_familiar_id, "charm");
    }

    #[test]
    fn emit_running_returns_call_id() {
        let tmp = tempfile::tempdir().unwrap();
        let call_id = emit_running(tmp.path(), "nova", "unknown", "Do a thing", Some("sess-1")).unwrap();
        assert!(!call_id.is_empty());

        let calls = load_calls(tmp.path()).unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].id, call_id);
        assert_eq!(calls[0].status, "running");
    }

    #[test]
    fn emit_terminal_updates_status() {
        let tmp = tempfile::tempdir().unwrap();
        let call_id = emit_running(tmp.path(), "nova", "sage", "Task", None).unwrap();
        emit_terminal(
            tmp.path(),
            &call_id,
            CovenCallStatus::Completed,
            "2026-06-05T22:00:00.000Z",
            Some("artifact-text"),
        ).unwrap();

        let calls = load_calls(tmp.path()).unwrap();
        assert_eq!(calls[0].status, "completed");
        assert_eq!(calls[0].ended_at.as_deref(), Some("2026-06-05T22:00:00.000Z"));
        assert_eq!(calls[0].artifact.as_deref(), Some("artifact-text"));
    }

    #[test]
    fn emit_terminal_noop_when_call_id_missing() {
        let tmp = tempfile::tempdir().unwrap();
        // Should not error even when the file doesn't exist yet.
        let result = emit_terminal(
            tmp.path(),
            "nonexistent-id",
            CovenCallStatus::Failed,
            "2026-06-05T22:00:00.000Z",
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn request_is_truncated_to_300_chars() {
        let tmp = tempfile::tempdir().unwrap();
        let long_request = "x".repeat(500);
        let call = record_call(tmp.path(), "nova", "sage", &long_request, "sess-1").unwrap();
        assert!(call.request.chars().count() <= 302);
        assert!(call.request.ends_with('…'));
    }

    #[test]
    fn short_request_is_not_truncated() {
        let tmp = tempfile::tempdir().unwrap();
        let call = record_call(tmp.path(), "nova", "sage", "Short task", "sess-1").unwrap();
        assert_eq!(call.request, "Short task");
    }

    #[test]
    fn load_calls_returns_empty_when_file_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let calls = load_calls(tmp.path()).unwrap();
        assert!(calls.is_empty());
    }

    #[test]
    fn call_status_variants_serialize_correctly() {
        assert_eq!(CovenCallStatus::Completed.as_str(), "completed");
        assert_eq!(CovenCallStatus::Failed.as_str(), "failed");
        assert_eq!(CovenCallStatus::Cancelled.as_str(), "cancelled");
    }
}
