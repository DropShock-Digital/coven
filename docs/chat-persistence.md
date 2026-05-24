# Chat Conversation Persistence

How `coven chat` keeps follow-up messages in the same conversation, and how to
extend the mechanism to additional harnesses.

## Status

| Harness | Resume support | Mechanism |
| --- | --- | --- |
| `claude` | ✅ | `claude --print --session-id <uuid>` on turn 1; `claude --print --resume <uuid>` on subsequent turns |
| `codex` | ❌ (not yet) | One-shot per turn; no conversation memory across turns |

Resume support is scoped to a single `coven chat` invocation. Exiting the chat
ends the conversation; the next invocation starts fresh. See **Future work**
below for cross-restart persistence.

## How it works

Every chat turn launches a fresh daemon session in `NonInteractive` launch
mode (`claude --print …`, `codex exec …`). To preserve conversational state
across those one-shot launches, the chat app passes a `ConversationHint` along
with each launch:

- **`Init { id }`** — first turn for this harness. The harness CLI is told to
  claim a session under this UUID.
- **`Resume { id }`** — subsequent turn. The harness CLI is told to resume
  that session and append the new prompt.

The chat app keeps a `HashMap<harness_id, conversation_id>` for the lifetime
of the `App`. On the first turn for a harness, it generates a UUID, stores it,
and sends `Init`. On every later turn it sends `Resume` with the same id.
`/clear` (and Ctrl+L) drop the map so the next turn starts a brand-new
conversation.

### Data flow

```
chat App
  └─ run_harness_prompt(harness, prompt)
       └─ conversation_hint_for_harness(harness)  → Option<ConversationHint>
            └─ LaunchRequest::with_conversation(hint)
                 └─ POST /api/v1/sessions  { ..., "conversation": {"mode": "init"|"resume", "id": "<uuid>"} }
                      └─ daemon: pty_runner::build_harness_command_with_conversation
                           └─ harness::command_parts_for_harness_with_conversation
                                └─ continuity_args(spec, mode, hint)  → ["--print","--resume","<uuid>"]
```

`continuity_args` is the per-harness translation point — it's where you wire
up a new harness's resume flags. It lives in `crates/coven-cli/src/harness.rs`.

### Why not drive the harness TUI through a PTY?

An earlier approach launched the harness in `Interactive` mode (full TUI) and
piped subsequent messages as raw stdin bytes. That works for turn 1 but turn 2
silently fails: once the harness negotiates the Kitty keyboard protocol
(`CSI > 1 u`), Enter is encoded as `\x1b[13u`, not raw `\n`, so a piped
`"<text>\n"` types the characters into the harness's input box but never
submits. The output stream is also flooded with TUI rendering (spinner frames,
status bars, ANSI repaints) that has to be filtered. Resume via the harness
CLI's own session API avoids both problems.

### What does *not* resume

- **Switching agents mid-conversation** (`/agent codex` then `/agent claude`)
  preserves each harness's own conversation independently. Switching to codex
  doesn't carry over claude's context — codex has no resume support yet (see
  below).
- **Restarts** of `coven chat`. The conversation id is in-memory only.
- **`/attach`ed sessions.** Typing while attached to a session launched by
  `coven run` (not by chat) still forwards to that session's stdin — the
  resume path only applies to sessions chat itself launched.

## Adding support for a new harness

1. **Map the harness CLI's resume flags.** Read the CLI's docs to find:
   - How to create a session with a chosen id (or accept it auto-generating
     one and capture from output).
   - How to resume a session by id in non-interactive mode.

   For claude these are `--session-id <uuid>` and `--resume <uuid>` — both
   work with `--print`. For codex they're `codex exec resume <id>` and
   `codex exec resume --last`. Other harnesses will differ.

2. **Extend `continuity_args` in `crates/coven-cli/src/harness.rs`.** Add a
   new arm to the `match spec.id` block that translates `Init` and `Resume`
   hints into the harness's actual CLI args:

   ```rust
   "codex" => {
       let cmd = match hint {
           ConversationHint::Init { .. } => return None, // codex auto-creates
           ConversationHint::Resume { id } => vec![
               "exec".to_string(), "resume".to_string(), id.clone(),
               // ...any other flags...
           ],
       };
       Some(cmd)
   }
   ```

   Note codex's wrinkle: the `Init` turn doesn't take an id — codex generates
   one. You'd need to either:
   - Capture codex's session id from the first turn's output (it prints
     `session id: <uuid>` in its header), parse it, and stash it as the
     `Resume` id for follow-up turns. This is fragile to upstream format
     changes.
   - Or use `codex exec resume --last`, which avoids parsing but breaks if
     the user runs another codex session in between (the wrong session
     becomes "last").

3. **Flip `harness_supports_chat_resume` in `crates/coven-cli/src/tui/chat/app.rs`.**
   It's a simple `harness == "claude"` check today; add your new id.

4. **Add tests** in `harness::tests` covering Init + Resume → expected args,
   matching the existing `claude_init_hint_attaches_session_id_flag_in_print_mode`
   and `claude_resume_hint_attaches_resume_flag_in_print_mode`.

5. **Add an end-to-end app test** in `tui::chat::app::tests` similar to
   `second_claude_chat_turn_reuses_init_id_as_resume`, asserting your harness
   threads the hint through `LaunchRequest`.

## Future work

### Cross-restart persistence

Right now a closing `coven chat` loses the conversation. To persist:

1. On `Init`, write the conversation id to a per-project file under
   `$COVEN_HOME/chat-conversations/<project-hash>.json` (or extend
   `chat-settings.json`).
2. On `coven chat` startup, read it back and seed
   `harness_conversation_ids` with the stored id. Next message will send
   `Resume`.
3. Add a `/new` slash command to clear stored ids and start fresh (mirroring
   what `/clear` does in-memory today).
4. Decide what to do when the stored id no longer exists on the harness side
   (claude's `--resume` will error). Either surface the error and fall back to
   `Init`, or detect the missing-session error pattern and silently regenerate.

### One ledger row per conversation

Today each chat turn shows up as a separate session in `/sessions`. That's
ledger noise. Options:

- Daemon API change: add a `conversation_id` column to the session store and
  group by it in the `/sessions` overlay.
- Chat-side aggregation: keep displaying one row per launch, but tag each
  with its conversation id and let the overlay collapse them.

### True streaming follow-ups

Each follow-up turn is a fresh process; latency includes the harness CLI's
cold start (~1-3 s for claude). For lower-latency chat, options are:

- Use `claude --input-format stream-json --output-format stream-json` to keep
  one harness process alive across turns, feeding new prompts as JSON
  messages on stdin. Avoids cold-start per turn but requires a
  daemon-side change to keep a long-lived process per chat and route
  per-turn JSON messages to it.
- A first-party Coven gateway that holds the model connection directly, with
  the harness CLI being just one of several backends.
