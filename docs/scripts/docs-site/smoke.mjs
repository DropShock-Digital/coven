#!/usr/bin/env node

/**
 * Smoke test for documentation build
 * Checks that key pages are built and accessible
 */

import fs from 'fs';
import path from 'path';

const distDir = path.join(process.cwd(), 'dist', 'docs-site');

console.log('🧪 Running smoke tests...');

const testPages = [
  'getting-started/index.html',
  'getting-started/install.html',
  'core/architecture/overview.html',
  'core/agents/index.html',
  'search-index.json'
];

let passed = 0;
let failed = 0;

for (const page of testPages) {
  const fullPath = path.join(distDir, page);
  
  if (fs.existsSync(fullPath)) {
    console.log(`✅ ${page}`);
    passed++;
  } else {
    console.log(`❌ ${page} NOT FOUND`);
    failed++;
  }
}

console.log(`\n📊 Results: ${passed} passed, ${failed} failed`);

if (failed > 0) {
  process.exit(1);
}
