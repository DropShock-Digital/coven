---
title: "Harnesses"
description: "Understanding agent harnesses and runtimes"
---

# Harnesses

A **harness** is the runtime environment where agents execute and interact with tools.

## Types of Harnesses

### Standard Harness

The default harness for most use cases:
- Single-threaded agent execution
- Full tool access
- Persistent state
- Suitable for autonomous agents

**Use when:** You need a general-purpose agent

### Sandbox Harness

Restricted execution environment:
- Limited tool access
- Resource constraints
- Time limits on execution
- Safe for untrusted code

**Use when:** Running user-provided agent code

### Stream Harness

Designed for streaming/reactive workflows:
- Event-driven execution
- Real-time responses
- Lower latency
- Ideal for chat interfaces

**Use when:** Building responsive chat agents

### Batch Harness

For processing large workloads:
- Process queued tasks
- Optimized for throughput
- Result batching
- Cost-effective

**Use when:** Running scheduled tasks or bulk operations

## Harness Configuration

Each harness has its own `harness.json`:

```json
{
  "type": "standard",
  "name": "primary-harness",
  "concurrency": 4,
  "timeout_seconds": 300,
  "memory_limit_mb": 1024,
  "tools": ["web_search", "file_read", "code_execute"],
  "resource_limits": {
    "cpu_cores": 2,
    "memory_mb": 1024,
    "disk_mb": 5000
  },
  "logging": {
    "level": "info",
    "output": "stdout"
  }
}
```

## Tool Execution in Harnesses

### Standard Flow

```
1. Agent requests tool
2. Harness validates permission
3. Tool executes with timeout
4. Result returned to agent
5. Result stored in memory
```

### Resource Limits

```
Tool Execution
    ↓
Time Limit? (300s) → Timeout
    ↓
Memory Limit? (1GB) → Out of Memory
    ↓
CPU Limit? (2 cores) → Throttled
    ↓
Success → Result to Agent
```

## Multiple Harnesses

Run multiple harnesses for:

**High Availability**
```
Gateway
    ├── Harness 1 (4 agents)
    ├── Harness 2 (4 agents)
    └── Harness 3 (4 agents)
```

**Specialized Workloads**
```
Gateway
    ├── Stream Harness (chat agents)
    ├── Batch Harness (background tasks)
    └── Sandbox Harness (user code)
```

## Monitoring

Check harness health:

```bash
coven harness status
coven harness logs --name primary-harness --tail 100
coven harness metrics --name primary-harness
```

---

[Next: Familiars →](/core/architecture/familiars)
