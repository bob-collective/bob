#!/usr/bin/env node
// Checks the agent skills in skills/:
//   - frontmatter against the Agent Skills spec (https://agentskills.io/specification)
//   - every skill is listed in the Claude Code marketplace manifest, and nothing else is
//   - relative links resolve and stay inside their skill directory
//   - the facts the skills restate still match their sources: the /v4 endpoints and
//     error codes in gateway-docs/openapi.json, and the AllowanceHolder spenders in
//     sdk/src/gateway/allowance-holder.ts
//   - with --base <git ref>: the plugin version was bumped if skills/ changed since then
//
// Usage: node .github/scripts/check-skills.mjs [--base origin/master]

import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const SKILLS_DIR = join(ROOT, 'skills');
const MARKETPLACE = join(ROOT, '.claude-plugin/marketplace.json');
const OPENAPI = join(ROOT, 'gateway-docs/openapi.json');
const ALLOWANCE_HOLDER_TS = join(ROOT, 'sdk/src/gateway/allowance-holder.ts');
const SDK_CLIENT_TS = join(ROOT, 'sdk/src/gateway/client.ts');
const API_REFERENCE = join(SKILLS_DIR, 'gateway-integrate/api-reference.md');
const SPENDER_TABLE = join(SKILLS_DIR, 'gateway-swap/SKILL.md');

// Upper-case identifiers the skills may name that are not in the OpenAPI spec.
const KNOWN_IDENTIFIERS = {
    GATEWAY_API_KEY: 'environment variable',
    BOB_GATEWAY_MCP_URL: 'environment variable',
    CAPTCHA_VERIFICATION_FAILED: 'returned by the gateway, but missing from the OpenAPI spec',
};

const FRONTMATTER_KEYS = new Set(['name', 'description', 'license', 'compatibility', 'metadata', 'allowed-tools']);
const MAX_SKILL_LINES = 500;

const errors = new Set();
const fail = (message) => errors.add(message);
const rel = (path) => relative(ROOT, path);
const read = (path) => readFileSync(path, 'utf8');

function walk(dir) {
    return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
        const path = join(dir, entry.name);
        return entry.isDirectory() ? walk(path) : [path];
    });
}

