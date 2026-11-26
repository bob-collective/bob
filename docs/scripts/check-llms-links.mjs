// Post-build step for build/llms.txt and build/llms-full.txt:
// 1. points the home page link at `/` (it has `slug: /`, which
//    docusaurus-plugin-llms cannot resolve and emits as `/docs/`);
// 2. fails when either file links a docs.gobob.xyz page the build did not
//    produce.
// Run after `docusaurus build` (see `build:docs`).
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const buildDir = resolve(process.argv[2] ?? "build");
const files = ["llms.txt", "llms-full.txt"];
const LINK = /https:\/\/docs\.gobob\.xyz([A-Za-z0-9_./#?=%-]*)/g;
// `/docs/` or `/docs` not followed by more path.
const HOME_LINK = /https:\/\/docs\.gobob\.xyz\/docs\/?([)\s.,]|$)/g;
// /gateway and /api-reference are rewritten to the separate Gateway docs app
// (see vercel.json), so the Docusaurus build never contains them.
const GATEWAY_APP = /^\/(gateway|api-reference)(\/|$)/;

const pageExists = (path) => {
  const target = join(buildDir, decodeURIComponent(path));
  return (
    path === "/" ||
    existsSync(join(target, "index.html")) ||
    existsSync(`${target}.html`) ||
    (existsSync(target) && statSync(target).isFile())
  );
};

const broken = [];
for (const file of files) {
  const filePath = join(buildDir, file);
  if (!existsSync(filePath)) {
    console.error(`check-llms-links: ${filePath} not found`);
    process.exit(1);
  }
  const original = readFileSync(filePath, "utf8");
  const text = original.replaceAll(HOME_LINK, "https://docs.gobob.xyz/$1");
  if (text !== original) writeFileSync(filePath, text);
  const links = new Set();
  for (const [, rest] of text.matchAll(LINK)) {
    // Drop sentence punctuation, query and fragment; keep a bare path.
    const path = rest.replace(/[.,]+$/, "").replace(/[#?].*$/, "") || "/";
    links.add(path);
  }
  for (const path of links) {
    if (GATEWAY_APP.test(path)) continue;
    if (!pageExists(path)) broken.push(`${file}: https://docs.gobob.xyz${path}`);
  }
}

if (broken.length > 0) {
  console.error(`check-llms-links: ${broken.length} link(s) to pages the build did not produce:`);
  for (const line of broken) console.error(`  ${line}`);
  process.exit(1);
}
console.log("check-llms-links: all docs.gobob.xyz links resolve to built pages");
