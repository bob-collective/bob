#!/usr/bin/env node
/**
 * Port the Mintlify docs under docs/gateway/ into this app's content tree.
 *
 * This is re-runnable and always rewrites the whole output tree, so it can be
 * run again if the Mintlify source picks up edits before we cut over. Once
 * Mintlify is switched off, delete this script and edit content/docs directly.
 *
 * Component mapping (Mintlify -> fumadocs-ui):
 *   <Note> | <Info>       -> <Callout>
 *   <Tip>                 -> <Callout type="idea">
 *   <Warning>             -> <Callout type="warn">
 *   <CardGroup cols={n}>  -> <Cards>
 *   <AccordionGroup>      -> <Accordions>
 *   <CodeGroup>           -> <Tabs> / <Tab>, one per fenced block
 *   <Step title="X">      -> <Step> with an "### X" heading
 *
 * Card icons keep their Font Awesome name; src/components/icons.tsx resolves
 * those to Lucide at render time. Frontmatter icons are mapped here, because
 * fumadocs' lucideIconsPlugin expects a Lucide name in the frontmatter itself.
 */
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.join(here, '..');
const SRC = path.resolve(root, '..', 'docs', 'gateway');
const OUT = path.join(root, 'content', 'docs');

const faToLucide = JSON.parse(
  fs.readFileSync(path.join(root, 'src', 'lib', 'fa-to-lucide.json'), 'utf8'),
);

/** Mintlify callout tag -> extra props on the fumadocs <Callout>. */
const CALLOUTS = {
  Note: '',
  Info: '',
  Tip: ' type="idea"',
  Warning: ' type="warn"',
};

/** Straight tag renames where the props carry over unchanged. */
const RENAMES = {
  CardGroup: 'Cards',
  AccordionGroup: 'Accordions',
};

/**
 * Pages we no longer generate. Mintlify let MDX declare inline React components
 * with hooks in scope; MDX does not. The route table was extracted to
 * src/components/supported-routes.tsx and is maintained by hand from here on.
 */
const SKIP = new Set(['gateway/supported-routes.mdx']);

function splitFrontmatter(raw) {
  const match = raw.match(/^---\n([\s\S]*?)\n---\n?/);
  if (!match) return { frontmatter: {}, body: raw };

  const frontmatter = {};
  for (const line of match[1].split('\n')) {
    const kv = line.match(/^([A-Za-z_][\w-]*):\s*(.*)$/);
    if (kv) frontmatter[kv[1]] = kv[2].trim().replace(/^["']|["']$/g, '');
  }
  return { frontmatter, body: raw.slice(match[0].length) };
}

function buildFrontmatter({ title, description, icon }) {
  const lines = ['---'];
  if (title) lines.push(`title: ${JSON.stringify(title)}`);
  if (description) lines.push(`description: ${JSON.stringify(description)}`);

  // Unmapped names are dropped rather than guessed at -- a wrong icon is worse
  // than none, and lucideIconsPlugin throws on names it cannot resolve.
  const lucideName = icon ? faToLucide[icon] : null;
  if (lucideName) lines.push(`icon: ${lucideName}`);

  lines.push('---', '');
  return lines.join('\n');
}

function convertCallouts(body) {
  for (const [tag, extraProps] of Object.entries(CALLOUTS)) {
    body = body
      .replace(new RegExp(`<${tag}(\\s[^>]*)?>`, 'g'), `<Callout${extraProps}>`)
      .replace(new RegExp(`</${tag}>`, 'g'), '</Callout>');
  }
  return body;
}

function convertRenames(body) {
  for (const [from, to] of Object.entries(RENAMES)) {
    // Drop Mintlify-only layout props (e.g. cols={2}); fumadocs <Cards> is a grid already.
    body = body
      .replace(new RegExp(`<${from}(\\s[^>]*)?>`, 'g'), `<${to}>`)
      .replace(new RegExp(`</${from}>`, 'g'), `</${to}>`);
  }
  return body;
}

/**
 * fumadocs' <Step> has no title prop -- the convention is a heading inside the
 * step. Blank lines matter: they are what makes MDX parse the heading as
 * markdown rather than literal text.
 */
function convertSteps(body) {
  return body.replace(/<Step\s+([^>]*)>/g, (full, props) => {
    const title = props.match(/title="([^"]*)"/);
    return title ? `<Step>\n\n### ${title[1]}\n` : '<Step>';
  });
}

/** Matches a fenced block, capturing language, optional meta, and body. */
const FENCE = /^```(\S+)([ \t]+[^\n]*)?\n([\s\S]*?)^```/gm;

function convertCodeGroups(body) {
  return body.replace(/<CodeGroup>\n([\s\S]*?)\n<\/CodeGroup>/g, (full, inner) => {
    const blocks = [];
    for (const match of inner.matchAll(FENCE)) {
      blocks.push({
        lang: match[1],
        title: (match[2] ?? '').trim() || match[1],
        code: match[3],
      });
    }
    if (blocks.length === 0) return full;

    const items = blocks.map((block) => block.title);
    const tabs = blocks
      .map(
        (block) =>
          `<Tab value=${JSON.stringify(block.title)}>\n\n` +
          '```' +
          `${block.lang}\n${block.code}` +
          '```\n\n</Tab>',
      )
      .join('\n\n');

    return `<Tabs items={${JSON.stringify(items)}}>\n\n${tabs}\n\n</Tabs>`;
  });
}

/**
 * Mintlify writes a bare fence title (```bash npm). Shiki expects key="value"
 * meta, so anything without an `=` becomes title="...".
 */
function normalizeFenceTitles(body) {
  return body.replace(/^```(\S+)[ \t]+([^\n]+)$/gm, (full, lang, meta) => {
    if (meta.includes('=')) return full;
    return '```' + lang + ` title=${JSON.stringify(meta.trim())}`;
  });
}

function convert(raw) {
  const { frontmatter, body } = splitFrontmatter(raw);

  let out = body;
  out = convertCodeGroups(out); // before fence normalisation -- emits bare fences
  out = normalizeFenceTitles(out);
  out = convertCallouts(out);
  out = convertRenames(out);
  out = convertSteps(out);

  return buildFrontmatter(frontmatter) + out.trimStart();
}

function findMdx(dir) {
  const found = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) found.push(...findMdx(full));
    else if (entry.name.endsWith('.mdx')) found.push(full);
  }
  return found;
}

const files = findMdx(SRC);
if (files.length === 0) {
  console.error(`No .mdx files found under ${SRC}`);
  process.exit(1);
}

let written = 0;
for (const file of files) {
  const relative = path.relative(SRC, file);
  if (SKIP.has(relative.split(path.sep).join('/'))) {
    console.log(`  ${relative} (skipped -- maintained by hand)`);
    continue;
  }

  const target = path.join(OUT, relative);
  written++;

  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, convert(fs.readFileSync(file, 'utf8')));
  console.log(`  ${relative}`);
}

console.log(`\nPorted ${written} pages into ${path.relative(root, OUT)}`);