const skillDirs = readdirSync(SKILLS_DIR, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort();
const markdownFiles = walk(SKILLS_DIR).filter((path) => path.endsWith('.md'));

// ── Frontmatter ──────────────────────────────────────────────────────────────

/** Top-level `key: value` pairs of a SKILL.md frontmatter, keeping each raw value. */
function readFrontmatter(file) {
    const match = read(file).match(/^---\n([\s\S]*?)\n---\n/);
    if (!match) {
        fail(`${rel(file)}: no YAML frontmatter`);
        return {};
    }
    const fields = {};
    for (const line of match[1].split('\n')) {
        if (line.trim() === '' || /^\s/.test(line)) continue; // blank, or nested under `metadata`
        const pair = line.match(/^([A-Za-z][\w-]*):\s*(.*)$/);
        if (!pair) {
            fail(`${rel(file)}: cannot parse frontmatter line "${line}"`);
            continue;
        }
        const [, key, raw] = pair;
        fields[key] = { raw, value: raw.replace(/^(['"])(.*)\1$/, '$2') };
    }
    return fields;
}

for (const dir of skillDirs) {
    const file = join(SKILLS_DIR, dir, 'SKILL.md');
    if (!existsSync(file)) {
        fail(`skills/${dir}: no SKILL.md`);
        continue;
    }
    const fields = readFrontmatter(file);

    for (const [key, { raw }] of Object.entries(fields)) {
        if (!FRONTMATTER_KEYS.has(key)) fail(`${rel(file)}: frontmatter key "${key}" is not in the Agent Skills spec`);
        // Unquoted YAML breaks on ": " and " #", and block scalars defeat simple parsers.
        if (!/^['"]/.test(raw) && (/: | #/.test(raw) || /^[>|]/.test(raw))) {
            fail(`${rel(file)}: quote the "${key}" value or keep it on one line without ": " and " #"`);
        }
    }

    const name = fields.name?.value;
    if (name !== dir) fail(`${rel(file)}: name "${name ?? ''}" must equal the directory name "${dir}"`);
    if (!name || name.length > 64 || !/^[a-z0-9]+(-[a-z0-9]+)*$/.test(name)) {
        fail(`${rel(file)}: name must be 1-64 lowercase letters, digits and single hyphens`);
    }

    const description = fields.description?.value ?? '';
    if (!description) fail(`${rel(file)}: no description`);
    if (description.length > 1024) fail(`${rel(file)}: description is ${description.length} characters; the limit is 1024`);
    if (description && !description.startsWith('Use when')) fail(`${rel(file)}: description must start with "Use when"`);

    const lines = read(file).split('\n').length;
    if (lines > MAX_SKILL_LINES) fail(`${rel(file)}: ${lines} lines; keep SKILL.md under ${MAX_SKILL_LINES} and move detail into files beside it`);
}

// ── Marketplace listing ──────────────────────────────────────────────────────

const plugin = JSON.parse(read(MARKETPLACE)).plugins?.find((entry) => entry.source === './skills');
if (!plugin) {
    fail(`${rel(MARKETPLACE)}: no plugin with "source": "./skills"`);
} else {
    const listed = (plugin.skills ?? []).map((path) => path.replace(/^\.\//, '').replace(/\/$/, ''));
    for (const dir of skillDirs) {
        if (!listed.includes(dir)) fail(`${rel(MARKETPLACE)}: skills/${dir} is not listed under "skills"`);
    }
    for (const dir of listed) {
        if (!skillDirs.includes(dir)) fail(`${rel(MARKETPLACE)}: lists ./${dir}, which is not a directory in skills/`);
    }
}

// ── Links ────────────────────────────────────────────────────────────────────

const LINK = /\]\(([^)\s]+)\)/g;
for (const file of markdownFiles) {
    const [skill] = relative(SKILLS_DIR, file).split(sep);
    const skillRoot = skillDirs.includes(skill) ? join(SKILLS_DIR, skill) : null;
    for (const [, target] of read(file).matchAll(LINK)) {
        if (target.startsWith('#') || /^[a-z][a-z0-9+.-]*:/i.test(target)) continue; // anchor or URL
        const path = resolve(dirname(file), decodeURI(target.split('#')[0]));
        if (!existsSync(path)) {
            fail(`${rel(file)}: broken link "${target}"`);
        } else if (skillRoot && !`${path}${sep}`.startsWith(`${skillRoot}${sep}`)) {
            fail(`${rel(file)}: link "${target}" leaves the skill directory; installers copy each skill on its own`);
        }
    }
}

// ── OpenAPI spec: endpoints and error codes ──────────────────────────────────

const spec = JSON.parse(read(OPENAPI));
const normalize = (path) => path.replace(/\{[^}]*\}/g, '{}');
const METHODS = ['get', 'put', 'post', 'patch', 'delete'];
const specOperations = new Set(
    Object.entries(spec.paths)
        .filter(([path]) => path.startsWith('/v4/'))
        .flatMap(([path, item]) =>
            Object.keys(item)
                .filter((method) => METHODS.includes(method))
                .map((method) => `${method.toUpperCase()} ${normalize(path)}`)
        )
);

const ENDPOINT = /\b(GET|PUT|POST|PATCH|DELETE) (\/v4\/[\w\-/{}]+)/g;
const operationsIn = (file) => [...read(file).matchAll(ENDPOINT)].map(([, method, path]) => [method, path]);

for (const file of markdownFiles) {
    for (const [method, path] of operationsIn(file)) {
        if (!specOperations.has(`${method} ${normalize(path)}`)) fail(`${rel(file)}: ${method} ${path} is not in ${rel(OPENAPI)}`);
    }
}
const documented = new Set(operationsIn(API_REFERENCE).map(([method, path]) => `${method} ${normalize(path)}`));
for (const operation of specOperations) {
    if (!documented.has(operation)) fail(`${rel(API_REFERENCE)}: ${operation} is in the OpenAPI spec but not listed`);
}

const IDENTIFIER = /\b[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+\b/g;
const specIdentifiers = new Set(JSON.stringify(spec).match(IDENTIFIER));
for (const file of markdownFiles) {
    for (const [, identifier] of read(file).matchAll(/`([A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+)`/g)) {
        if (!specIdentifiers.has(identifier) && !(identifier in KNOWN_IDENTIFIERS)) {
            fail(`${rel(file)}: \`${identifier}\` is not in ${rel(OPENAPI)}; fix it, or add it to KNOWN_IDENTIFIERS with a reason`);
        }
    }
}

// ── AllowanceHolder spenders ─────────────────────────────────────────────────

const BASE58 = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';

/** Hex form of a Tron Base58 address: 0x41 prefix, 20-byte address, 4-byte checksum. */
function tronToHex(address) {
    let value = 0n;
    for (const char of address) {
        const digit = BASE58.indexOf(char);
        if (digit < 0) return null;
        value = value * 58n + BigInt(digit);
    }
    const hex = value.toString(16).padStart(50, '0');
    return hex.length === 50 && hex.startsWith('41') ? `0x${hex.slice(2, 42)}` : null;
}

const toHex = (address) => (address.startsWith('T') ? tronToHex(address) : address.toLowerCase());

function sdkSpenders() {
    const source = read(ALLOWANCE_HOLDER_TS);
    const constants = Object.fromEntries(
        [...source.matchAll(/const (\w+): Address = '(0x[0-9a-fA-F]{40})'/g)].map(([, name, address]) => [name, address.toLowerCase()])
    );
    const table = source.match(/ALLOWANCE_HOLDER_BY_CHAIN[^{]*\{([\s\S]*?)\n\}/)?.[1];
    if (!table) {
        fail(`${rel(ALLOWANCE_HOLDER_TS)}: cannot find ALLOWANCE_HOLDER_BY_CHAIN`);
        return {};
    }
    return Object.fromEntries(
        [...table.matchAll(/^\s*['"]?([\w-]+)['"]?:\s*(\w+),/gm)].map(([, chain, constant]) => [chain, constants[constant]])
    );
}

function skillSpenders() {
    const spenders = {};
    const ROW = /^\|(.*)\|\s*`(0x[0-9a-fA-F]{40}|T[1-9A-HJ-NP-Za-km-z]{33})`\s*\|\s*$/gm;
    for (const [, chains, address] of read(SPENDER_TABLE).matchAll(ROW)) {
        for (const [, chain] of chains.matchAll(/`([a-z0-9-]+)`/g)) spenders[chain] = toHex(address);
    }
    return spenders;
}

const expected = sdkSpenders();
const actual = skillSpenders();
for (const [chain, address] of Object.entries(expected)) {
    if (actual[chain] !== address) {
        fail(`${rel(SPENDER_TABLE)}: spender for "${chain}" is ${actual[chain] ?? 'missing'}; the SDK has ${address}`);
    }
}
for (const chain of Object.keys(actual)) {
    if (!(chain in expected)) fail(`${rel(SPENDER_TABLE)}: spender listed for "${chain}", which the SDK does not know`);
}

// Every chain with a spender needs a row in the approve-page table, even if it says "none".
const approvePages = read(SPENDER_TABLE).match(/^\| Source chain \| Approve page \|\n\|[-|]+\|\n((?:\|.*\|\n)+)/m)?.[1];
if (!approvePages) {
    fail(`${rel(SPENDER_TABLE)}: no "| Source chain | Approve page |" table`);
} else {
    const pageChains = new Set(
        [...approvePages.matchAll(/^\|([^|]*)\|/gm)].flatMap(([, cell]) => [...cell.matchAll(/`([a-z0-9-]+)`/g)].map(([, chain]) => chain))
    );
    for (const chain of Object.keys(expected)) {
        if (!pageChains.has(chain)) fail(`${rel(SPENDER_TABLE)}: no approve page row for "${chain}"`);
    }
    for (const chain of pageChains) {
        if (!(chain in expected)) fail(`${rel(SPENDER_TABLE)}: approve page row for "${chain}", which has no spender`);
    }
}

// The legacy-USDT warning must name the token the SDK resets before approving.
const legacyUsdt = read(SDK_CLIENT_TS).match(/export const ETHEREUM_USDT_ADDRESS = '(0x[0-9a-fA-F]{40})'/)?.[1];
if (!legacyUsdt) {
    fail(`${rel(SDK_CLIENT_TS)}: cannot find ETHEREUM_USDT_ADDRESS`);
} else if (!read(SPENDER_TABLE).toLowerCase().includes(legacyUsdt.toLowerCase())) {
    fail(`${rel(SPENDER_TABLE)}: the legacy USDT warning does not name ${legacyUsdt}, which the SDK resets before approving`);
}

// ── Version bump ─────────────────────────────────────────────────────────────

const baseIndex = process.argv.indexOf('--base');
if (baseIndex !== -1 && plugin) {
    const base = process.argv[baseIndex + 1];
    const git = (...args) => execFileSync('git', args, { cwd: ROOT, encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] });
    if (!base) {
        fail('--base needs a git ref');
    } else {
        const changed = git('diff', '--name-only', `${base}...HEAD`, '--', 'skills').trim();
        let basePlugin;
        try {
            basePlugin = JSON.parse(git('show', `${base}:.claude-plugin/marketplace.json`)).plugins?.find(
                (entry) => entry.name === plugin.name
            );
        } catch {
            // The manifest does not exist on the base yet: nothing to compare.
        }
        if (changed && basePlugin?.version === plugin.version) {
            fail(`${rel(MARKETPLACE)}: skills/ changed since ${base}, but "${plugin.name}" is still at version ${plugin.version}`);
        }
    }
}

// ── Report ───────────────────────────────────────────────────────────────────

if (errors.size > 0) {
    for (const message of errors) console.error(`✗ ${message}`);
    console.error(`\n${errors.size} problem(s) in the agent skills.`);
    process.exit(1);
}
console.log(`✓ ${skillDirs.length} skills checked`);
