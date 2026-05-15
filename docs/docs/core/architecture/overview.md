---
title: "Architecture Overview"
description: "Understand Coven's system architecture"
---

# Architecture Overview

Coven is built on a modular architecture that separates concerns and enables horizontal scaling.

## System Components

```
┌─────────────────────────────────────────────────────┐
│                   User Interface Layer              │
│  (Chat CLI, Web Dashboard, IDE Extensions, API)    │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────v────────────────────────────────┐
│                 Gateway Service                      │
│  (Session Management, Routing, Authentication)     │
└─────────┬──────────────────────────────────┬────────┘
          │                                  │
    ┌─────v──────┐                  ┌──────v──────┐
    │  Harness   │                  │  Harness   │
    │ (Runtime)  │                  │ (Runtime)  │
    │ ┌────────┐ │                  │ ┌────────┐ │
    │ │Agent 1 │ │                  │ │Agent 2 │ │
    │ │Agent 2 │ │                  │ │Agent 3 │ │
    │ └────────┘ │                  │ └────────┘ │
    └─────┬──────┘                  └──────┬─────┘
          │                                 │
    ┌─────v─────────────────────────────────v─┐
    │     Memory & Knowledge Store             │
    │  (Persistent Storage, Semantic Index)   │
    └────────────────────────────────────────┘
```

## Core Layers

### Gateway

The central hub that:
- Routes messages to/from agents
- Manages sessions
- Handles authentication
- Coordinates multi-agent workflows

### Harness

Runtime environments that:
- Execute agent code
- Manage tool execution
- Isolate agents from each other
- Scale independently

### Agents (Familiars)

Persistent entities that:
- Maintain identity and memory
- Process messages
- Make decisions
- Delegate work

### Memory Store

Long-term storage that:
- Persists agent knowledge
- Enables semantic search
- Supports hierarchical organization
- Tracks learning over time

## Data Flow

### Simple Message

```
User → Gateway → Harness → Agent → Memory → Response → User
```

### Delegation

```
User → Agent A → Delegates to → Agent B → Returns result → Agent A → Response → User
        ↓                                      ↓
      Memory                                Memory
```

## Scalability

### Horizontal Scaling

- Multiple gateways behind load balancer
- Multiple harnesses on different machines
- Shared memory backend (PostgreSQL, MongoDB, etc.)
- Each agent can have dedicated resources

### Vertical Scaling

- Increase harness concurrency
- Optimize memory storage
- Cache frequently accessed knowledge

## Security Boundaries

```
┌─────────────────────────────────────┐
│         User Sessions               │
│  ┌──────────────┐  ┌──────────────┐ │
│  │ Agent A      │  │ Agent B      │ │
│  │ (Isolated)   │  │ (Isolated)   │ │
│  └──────────────┘  └──────────────┘ │
└────────────┬────────────────────────┘
             │ (Controlled via Memory ACLs)
        ┌────v─────────┐
        │ Shared Memory │
        └──────────────┘
```

Each agent:
- Runs in isolated process/container
- Can only access authorized memory
- Has resource limits
- Logs all actions

---

[Next: Harnesses →](/core/architecture/harnesses)
