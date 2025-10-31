import ecc from '@bitcoinerlab/secp256k1';
import * as bitcoin from 'bitcoinjs-lib';
import {
    Address,
    encodeAbiParameters,
    encodePacked,
    Hex,
    isAddress,
    isAddressEqual,
    maxUint256,
    padHex,
    parseAbiParameters,
    parseEther,
    StateOverride,
    toHex,
    zeroAddress,
} from 'viem';
import { bob, bobSepolia, mainnet } from 'viem/chains';
import { layerZeroOftAbi, quoterV2Abi } from './abi';
import { AllWalletClientParams, GatewayApiClient } from './client';
import { getTokenAddress, getTokenSlots } from './tokens';
import {
    CrossChainOrder,
    ExecuteQuoteParams,
    GatewayOrder,
    GatewayOrderType,
    GetQuoteParams,
    LayerZeroChainInfo,
    LayerZeroDeploymentsMetadataResponse,
    LayerZeroMessagesWalletResponse,
    LayerZeroQuoteParamsExt,
    LayerZeroSendParam,
    LayerZeroTokenDeploymentsResponse,
} from './types';
import {
    computeAllowanceSlot,
    computeBalanceSlot,
    getChainConfig,
    getCrossChainStatus,
    toHexScriptPubKey,
    viemClient,
} from './utils';

bitcoin.initEccLib(ecc);

export class SwapsClient {
    private basePath: string;

    private getChainDeploymentsPromiseCache: Promise<Array<{ name: string; vmId: string; chainId?: number }>> | null =
        null;

    constructor() {
        this.basePath = 'https://api-v2.swaps.xyz/api/';
    }

    private async getChainDeployments(): Promise<Array<{ name: string; vmId: string; chainId?: number }>> {
        if (!this.getChainDeploymentsPromiseCache) {
            this.getChainDeploymentsPromiseCache = this.getJson<
                Array<{ name: string; vmId: string; chainId?: number }>
            >(`${this.basePath}/get-chain-list`).catch((err) => {
                // On failure, clear the cache to allow retries on subsequent calls.
                this.getChainDeploymentsPromiseCache = null;
                throw err;
            });
        }

        return this.getChainDeploymentsPromiseCache;
    }

    async getVmIdForChain(chainKey: string): Promise<string | null> {
        const chainList = await this.getChainDeployments();
        const chain = chainList.find((c) => c.name.toLowerCase() === chainKey.toLowerCase());
        return chain?.vmId || null;
    }

    async getChainId(vmId: number): Promise<number | null> {
        const chainList = await this.getChainDeployments();
        const chain = chainList.find((c) => c.vmId === vmId.toString() || Number(c.vmId) === vmId);
        return chain?.chainId || null;
    }

    private async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return (await response.json()) as Promise<T>;
    }
}
