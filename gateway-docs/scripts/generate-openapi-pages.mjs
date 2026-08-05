#!/usr/bin/env node
/**
 * Generate the API reference pages from the Gateway OpenAPI spec.
 *
 * The spec is owned by the backend and refreshed by the openapi-docs-sync
 * workflow -- this script only reads it. Output is regenerated on every build
 * (see the `prebuild` script), so the reference never drifts from the spec.
 *
 * Operations are grouped by their tag (v1 / v2 / v3), matching how the Mintlify
 * docs.json grouped them. v1 and v2 are still built and reachable by URL, but
 * are left out of api-reference/meta.json so they stay out of the sidebar --
 * the equivalent of Mintlify's "hidden": true.
 */
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { generateFiles } from 'fumadocs-openapi';
import { createOpenAPI } from 'fumadocs-openapi/server';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.join(here, '..');

// Run from the app root so the spec path below stays relative. fumadocs bakes
// this string into every generated page, so an absolute path would hard-code
// one machine's layout and break the Vercel build.
process.chdir(root);

const SPEC = './openapi.json';
const OUT = path.join(root, 'content', 'docs', 'api-reference');

// generateFiles only writes -- it never deletes. Without this, an operation that
// gets renamed or dropped in the spec leaves an orphan page behind forever.
// Only the tag folders are generated; overview.mdx and meta.json are ours.
if (fs.existsSync(OUT)) {
  for (const entry of fs.readdirSync(OUT, { withFileTypes: true })) {
    if (entry.isDirectory()) fs.rmSync(path.join(OUT, entry.name), { recursive: true });
  }
}

const openapi = createOpenAPI({ input: [SPEC] });

await generateFiles({
  input: openapi,
  output: OUT,
  per: 'operation',
  groupBy: 'tag',
});

console.log(`Generated API reference pages into ${path.relative(root, OUT)}`);

// api-reference/overview.mdx links to the raw spec, and nothing else serves it
// at that URL -- fumadocs only reads it at build time. Publish a copy so the
// documented download link resolves both here and through the proxy.
const PUBLIC_SPEC = path.join(root, 'public', 'api-reference', 'openapi.json');
fs.mkdirSync(path.dirname(PUBLIC_SPEC), { recursive: true });
fs.copyFileSync(path.join(root, SPEC), PUBLIC_SPEC);

console.log(`Published the spec to ${path.relative(root, PUBLIC_SPEC)}`);
