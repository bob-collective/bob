import { Transaction } from 'bitcoinjs-lib';

/**
 * Txid of a signed raw Bitcoin transaction; throws when it does not parse.
 *
 * Its own module so the client can load it with `import()` only when a registration fails: the
 * transaction parser then stays out of the initial chunk of an app that imports `GatewaySDK`.
 */
export function bitcoinTxId(txHex: string): string {
    return Transaction.fromHex(txHex).getId();
}
