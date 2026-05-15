# Coven Documentation Structure

## Overview

This documentation is built with **Mintlify** (same framework as OpenClaw docs), providing a modern, fast, and SEO-friendly documentation experience.

## Directory Tree

```
coven/docs/
├── docs/                                    # Source markdown files
│   ├── getting-started/                    # 🚀 Getting Started Section
│   │   ├── index.md                        # Welcome & overview
│   │   ├── what-is-coven.md               # What is Coven?
│   │   ├── concepts.md                     # Key concepts
│   │   ├── install.md                      # Installation guide
│   │   ├── quickstart.md                   # 5-minute quickstart
│   │   └── first-agent.md                  # Your first agent
│   │
│   ├── core/                               # 🏗️ Core Documentation
│   │   ├── architecture/                   # System architecture
│   │   │   ├── overview.md                 # Architecture overview
│   │   │   ├── harnesses.md               # Harness runtime system
│   │   │   ├── familiars.md               # Persistent agents
│   │   │   ├── sessions.md                # Session management
│   │   │   └── memory.md                  # Memory system
│   │   │
│   │   ├── agents/                        # Built-in agents
│   │   │   ├── index.md                   # Agent overview
│   │   │   ├── nova.md                    # Nova (personal assistant)
│   │   │   ├── cody.md                    # Cody (code expert)
│   │   │   ├── sage.md                    # Sage (researcher)
│   │   │   ├── echo.md                    # Echo (writer)
│   │   │   ├── charm.md                   # Charm (coordinator)
│   │   │   └── kitty.md                   # Kitty (admin)
│   │   │
│   │   └── api/                           # API Reference
│   │       ├── gateway.md                 # Gateway API overview
│   │       ├── sessions.md                # Sessions API
│   │       ├── delegation.md              # Delegation API
│   │       ├── memory.md                  # Memory API
│   │       └── events.md                  # Events API
│   │
│   ├── guides/                             # 📚 How-To Guides
│   │   ├── create-agent.md                # Creating agents
│   │   ├── delegate-work.md               # Delegating between agents
│   │   ├── manage-memory.md               # Managing agent memory
│   │   ├── multi-agent-orchestration.md   # Multi-agent workflows
│   │   ├── custom-tools.md                # Building custom tools
│   │   ├── plugin-system.md               # Plugin system
│   │   ├── custom-harness.md              # Custom harnesses
│   │   ├── performance-tuning.md          # Performance optimization
│   │   └── security-best-practices.md     # Security guide
│   │
│   ├── resources/                         # 🔗 Resources & Examples
│   │   ├── troubleshooting/               # Troubleshooting
│   │   │   ├── index.md                   # Overview
│   │   │   ├── common-issues.md           # Common problems
│   │   │   ├── debugging.md               # Debugging guide
│   │   │   └── faq.md                     # Frequently asked questions
│   │   │
│   │   └── examples/                      # Code examples
│   │       ├── index.md                   # Examples overview
│   │       ├── basic-workflow.md          # Basic workflow
│   │       ├── advanced-delegation.md     # Advanced delegation
│   │       └── memory-patterns.md         # Memory patterns
│   │
│   ├── styles.css                         # Documentation styles
│   └── images/                            # (for future screenshots)
│
├── scripts/docs-site/                     # Build scripts
│   ├── build.mjs                         # Main build script
│   ├── source-index.mjs                  # Search indexing
│   └── smoke.mjs                         # Quality checks
│
├── docs.json                              # Mintlify configuration
├── package.json                           # Dependencies
├── README.md                              # Build & deployment guide
├── STRUCTURE.md                           # This file
├── .gitignore                             # Git rules
└── dist/                                  # Build output (generated)
    └── docs-site/
        ├── *.html                         # Generated pages
        └── search-index.json              # Search index
```

## Content Organization

### Getting Started (6 pages)
- **Purpose:** New user onboarding
- **Flow:** What is Coven → Concepts → Install → Quickstart → First Agent
- **Audience:** Beginners, evaluating Coven

### Core (17 pages)
- **Architecture (5 pages):** System design, components, memory system
- **Agents (7 pages):** Built-in familiars and their capabilities
- **API (5 pages):** Gateway, sessions, delegation, memory, events
- **Audience:** Developers, architects, power users

### Guides (9 pages)
- **Purpose:** How-to documentation
- **Coverage:** Common tasks, advanced patterns, optimization
- **Audience:** All users looking to accomplish specific tasks

### Resources (8 pages)
- **Troubleshooting (4 pages):** Problems, solutions, debugging, FAQ
- **Examples (4 pages):** Real workflows, code samples, patterns
- **Audience:** Intermediate to advanced users

## Navigation Structure

The navigation in `docs.json` mirrors this hierarchy:

```json
{
  "navigation": {
    "languages": [
      {
        "tabs": [
          { "tab": "Get started", "groups": [...] },
          { "tab": "Core", "groups": [...] },
          { "tab": "Guides", "groups": [...] },
          { "tab": "Resources", "groups": [...] }
        ]
      }
    ]
  }
}
```

## Key Features

### 🎨 Visual Style
- **Primary Color:** Violet (#7C3AED) - mystical, tech-forward
- **Typography:** DM Sans (headings), Fragment Mono (code)
- **Theme:** Dark-friendly, accessible, professional

### 🔍 Search
- Full-text search via Pagefind
- Automatic indexing during build
- Search all page titles, descriptions, and content

### 📱 Responsive
- Mobile-friendly layout
- Touch-friendly navigation
- Fast on all devices

### ⚡ Performance
- Static HTML (fast)
- Minimal dependencies
- Quick build times (~1-2s)
- Optimized for Cloudflare/Vercel

## Build & Deploy

### Local Development

```bash
npm run docs:build:dev
```

### Production Build

```bash
npm run docs:build
```

### Quality Check

```bash
npm run docs:check
```

### Deploy

```bash
# Vercel
npm run docs:build
# Deploy dist/docs-site/

# Cloudflare
npm run docs:build:r2
npm run docs:r2:upload
```

## Content Status

| Section | Status | Pages | Notes |
|---------|--------|-------|-------|
| Getting Started | ✅ Ready | 6 | Fully written, ready to review |
| Core | ⚠️ Partial | 17 | Architecture complete, agents/API are stubs |
| Guides | ⚠️ Stubs | 9 | Structure ready, content TBD |
| Resources | ⚠️ Stubs | 8 | Structure ready, content TBD |

## Next Steps

1. **Fill in stub pages** - Guides, agents, API reference
2. **Add examples** - Real code samples and workflows
3. **Test build** - `npm run docs:check`
4. **Deploy** - Vercel or Cloudflare
5. **Iterate** - Based on user feedback

---

Built with Mintlify. Same platform as OpenClaw docs.
