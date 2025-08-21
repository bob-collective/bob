import { encodeAbiParameters, encodePacked, Hex, parseAbiParameters } from 'viem';
import { GatewayApiClient } from './client';
import { GatewayQuote, GatewayTokensInfo, GetQuoteParams, OfframpQuote } from './types';
import { bob } from 'viem/chains';

export class LayerZeroClient {
    private basePath: string;

    constructor() {
        this.basePath = 'https://metadata.layerzero-api.com/v1/metadata';
    }

    async getSupportedChains(): Promise<Array<string>> {
        const params = new URLSearchParams({
            symbols: 'WBTC',
        });

        const data = await this.getJson<{
            WBTC: [{ deployments: { [chainKey: string]: { address: string } } }];
        }>(`${this.basePath}/experiment/ofts/list?${params.toString()}`);

        return Object.keys(data.WBTC[0].deployments);
    }

    async getEidForChain(chainKey: string) {
        const data = await this.getJson<{
            [chainKey: string]: {
                deployments: [
                    {
                        version: number;
                        eid: string;
                    },
                ];
            };
        }>(`${this.basePath}`);

        return data[chainKey]?.deployments.find((item) => item.version === 2)?.eid || null;
    }

    private async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return (await response.json()) as Promise<T>;
    }
}

export class LayerZeroGatewayClient extends GatewayApiClient {
    async getQuote(params: GetQuoteParams): Promise<{
        params: GetQuoteParams;
        onrampQuote?: GatewayQuote & GatewayTokensInfo;
        offrampQuote?: OfframpQuote;
    }> {
        const fromChain = params.fromChain.toString().toLowerCase();
        const toChain = params.toChain.toString().toLowerCase();

        // Handle bitcoin -> bob and bob -> bitcoin: use normal flow
        if (fromChain === 'bitcoin' && params.toChain === bob.id) {
            return super.getQuote(params);
        }

        if (params.fromChain === bob.id && toChain === 'bitcoin') {
            return super.getQuote(params);
        }

        // Handle bitcoin -> l0 chain: need to add calldata
        if (fromChain === 'bitcoin') {
            // TODO: use L0 SDK
            const layerZeroClient = new LayerZeroClient();
            const dstEid = await layerZeroClient.getEidForChain(toChain);

            if (!dstEid) {
                throw new Error(`Unsupported destination chain: ${toChain}`);
            }

            const option = encodePacked(['uint128', 'uint128'], [BigInt(65000), BigInt(0)]);
            const extraOptions = encodePacked(
                ['uint8', 'uint8', 'uint16', 'uint8', 'bytes'],
                [
                    // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/oapp/contracts/oapp/libs/OptionsBuilder.sol#L22
                    3, // TYPE_3
                    // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/messagelib/contracts/libs/ExecutorOptions.sol#L10
                    1, // WORKER_ID
                    Buffer.from(option, 'hex').length + 1, // +1 for optionType
                    1, // OPTION_TYPE_LZRECEIVE
                    option,
                ]
            );

            // https://github.com/LayerZero-Labs/LayerZero-v2/blob/200cda254120375f40ed0a7e89931afb897b8891/packages/layerzero-v2/evm/oapp/contracts/oft/interfaces/IOFT.sol#L10-L18
            const encodedParameters = encodeAbiParameters(
                parseAbiParameters([
                    'uint32 dstEid',
                    'bytes32 to',
                    'uint256 amountLD',
                    'uint256 minAmountLD',
                    'bytes extraOptions',
                    'bytes composeMsg',
                    'bytes oftCmd',
                ]),
                [
                    parseInt(dstEid!, 10),
                    params.toUserAddress as Hex,
                    BigInt(0), // will be added inside the strategy
                    BigInt(0), // will be added inside the strategy
                    extraOptions,
                    '0x',
                    '0x',
                ]
            );

            // TODO: add layerzero strategy address
            params.strategyAddress = '0x';
            params.message = encodedParameters;
            // change to BOB chain for bridging
            params.toChain = bob.id;

            return super.getQuote(params);
        } else if (toChain === 'bitcoin') {
            // TODO add offramp support
            throw new Error(`${fromChain} -> bitcoin is not supported yet`);
        } else {
            throw new Error(`Unsupported chain combination: ${fromChain} -> ${toChain}`);
        }
    }
}
