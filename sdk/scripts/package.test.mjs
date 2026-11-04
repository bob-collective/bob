import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import { createRequire } from 'node:module';
import test from 'node:test';

const require = createRequire(import.meta.url);
const root = new URL('../', import.meta.url);
const metadata = JSON.parse(readFileSync(new URL('package.json', root), 'utf8'));

test('CommonJS and native ESM expose the same public API and Bitcoin amount behavior', async () => {
    const commonjs = require(new URL(metadata.main, root).pathname);
    const esm = await import(new URL(metadata.module, root));
    assert.deepEqual(Object.keys(esm).sort(), Object.keys(commonjs).sort());
    for (const sdk of [commonjs, esm]) {
        assert.equal(sdk.parseBtc('0.12345678'), 12345678n);
        assert.equal(sdk.formatBtc(12345678n), '0.12345678');
        assert.equal(typeof sdk.GatewaySDK, 'function');
        assert.equal(typeof sdk.ScureBitcoinSigner.fromSeedPhrase, 'function');
    }
});

test('ESM keeps external viem imports and contains no CommonJS require calls', () => {
    const output = new URL('dist/esm/', root);
    const files = readdirSync(output, { recursive: true }).filter((filename) => filename.endsWith('.js'));
    assert.ok(files.length > 0);
    for (const filename of files) {
        assert.doesNotMatch(readFileSync(new URL(filename, output), 'utf8'), /\brequire\s*\(/, filename);
    }
    assert.match(readFileSync(new URL('gateway/utils/common.js', output), 'utf8'), /from ['"]viem['"]/);
});
