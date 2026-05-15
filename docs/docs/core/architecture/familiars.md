---
title: "Familiars"
description: "Creating and managing persistent agents"
---

# Familiars

A **familiar** is a persistent agent with its own identity, memory, and capabilities. Unlike stateless agents, familiars retain knowledge and relationships over time.

## Characteristics of Familiars

- **Persistent Identity** - Same agent across sessions
- **Accumulated Memory** - Learns from all interactions
- **Autonomous Behavior** - Can act without explicit prompting
- **Relationship Building** - Develops context with users
- **Delegation Capable** - Can coordinate with other familiars

## Built-in Familiars

### Nova - Personal Assistant
Your default familiar for general tasks. Learns your preferences and workflow.

### Cody - Code Expert
Specializes in code review, refactoring, and debugging.

### Sage - Research Agent
Researches topics, synthesizes information, and cites sources.

### Echo - Communication Helper
Helps with writing, editing, and translating content.

### Charm - Relationship Manager
Manages interactions, schedules, and coordination.

### Kitty - System Administrator
Manages infrastructure, monitoring, and deployment.

## Creating a Custom Familiar

```bash
coven familiar create my-familiar \
  --template research \
  --model gpt-4-turbo
```

Configuration (`my-familiar/familiar.json`):

```json
{
  "name": "my-familiar",
  "type": "familiar",
  "model": "gpt-4-turbo",
  "personality": {
    "name": "Scholar",
    "description": "A thoughtful researcher",
    "traits": ["analytical", "curious", "rigorous"]
  },
  "memory_config": {
    "type": "persistent",
    "max_size": "2GB",
    "retention": "unlimited"
  },
  "capabilities": [
    "research",
    "synthesis",
    "writing",
    "delegation"
  ],
  "relationships": {
    "can_delegate_to": ["cody", "echo"],
    "supervised_by": ["nova"]
  }
}
```

## Memory Accumulation

Familiars build memory through:

### Interaction Memory
```json
{
  "session_id": "sess_123",
  "user": "alice",
  "messages": 42,
  "outcomes": "Successful research synthesis",
  "timestamp": "2025-01-15T10:30:00Z"
}
```

### Learning Patterns
```json
{
  "user_preferences": {
    "writing_style": "technical",
    "citation_format": "chicago",
    "detail_level": "comprehensive"
  },
  "effective_approaches": [
    "Start with broad overview",
    "Drill into specifics",
    "Provide citations"
  ]
}
```

### Relationships
```json
{
  "users": {
    "alice": { "interactions": 48, "trust": "high" },
    "bob": { "interactions": 12, "trust": "medium" }
  },
  "agents": {
    "cody": { "delegations": 15, "success_rate": 0.93 }
  }
}
```

## Querying Familiar Memory

```bash
# Search memory semantically
coven memory my-familiar --search "research on blockchain"

# View interaction history
coven familiar history my-familiar --user alice

# Export learned patterns
coven familiar export my-familiar --patterns
```

## Familiar Autonomy

Familiars can:

```bash
# Run on a schedule
coven familiar schedule my-familiar \
  --task "daily-research-digest" \
  --cron "0 9 * * MON-FRI"

# React to events
coven familiar on my-familiar \
  --trigger "file-uploaded" \
  --action "analyze-and-summarize"

# Proactive outreach
coven familiar alert my-familiar \
  --condition "important-deadline" \
  --action "notify-users"
```

## Familiar Coordination

Familiars can work together:

```bash
# Delegate research to Sage
coven ask my-familiar "Research topic X, use Sage for deep research"

# Parallel work
coven ask my-familiar "Have Cody review and Echo edit the code"

# Sequential workflow
coven ask my-familiar "Search → Sage synthesizes → Echo writes summary"
```

## Best Practices

1. **Give them personality** - Familiars with clear traits perform better
2. **Regular interaction** - More interactions = better memory and performance
3. **Clear delegation** - Define who delegates to whom
4. **Monitor relationships** - Check interaction quality and outcomes
5. **Periodic review** - Review and update familiar configurations

---

[Next: Sessions →](/core/architecture/sessions)
