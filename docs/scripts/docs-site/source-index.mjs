#!/usr/bin/env node

/**
 * Create searchable index from documentation
 * Generates JSON for Pagefind search indexing
 */

import fs from 'fs';
import path from 'path';
import matter from 'gray-matter';

const docsDir = path.join(process.cwd(), 'docs');
const indexPath = path.join(process.cwd(), 'dist', 'docs-site', 'search-index.json');

console.log('🔍 Indexing documentation for search...');

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

const files = findMarkdownFiles(docsDir);
const index = [];

for (const file of files) {
  try {
    const fullPath = path.join(docsDir, file);
    const content = fs.readFileSync(fullPath, 'utf8');
    const { data, content: markdown } = matter(content);
    
    // Create search entry
    const url = '/' + file.replace(/\.md$/, '').replace(/\/index$/, '');
    
    index.push({
      id: file,
      title: data.title || 'Untitled',
      description: data.description || '',
      url,
      content: markdown.substring(0, 500), // First 500 chars
      tags: data.tags || []
    });
  } catch (error) {
    console.error(`Error indexing ${file}:`, error.message);
  }
}

// Write index
fs.mkdirSync(path.dirname(indexPath), { recursive: true });
fs.writeFileSync(indexPath, JSON.stringify(index, null, 2));

console.log(`✅ Created search index with ${index.length} documents`);
