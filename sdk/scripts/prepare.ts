import { copyFile, mkdir } from 'fs/promises';
import path from 'path';

const tokenlist = path.resolve('..', 'tokenlist', 'tokenlist.json');
const assetsDir = path.resolve('src', 'assets');
const DEST = path.join(assetsDir, 'tokenlist.json');

async function main() {
    try {
        await mkdir(assetsDir, { recursive: true });
        await copyFile(tokenlist, DEST);
        console.log(`Copied: ${tokenlist} -> ${DEST}`);
    } catch (err) {
        console.error('Error copying tokenlist.json:', err);
        process.exit(1);
    }
}

main();
