---
title: "Key Concepts"
description: "Learn the fundamental concepts of Coven"
---

# Key Concepts

## Harness

A **harness** is the runtime environment where agents execute. It provides:
- Task execution
- Tool access
- Session management
- Memory storage

Think of it as the container that brings an agent to life.

## Familiar

A **familiar** is a persistent agent with its own identity, memory, and capabilities. Familiars are designed to:
- Maintain state across sessions
- Learn from interactions
- Develop relationships with users
- Act autonomously when needed

Examples: Nova (your personal assistant), Cody (code reviewer), Sage (research agent).

## Session

A **session** is a conversation or workflow between a user and one or more familiars. Sessions:
- Have unique IDs for reference
- Contain message history
- Can be paused and resumed
- Support multi-agent collaboration

## Memory

**Memory** is how agents store and retrieve knowledge:
- **Short-term** - Current session context
- **Long-term** - Persistent knowledge across sessions
- **Semantic** - Searchable by meaning, not just keywords
- **Hierarchical** - Organized by importance and recency

## Delegation

When one agent delegates work to another:
1. The delegating agent creates a sub-task
2. The receiving agent takes ownership
3. Results are returned to the original agent
4. Both agents maintain memory of the collaboration

## Tools

**Tools** are capabilities agents can use:
- Code execution
- File operations
- Web search
- API calls
- Message sending
- And more...

---

[Next: Installation →](/getting-started/install)
