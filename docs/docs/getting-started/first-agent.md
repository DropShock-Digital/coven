---
title: "Your First Agent"
description: "Build and customize your first Coven agent"
---

# Your First Agent

Now that you've done the quickstart, let's customize an agent with real capabilities.

## Create a Custom Agent

```bash
coven agent new my-research-agent \
  --description "Research and synthesize information" \
  --model gpt-4-turbo \
  --harness standard
```

This creates:
```
my-research-agent/
├── agent.json          # Configuration
├── README.md           # Agent documentation
└── tools.json          # Available tools
```

## Configure Your Agent

Edit `my-research-agent/agent.json`:

```json
{
  "name": "my-research-agent",
  "description": "Research and synthesize information from multiple sources",
  "harness": "standard",
  "model": "gpt-4-turbo",
  "temperature": 0.7,
  "max_tokens": 4000,
  "tools": ["web_search", "web_fetch", "file_write"],
  "memory": {
    "type": "persistent",
    "backend": "sqlite",
    "max_size": "1GB"
  },
  "permissions": {
    "can_write_files": true,
    "can_delegate": true,
    "can_read_memory": true
  },
  "personality": {
    "tone": "analytical and thorough",
    "style": "clear and concise"
  }
}
```

## Add a System Prompt

Create `my-research-agent/system.md`:

```markdown
# Research Agent System Prompt

You are a thorough research assistant. Your job is to:

1. Search for information on the given topic
2. Synthesize findings into a coherent narrative
3. Cite sources accurately
4. Highlight areas of uncertainty or disagreement

When you don't know something, say so explicitly.
When you delegate research to other agents, wait for their results
and incorporate them into your final synthesis.
```

## Test Your Agent

```bash
# Start the agent
coven start my-research-agent

# In another terminal, chat
coven chat my-research-agent

# Ask it to research something
You: Research the history of the internet and give me a 500-word summary
Agent: [Performs web search, synthesizes results, provides summary]
```

## Enable Tools

Your agent can use various tools. Check what's available:

```bash
coven tools list
```

Enable tools in `agent.json`:

```json
{
  "tools": [
    "web_search",      // Search the web
    "web_fetch",       // Fetch full page content
    "file_read",       // Read files
    "file_write",      // Write files
    "code_execute",    // Run code
    "delegate",        // Delegate to other agents
    "memory_search"    // Search agent memory
  ]
}
```

## Run Your Agent

```bash
# Long-running mode (agent stays active)
coven start my-research-agent

# One-off execution
coven ask my-research-agent "Research topic X"

# With a specific input file
coven ask my-research-agent --input research-request.txt
```

## Monitor Your Agent

```bash
# Check logs
coven logs my-research-agent --tail 50

# View memory usage
coven memory my-research-agent --stats

# Export session
coven export my-research-agent > session.json
```

## Delegation Example

Let your agent delegate to others:

```bash
# Create a data-processing agent
coven agent new data-agent \
  --description "Process and analyze data"

# In your research agent, delegate:
You: Research topic X and have the data-agent process the findings
Agent: I'll search for information and delegate processing to the data-agent...
```

## What's Next?

- [Multi-Agent Orchestration](/guides/multi-agent-orchestration) - Coordinate multiple agents
- [Custom Tools](/guides/custom-tools) - Build custom capabilities
- [Memory Management](/guides/manage-memory) - Optimize agent memory

---

[Back to Getting Started →](/getting-started/index)
