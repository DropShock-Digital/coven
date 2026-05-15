---
title: "Quickstart"
description: "Get your first agent working in 5 minutes"
---

# Quickstart

Let's create and deploy your first Coven agent in 5 minutes.

## Step 1: Start the Gateway (1 min)

```bash
coven gateway start
```

You should see:
```
✅ Gateway listening on http://localhost:8080
✅ WebSocket ready at ws://localhost:8080
```

## Step 2: Create an Agent (1 min)

In a new terminal:

```bash
coven agent new quickstart-agent
cd quickstart-agent
cat agent.json
```

You'll see your agent's configuration:

```json
{
  "name": "quickstart-agent",
  "harness": "standard",
  "model": "gpt-4-turbo",
  "memory": {
    "type": "persistent",
    "backend": "sqlite"
  }
}
```

## Step 3: Start Your Agent (1 min)

```bash
coven start
# Agent "quickstart-agent" initialized
# Connecting to gateway...
# ✅ Ready for messages
```

## Step 4: Chat with Your Agent (2 min)

In a new terminal:

```bash
coven chat quickstart-agent
```

Now you can chat:

```
You: What can you do?
Agent Nova: I can help with code review, research, writing, and more.

You: Create a function that reverses a string
Agent Nova: [Creates function and explanation]

You: /help
Agent Nova: Available commands:
  /help - Show this menu
  /clear - Clear history
  /export - Save conversation
  /session - Show session info
```

## Key Commands

```bash
# See all your agents
coven list

# Check agent status
coven status quickstart-agent

# View agent memory
coven memory quickstart-agent --search "recent tasks"

# Export session
coven export quickstart-agent

# Stop the agent
coven stop quickstart-agent
```

## What's Next?

- [Your First Agent](/getting-started/first-agent) - Deep dive into agent creation
- [Core Architecture](/core/architecture/overview) - Understand how Coven works
- [Delegation Guide](/guides/delegate-work) - Have agents work together

---

**Stuck?** Check [Troubleshooting](/resources/troubleshooting/common-issues)
