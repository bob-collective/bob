import { type Address, Chain as ViemChain } from 'viem';
import {
    arbitrum,
    avalanche,
    base,
    berachain,
    bob,
    bsc,
    hyperEvm,
    mainnet,
    optimism,
    robinhood,
    sei,
    soneium,
    sonic,
    unichain,
} from 'viem/chains';

/** 0x Protocol's canonical AllowanceHolder singleton, pre-deployed on most supported EVM chains. */
const ALLOWANCE_HOLDER_0X_CANONICAL = '0x0000000000001fF3684f28c67538d4D072C22734';

/** Gateway self-deployed AllowanceHolder, for chains without 0x's vanity deployment. */
const ALLOWANCE_HOLDER_SELF_DEPLOYED = '0x8fd545b348e84deb145f0179a00c671f0b9519c3';

/** Gateway self-deployed AllowanceHolder on Tron (`TAfbit1ENsRmtZbPQfYU3srURpfYuWYS7K`). */
const ALLOWANCE_HOLDER_TRON = '0x07a39ae4c49dee86e892450b20881f32cd5d500d';

/** Ethereum mainnet, named so `ETHEREUM_USDT_ADDRESS` can read it with its literal type. */
const ETHEREUM = {
    slug: 'ethereum',
    family: 'evm',
    viemChain: mainnet,
    allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL,
    gasEstimation: true,
    usdtReset: '0xdAC17F958D2ee523a2206206994597C13D831ec7',
} as const satisfies ChainRecord;

/** One source or destination chain the Gateway SDK knows about. */
export interface ChainRecord {
    /** Lowercase name the Gateway API uses for the chain (e.g. `srcChain`). */
    readonly slug: string;
    /** Other names the Gateway API uses for the same chain; resolved like `slug`. */
    readonly aliases?: readonly string[];
    readonly family: 'evm' | 'tron';
    /** viem chain config, for chains `getChainConfig` resolves. */
    readonly viemChain?: ViemChain;
    /**
     * AllowanceHolder the Gateway backend uses on this chain, the only spender `executeQuote`
     * approves. Absent means an order from this chain that needs an approval is refused.
     */
    readonly allowanceHolder?: Address;
    /** False when the chain's adapter does not support `estimateGas` / a `gas` field. */
    readonly gasEstimation: boolean;
    /** Token whose non-zero allowance must be reset to 0 before a new approval (USDT). */
    readonly usdtReset?: Address;
}

/**
 * The single source of chain data for the SDK. The tables the rest of the SDK reads
 * (`supportedChainsMapping`, the AllowanceHolder table, the chains without gas estimation and
 * `ETHEREUM_USDT_ADDRESS`) are derived from it.
 *
 * Order matters: `supportedChainsMapping` keeps this order, with each alias right after its chain.
 */
export const CHAINS = [
    {
        slug: 'bob',
        family: 'evm',
        viemChain: bob,
        allowanceHolder: ALLOWANCE_HOLDER_SELF_DEPLOYED,
        gasEstimation: true,
    },
    ETHEREUM,
    { slug: 'sonic', family: 'evm', viemChain: sonic, gasEstimation: true },
    { slug: 'bsc', family: 'evm', viemChain: bsc, allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL, gasEstimation: true },
    {
        slug: 'unichain',
        family: 'evm',
        viemChain: unichain,
        allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL,
        gasEstimation: true,
    },
    { slug: 'bera', family: 'evm', viemChain: berachain, gasEstimation: true },
    { slug: 'sei', family: 'evm', viemChain: sei, gasEstimation: true },
    {
        slug: 'avalanche',
        family: 'evm',
        viemChain: avalanche,
        allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL,
        gasEstimation: true,
    },
    {
        slug: 'base',
        family: 'evm',
        viemChain: base,
        allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL,
        gasEstimation: true,
    },
    { slug: 'soneium', family: 'evm', viemChain: soneium, gasEstimation: true },
    { slug: 'optimism', family: 'evm', viemChain: optimism, gasEstimation: true },
    {
        slug: 'arbitrum',
        family: 'evm',
        viemChain: arbitrum,
        allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL,
        gasEstimation: true,
    },
    {
        slug: 'hyperevm',
        // `hyperliquid` is the staging gateway's name for the same chain (id 999);
        // accept both until the API canonicalizes to `hyperevm`.
        aliases: ['hyperliquid'],
        family: 'evm',
        viemChain: hyperEvm,
        allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL,
        gasEstimation: true,
    },
    {
        slug: 'robinhood',
        family: 'evm',
        viemChain: robinhood,
        allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL,
        gasEstimation: true,
    },
    // The Tron adapter is not required to implement `estimateGas`, and its `sendTransaction`
    // takes no `gas` field (README — required surface), so a limit there breaks the contract.
    { slug: 'tron', family: 'tron', allowanceHolder: ALLOWANCE_HOLDER_TRON, gasEstimation: false },
    { slug: 'plasma', family: 'evm', allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL, gasEstimation: true },
    { slug: 'polygon', family: 'evm', allowanceHolder: ALLOWANCE_HOLDER_0X_CANONICAL, gasEstimation: true },
] as const satisfies readonly ChainRecord[];

type Chains = (typeof CHAINS)[number];

/** Every name (slug or alias) of `C`. */
type NamesOf<C> = C extends { slug: infer S extends string; aliases: readonly (infer A extends string)[] }
    ? S | A
    : C extends { slug: infer S extends string }
      ? S
      : never;

/** `{ [slug | alias]: viemChain }` for every record with a viem chain. */
export type ViemChainsByName = {
    readonly [C in Extract<Chains, { viemChain: ViemChain }> as NamesOf<C>]: C['viemChain'];
};

function namesOf(chain: ChainRecord): string[] {
    return [chain.slug, ...(chain.aliases ?? [])];
}

const records: readonly ChainRecord[] = CHAINS;

/** `[name, value]` for every slug and alias of every record where `pick` returns a value. */
function entriesByName<T>(pick: (chain: ChainRecord) => T | undefined): [string, T][] {
    return records.flatMap((chain) => {
        const value = pick(chain);
        return value === undefined ? [] : namesOf(chain).map((name): [string, T] => [name, value]);
    });
}

/** viem chain config by slug and alias, in registry order. Backs `supportedChainsMapping`. */
export const VIEM_CHAINS_BY_NAME = Object.fromEntries(entriesByName((chain) => chain.viemChain)) as ViemChainsByName;

/** AllowanceHolder by slug and alias, for chains that have one. */
export const ALLOWANCE_HOLDER_BY_CHAIN: Readonly<Record<string, Address>> = Object.fromEntries(
    entriesByName((chain) => chain.allowanceHolder)
);

/** Slugs and aliases of chains that must not get a gas estimate or `gas` field. */
export const CHAINS_WITHOUT_GAS_ESTIMATION: ReadonlySet<string> = new Set(
    records.flatMap((chain) => (chain.gasEstimation ? [] : namesOf(chain)))
);

/** USDT on Ethereum, whose allowance must be reset to 0 before it is changed. */
export const ETHEREUM_USDT_ADDRESS = ETHEREUM.usdtReset;
