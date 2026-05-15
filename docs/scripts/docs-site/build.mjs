#!/usr/bin/env node

/**
 * Build Coven documentation site
 * Converts markdown to static HTML using Mintlify-compatible format
 */

import fs from 'fs';
import path from 'path';
import matter from 'gray-matter';
import MarkdownIt from 'markdown-it';
import anchor from 'markdown-it-anchor';

const md = new MarkdownIt();
md.use(anchor);

const docsDir = path.join(process.cwd(), 'docs');
const distDir = path.join(process.cwd(), 'dist', 'docs-site');

// Create dist directory
if (!fs.existsSync(distDir)) {
  fs.mkdirSync(distDir, { recursive: true });
}

// Read configuration
const configPath = path.join(process.cwd(), 'docs.json');
const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));

console.log(`🏗️  Building ${config.name} documentation...`);

// Find all markdown files
function findMarkdownFiles(dir, baseDir = '') {
  const files = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    const relPath = path.join(baseDir, entry.name);
    
    if (entry.isDirectory()) {
      files.push(...findMarkdownFiles(fullPath, relPath));
    } else if (entry.name.endsWith('.md')) {
      files.push(relPath);
    }
  }
  
  return files;
}

// Process markdown file
function processFile(relPath) {
  const fullPath = path.join(docsDir, relPath);
  const content = fs.readFileSync(fullPath, 'utf8');
  const { data, content: markdown } = matter(content);
  
  const html = md.render(markdown);
  
  return {
    path: relPath,
    title: data.title || 'Untitled',
    description: data.description || '',
    html,
    frontmatter: data
  };
}

// Build site
const files = findMarkdownFiles(docsDir);
console.log(`📄 Found ${files.length} markdown files`);

let processedCount = 0;
for (const file of files) {
  try {
    const doc = processFile(file);
    
    // Create output directory
    const outDir = path.join(distDir, path.dirname(file));
    fs.mkdirSync(outDir, { recursive: true });
    
    // Write HTML
    const outPath = path.join(outDir, path.basename(file, '.md') + '.html');
    const htmlContent = `<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>${doc.title} - ${config.name}</title>
  <meta name="description" content="${doc.description}">
  <link rel="stylesheet" href="/styles.css">
</head>
<body>
  <main>${doc.html}</main>
</body>
</html>`;
    
    fs.writeFileSync(outPath, htmlContent);
    processedCount++;
  } catch (error) {
    console.error(`Error processing ${file}:`, error.message);
  }
}

console.log(`✅ Built ${processedCount} pages`);
console.log(`📁 Output: ${distDir}`);
