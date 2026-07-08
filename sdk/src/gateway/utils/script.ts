import * as bitcoin from 'bitcoinjs-lib';

// Kept in its own module (not re-exported from the utils barrel) so that importing the
// gateway quote/execute client does not pull bitcoinjs-lib into the bundle. The quote
// path needs none of this; only callers that build BTC scripts do.
export function toHexScriptPubKey(userAddress: string, network: bitcoin.Network): string {
    const address = bitcoin.address.toOutputScript(userAddress, network);
    const buffer = Buffer.concat([Buffer.from([address.length]), address]);
    return '0x' + buffer.toString('hex'); // Convert buffer to hex string
}
