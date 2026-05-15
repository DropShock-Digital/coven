---
title: "Memory System"
description: "Understanding Coven's persistent memory architecture"
---

# Memory System

Coven's memory system enables agents to learn, adapt, and maintain context over time.

## Memory Layers

### Session Memory (Short-term)

```
Current Conversation
├── Messages (last 50-200)
├── Current task context
├── Working memory
└── Scratch space for reasoning
```

**Duration:** Active session  
**Size:** ~1-10 MB  
**Access:** Current agent only

### Agent Memory (Mid-term)

```
Agent Knowledge
├── Interaction patterns (last 100 sessions)
├── User preferences learned
├── Effective strategies
├── Relationship context
└── Project state
```

**Duration:** Agent lifetime  
**Size:** ~100 MB - 1 GB  
**Access:** Agent + supervisors

### Semantic Memory (Long-term)

```
Knowledge Base
├── Facts and findings
├── Analysis and insights
├── Lessons learned
├── Best practices
└── Domain knowledge
```

**Duration:** Indefinite  
**Size:** Unlimited (stored in database)  
**Access:** Any agent with permissions

## Memory Organization

### Hierarchical Structure

```
Root
├── Projects
│   ├── project-alpha
│   │   ├── findings
│   │   ├── decisions
│   │   └── learnings
│   └── project-beta
├── Agents
│   ├── nova
│   │   ├── preferences
│   │   └── patterns
│   └── cody
└── Users
    ├── alice
    └── bob
```

### Metadata Tagging

```json
{
  "id": "mem_abc123",
  "content": "Best practice: Always validate input before processing",
  "type": "lesson",
  "tags": ["validation", "security", "best-practice"],
  "confidence": 0.95,
  "created": "2025-01-10T14:30:00Z",
  "last_accessed": "2025-01-14T09:15:00Z",
  "access_count": 23
}
```

## Semantic Search

Find memories by meaning, not just keywords:

```bash
# Semantic search
coven memory search "How do I handle database errors?"

# Vector-based matching finds related memories:
# - "Database error handling patterns"
# - "Exception management in Node.js"
# - "Error logging best practices"
```

### Implementation

```
Natural Language Query
        ↓
   Embeddings (vector)
        ↓
Vector Database Search
        ↓
   Ranked Results
```

## Memory Insertion

Automatic memory creation:

```
Agent Decision/Action
        ↓
        ├→ Task completed successfully?
        │  └→ Store as "lesson learned"
        │
        ├→ New insight discovered?
        │  └→ Store as "finding"
        │
        ├→ User preference detected?
        │  └→ Store as "preference"
        │
        └→ Effective pattern?
           └→ Store as "pattern"
```

## Memory Querying

Multiple query patterns:

```bash
# Direct lookup
coven memory get mem_abc123

# Semantic search
coven memory search "blockchain security"

# Time-based query
coven memory list --since "1 week ago"

# Tag-based query
coven memory list --tag "lesson-learned"

# Filter by confidence
coven memory list --min-confidence 0.8

# Combined query
coven memory search "database" --tag "performance" --since "1 month ago"
```

## Forgetting & Pruning

Manage memory size:

```bash
# Mark as low-confidence
coven memory update mem_123 --confidence 0.3

# Archive old memories
coven memory archive --before "1 year ago"

# Prune by access pattern (unused memories)
coven memory prune --unused-for "6 months"

# Consolidate redundant memories
coven memory consolidate --similar --threshold 0.9
```

## Privacy & Permissions

```json
{
  "id": "mem_abc123",
  "content": "...",
  "access_list": {
    "agents": ["nova", "cody"],
    "users": ["alice"],
    "teams": ["engineering"]
  },
  "visibility": "private"
}
```

## Memory Lifecycle Example

```
1. Session starts
   Agent processes query
   ↓

2. Discovery phase
   Agent learns something
   → Store with confidence 0.6
   ↓

3. Validation phase
   Agent tests approach multiple times
   → Update confidence to 0.9
   ↓

4. Integration phase
   Memory used in multiple contexts
   → Cross-reference and link
   ↓

5. Stability phase
   Core knowledge, rarely changes
   → High confidence (0.95+)
   → Widely accessible
```

## Best Practices

1. **Quality over quantity** - Valuable memories beat storage limits
2. **Tag consistently** - Improves discoverability
3. **Update confidence** - Refine as you learn more
4. **Link related memories** - Create knowledge networks
5. **Regular review** - Prune outdated information
6. **Respect privacy** - Set appropriate access controls

---

[Next: Gateway API →](/core/api/gateway)
