import { readdirSync, readFileSync } from 'node:fs';
import { isBuiltin } from 'node:module';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const SDK_ROOT = join(__dirname, '..');
const SRC_DIR = join(SDK_ROOT, 'src');

const IMPORT_SPECIFIER = /(?:\bfrom\s+|\bimport\s*\(\s*|\brequire\s*\(\s*|^\s*import\s+)['"]([^'"]+)['"]/gm;

function packageName(specifier: string): string {
    const parts = specifier.split('/');
    return specifier.startsWith('@') ? parts.slice(0, 2).join('/') : parts[0];
}

/** Packages that published code under `src/` loads, keyed by package name, with one importing file each. */
function externalImports(): Map<string, string> {
    const packages = new Map<string, string>();
    for (const file of readdirSync(SRC_DIR, { recursive: true, encoding: 'utf8' })) {
        if (!file.endsWith('.ts')) continue;

        const source = readFileSync(join(SRC_DIR, file), 'utf8');
        for (const [, specifier] of source.matchAll(IMPORT_SPECIFIER)) {
            if (specifier.startsWith('.') || isBuiltin(specifier)) continue;
            packages.set(packageName(specifier), file);
        }
    }
    return packages;
}

describe('package dependencies', () => {
    // Everything under `src/` is compiled into `dist/` and published, so a package it imports must be
    // installed for consumers. A devDependency only resolves there when another package happens to
    // pull in some version of it, and not at all under strict installs (pnpm `hoist=false`, Yarn PnP).
    it('declares every package that src/ imports in dependencies', () => {
        const { dependencies = {}, peerDependencies = {} } = JSON.parse(
            readFileSync(join(SDK_ROOT, 'package.json'), 'utf8')
        );
        const declared = new Set([...Object.keys(dependencies), ...Object.keys(peerDependencies)]);

        const imports = externalImports();
        expect(imports.size).toBeGreaterThan(0);

        const undeclared = [...imports]
            .filter(([name]) => !declared.has(name))
            .map(([name, file]) => `${name} (${file})`);
        expect(undeclared).toEqual([]);
    });
});
