import { existsSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import ts from 'typescript';

const require = createRequire(import.meta.url);
const root = fileURLToPath(new URL('../', import.meta.url));
const output = path.join(root, 'dist/esm');
const result = spawnSync(process.execPath, [require.resolve('typescript/bin/tsc'), '-p', 'tsconfig.esm.json'], {
    cwd: root,
    stdio: 'inherit',
});
if (result.error) throw result.error;
if (result.status !== 0) process.exit(result.status ?? 1);

// Node ESM requires explicit file extensions and does not resolve directory imports.
for (const entry of readdirSync(output, { recursive: true })) {
    if (!entry.endsWith('.js') && !entry.endsWith('.d.ts')) continue;
    const filename = path.join(output, entry);
    let source = readFileSync(filename, 'utf8');
    const imports = ts.preProcessFile(source).importedFiles;
    for (const imported of imports.toReversed()) {
        if (!imported.fileName.startsWith('.')) continue;
        const target = path.resolve(path.dirname(filename), imported.fileName);
        let specifier;
        if (existsSync(`${target}.js`)) specifier = `${imported.fileName}.js`;
        else if (existsSync(path.join(target, 'index.js'))) specifier = `${imported.fileName}/index.js`;
        else if (imported.fileName.endsWith('.js') && existsSync(target)) continue;
        else throw new Error(`Cannot resolve ESM import ${imported.fileName} in ${filename}`);
        const start = imported.pos + 1;
        source = source.slice(0, start) + specifier + source.slice(start + imported.fileName.length);
    }
    writeFileSync(filename, source);
}
writeFileSync(path.join(output, 'package.json'), JSON.stringify({ type: 'module' }) + '\n');
