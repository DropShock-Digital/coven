---
title: "Built-in Agents"
description: "Overview of Coven's built-in agents"
---

# Built-in Agents

Coven ships with several built-in agents (familiars) ready to use out of the box.

## Available Agents

| Agent | Role | Best For |
|-------|------|----------|
| **Nova** | Personal Assistant | General tasks, coordination |
| **Cody** | Code Expert | Development, debugging, review |
| **Sage** | Researcher | Research, synthesis, analysis |
| **Echo** | Writer | Writing, editing, communication |
| **Charm** | Coordinator | Scheduling, relationships, meetings |
| **Kitty** | Administrator | Infrastructure, operations, deployment |

## Quick Reference

- [Nova](/core/agents/nova) - Your all-purpose familiar
- [Sage](/core/agents/sage) - Deep research capabilities
- [Echo](/core/agents/echo) - Communication excellence
- [Charm](/core/agents/charm) - Coordination and relationships
- [Cody](/core/agents/cody) - Code expertise
- [Kitty](/core/agents/kitty) - System administration

## Customization

Each agent can be customized:

```bash
# Clone and customize
coven familiar clone nova my-nova
coven familiar edit my-nova --personality "more casual"

# Adjust capabilities
coven familiar config my-nova --add-tool "aws-cli"
```

---

[Next: Nova →](/core/agents/nova)
