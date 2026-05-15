# Coven Documentation

Welcome to the Coven documentation site repository. This is where all documentation for the Coven agent framework lives.

## 📁 Structure

```
docs/
├── docs/                      # Markdown source files
│   ├── getting-started/      # Installation, quickstart, first steps
│   ├── core/                 # Architecture, agents, API reference
│   │   ├── architecture/     # Core concepts (harnesses, familiars, sessions, memory)
│   │   ├── agents/          # Built-in agents (Nova, Cody, Sage, etc.)
│   │   └── api/             # Gateway API reference
│   ├── guides/              # How-to guides and tutorials
│   └── resources/           # Troubleshooting and examples
├── scripts/docs-site/        # Build scripts
├── docs.json                 # Mintlify configuration
├── package.json             # Dependencies
└── README.md               # This file
```

## 🚀 Quick Start

### Install Dependencies

```bash
npm install
```

### Build the Documentation

```bash
npm run docs:build
```

This generates static HTML in `dist/docs-site/`.

### Development Mode

```bash
npm run docs:build:dev
```

### Check Build

```bash
npm run docs:check
```

Runs build + smoke tests.

## 📝 Writing Documentation

### Markdown Format

All pages use GitHub-flavored Markdown with YAML frontmatter:

```markdown
---
title: "Page Title"
description: "Brief description for SEO"
---

# Main Heading

Page content here...
```

### File Organization

- One section per directory
- One page per `.md` file
- Use `index.md` for section overviews

### Navigation

Edit `docs.json` to add pages to the navigation structure:

```json
{
  "tabs": [
    {
      "tab": "Get started",
      "groups": [
        {
          "group": "Section Name",
          "pages": [
            "getting-started/index",
            "getting-started/install"
          ]
        }
      ]
    }
  ]
}
```

## 🎨 Styling

Pages use the Mintlify theme via `docs.json`. Customize:

- **Colors** - Edit `colors` in `docs.json`
- **Fonts** - Edit `fonts` in `docs.json`
- **Logo** - Replace files in `assets/`

## 🔍 Search

The documentation supports full-text search via Pagefind. The search index is generated automatically during build:

```bash
npm run docs:build
# Generates search-index.json
```

## 📦 Build Outputs

- **Static HTML** - `dist/docs-site/` (for deployment)
- **Search Index** - `dist/docs-site/search-index.json`
- **Manifest** - Generated during build

## 🌐 Deployment

### Cloudflare Pages

```bash
npm run docs:build
npm run docs:r2:upload
```

### Vercel

```bash
npm run docs:build
# Deploy dist/docs-site/ to Vercel
```

### Local Testing

```bash
npm run docs:build
# Serve dist/docs-site/ with any HTTP server
npx http-server dist/docs-site/
```

## ✅ Quality Checks

### Smoke Tests

```bash
npm run docs:smoke
```

Verifies key pages exist and search index is valid.

### Full Check

```bash
npm run docs:check
```

Runs build + smoke tests.

## 📚 Content Guidelines

### Getting Started

- Installation steps
- Quick start guide
- First agent walkthrough
- Concepts introduction

### Core Documentation

- Architecture explanation
- Component details
- API reference
- Agent profiles

### Guides

- Step-by-step instructions
- Code examples
- Best practices
- Common patterns

### Resources

- Troubleshooting
- FAQ
- Real-world examples
- Advanced topics

## 🔗 Internal Links

Use relative links:

```markdown
[My Link](/getting-started/install)  # Correct
[My Link](../other-page)              # Incorrect
```

## 🏗️ Adding New Pages

1. Create `.md` file in appropriate directory
2. Add YAML frontmatter with title + description
3. Write content using Markdown
4. Add entry to `docs.json` navigation
5. Run `npm run docs:check` to verify

## 📊 Build Performance

Build times depend on documentation size:

- < 100 pages: ~1s
- < 500 pages: ~5s
- > 500 pages: ~15s

Optimize large docs by:

- Breaking into smaller sections
- Using external links for references
- Archiving old content

## 🐛 Troubleshooting

**Build fails with "Cannot find module"**
```bash
npm install
npm run docs:build
```

**Pages not showing up**
- Verify page is added to `docs.json`
- Check frontmatter syntax
- Ensure filename matches configuration

**Search index is empty**
- Run full build: `npm run docs:build`
- Check `dist/docs-site/search-index.json` exists

## 📞 Support

- GitHub Issues: Report bugs and request features
- Discord: Ask questions and get help
- Email: docs@opencoven.ai

## 📄 License

Same as Coven project (TBD)

---

**Built with ❤️ for Coven**
