---
title: "Installation"
description: "Install Coven on your machine"
---

# Installation

## System Requirements

- **Node.js** 18+ or **Python** 3.9+
- **4GB RAM** minimum (8GB recommended)
- **macOS**, **Linux**, or **Windows** (WSL2)

## Quick Install

### macOS (Homebrew)

```bash
brew tap opencoven/coven
brew install coven
coven --version
```

### Linux / macOS (npm)

```bash
npm install -g @opencoven/cli
coven --version
```

### Windows (PowerShell)

```powershell
npm install -g @opencoven/cli
coven --version
```

## Verify Installation

```bash
coven status
# Should show: ✅ Coven is ready
```

## First Run

### 1. Start the Gateway

The gateway is Coven's central service:

```bash
coven gateway start
# Gateway running on localhost:8080
```

### 2. Create an Agent

```bash
coven agent new my-agent
# Created agent: my-agent
```

### 3. Chat with Your Agent

```bash
coven chat my-agent
# Connected to Nova (default harness)
# Type your message...
```

## Configuration

Coven stores config in `~/.coven/`:

```
~/.coven/
├── config.json      # Main configuration
├── agents/          # Agent definitions
├── memory/          # Agent memory storage
└── logs/            # System logs
```

Edit `~/.coven/config.json` to customize settings:

```json
{
  "gateway": {
    "port": 8080,
    "host": "localhost"
  },
  "agents": {
    "default_harness": "standard",
    "memory_backend": "sqlite"
  }
}
```

## Troubleshooting

**Port already in use?**
```bash
coven gateway start --port 8081
```

**Permissions error?**
```bash
sudo chown -R $(whoami) ~/.coven
```

**Still having issues?** See [Troubleshooting](/resources/troubleshooting/common-issues)

---

[Next: Quickstart →](/getting-started/quickstart)
