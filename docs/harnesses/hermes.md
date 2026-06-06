---
summary: "Experimental external adapter manifest notes for Hermes Agent."
read_when:
  - Tracking the Hermes adapter roadmap
  - Testing external harness manifests locally
title: "Hermes experimental adapter"
description: "Research and manifest-gated Hermes adapter notes for Coven. Hermes is not a default built-in adapter."
---

Hermes is **not** part of Coven's default built-in adapter set. Codex and Claude Code remain the current defaults.

This page documents an experimental, opt-in external adapter manifest for maintainers who want to validate the generic manifest path with a real Hermes install. Do not treat this as public built-in support, a default fallback, or permission to add Hermes-specific daemon/TUI/client branches.

## Status

| Field | Value |
|---|---|
| Default Coven adapter | No |
| Registration path | `<COVEN_HOME>/harness-adapters/hermes.toml` |
| Launch mode | One-shot argv prompt mode |
| Stream mode | Unsupported for generic manifests |
| Resume/preassigned upstream session ids | Unsupported for generic manifests |
| OpenClaw default mapping | No; explicit plugin config only |
| Public support stage | Experimental external manifest |

## Candidate manifest

Create this file under the trusted Coven home for local testing:

```text
~/.coven/harness-adapters/hermes.toml
```

```toml
schema = "coven.harness-adapter.v1"
id = "hermes"
label = "Hermes Agent"
executable = "hermes"
interactive_prompt_prefix_args = ["chat", "--source", "coven", "-q"]
non_interactive_prompt_prefix_args = ["chat", "--source", "coven", "-Q", "-q"]
install_hint = "Install Hermes Agent from https://hermes-agent.nousresearch.com/docs and run hermes setup or hermes doctor."
compatibility_notes = "One-shot argv mode only; not a default built-in adapter, not a stream/resume adapter, and not mapped by OpenClaw unless explicitly configured."
```

The manifest describes argv construction only. Coven appends the prompt as the final argument and does not run user-provided command strings through a shell.

## Local smoke path

After installing Hermes and adding the manifest:

```bash
coven doctor
coven run hermes --title hermes-manifest-smoke --archive "Reply with exactly HERMES_MANIFEST_OK."
```

Expected boundaries:

- `coven doctor` may list Hermes under external adapter manifests when the manifest exists and `hermes` is on `PATH`.
- `coven run hermes ...` is explicit and manifest-gated.
- Natural Cast defaults and guided default fallback still choose only Codex or Claude Code.
- `/hermes` is not a first-party Cast slash route.

## OpenClaw bridge notes

The `@opencoven/coven` bridge does not map Hermes by default. To test Hermes through OpenClaw, the operator must opt in twice:

1. register the Hermes adapter manifest in Coven; and
2. explicitly map an OpenClaw ACP agent id to `hermes` in the plugin config.

That keeps OpenClaw as an external bridge and keeps Hermes out of privileged daemon special-case logic.

## Not yet public support

Before this graduates beyond experimental manifest status, Coven needs maintainer-approved evidence for:

- command construction tests;
- real install smoke;
- attach/replay behavior expectations;
- client compatibility notes;
- docs that distinguish default adapters from external manifests; and
- a decision on whether generic external manifests should ever support stream/resume modes.

Until those are done, say "experimental external Hermes manifest," not "Hermes is a built-in Coven harness."
