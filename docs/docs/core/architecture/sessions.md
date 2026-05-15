---
title: "Sessions"
description: "Understanding and managing agent sessions"
---

# Sessions

A **session** is a conversation or workflow between a user and one or more agents. Sessions provide context, state management, and history.

## Session Anatomy

```json
{
  "id": "sess_7f4a9c2e",
  "user_id": "alice",
  "agent": "nova",
  "started": "2025-01-15T10:00:00Z",
  "status": "active",
  "message_count": 42,
  "memory_size": 156000,
  "metadata": {
    "project": "research-synthesis",
    "tags": ["urgent", "research"]
  }
}
```

## Session Lifecycle

### Creation

```bash
# Automatic (first message)
coven chat my-agent

# Explicit
coven session create --agent my-agent --name "Project Alpha"

# From template
coven session new --template research-session
```

### Active State

```bash
# View active sessions
coven session list --status active

# Attach to session
coven attach sess_7f4a9c2e

# Send messages
coven ask my-agent "Continue research on topic X" --session sess_7f4a9c2e
```

### Pause/Resume

```bash
# Pause (keeps memory, stops agent)
coven session pause sess_7f4a9c2e

# Resume (agent continues from where it left off)
coven session resume sess_7f4a9c2e
```

### Completion

```bash
# Archive (preserve history)
coven session archive sess_7f4a9c2e

# Delete (remove)
coven session delete sess_7f4a9c2e
```

## Session Context

Sessions maintain context across messages:

```
Message 1: "Research blockchain security"
  → Agent searches, reads papers, stores findings

Message 2: "What were the main risks mentioned?"
  → Agent queries session memory, finds previous research
  → Responds with specific risks from earlier research

Message 3: "Compare to cryptocurrency security"
  → Agent remembers blockchain context
  → Extends research to crypto
  → Compares frameworks from memory
```

## Multi-Agent Sessions

Sessions can involve multiple agents:

```bash
# Create collaborative session
coven session create --name "code-review-workflow"

# Delegate through session
coven ask --session code-review-workflow \
  "Have Cody review this code and Echo improve the documentation"

# Session maintains context across agents
# All agents access shared session memory
# Results integrated together
```

## Session Memory

Each session has its own memory layer:

```
Session Memory
├── Messages (full history)
├── Artifacts (code, files, outputs)
├── Metadata (tags, timestamps)
└── Relationships (linked sessions, users)
```

### Memory Access

```bash
# View session memory
coven memory --session sess_7f4a9c2e

# Search session memory
coven memory --session sess_7f4a9c2e --search "security recommendations"

# Export session memory
coven memory --session sess_7f4a9c2e --export json
```

## Session Isolation

```
Session A (Alice + Nova)
├── Messages (only Alice can see)
├── Memory (only Nova and Alice access)
└── Results (stored separately)

Session B (Bob + Nova)
├── Messages (only Bob can see)
├── Memory (only Nova and Bob access)
└── Results (stored separately)

Shared
└── Nova's long-term memory (patterns, learning)
```

## Session Branching

Create variants of sessions:

```bash
# Fork session
coven session fork sess_7f4a9c2e --name "alternative-approach"

# Continue on fork without affecting original
coven attach sess_fork_123
coven ask "Try a different approach"

# Compare results
coven diff sess_7f4a9c2e sess_fork_123
```

## Session Timeouts

Sessions automatically manage resources:

```json
{
  "timeout_config": {
    "idle_timeout": "24h",     // Pause after 24h inactivity
    "max_duration": "7d",      // Auto-archive after 7 days
    "memory_limit": "500MB",   // Stop if memory exceeds limit
    "message_limit": 10000     // Archive after 10k messages
  }
}
```

## Best Practices

1. **Name your sessions** - Makes them easier to find and reference
2. **Use tags** - Organize sessions by project or type
3. **Archive completed** - Keep active sessions lean
4. **Monitor memory** - Large sessions use more resources
5. **Fork for experiments** - Test approaches without affecting original

---

[Next: Memory →](/core/architecture/memory)
