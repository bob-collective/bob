import { assert, describe, expect, it } from "vitest";
import { GatewaySDK } from "../src/gateway";
import { MAINNET_GATEWAY_BASE_URL } from "../src/gateway/client";
import { SYMBOL_LOOKUP } from "../src/gateway/tokens";
import { Chain, ChainId } from "../src/gateway/types";
import { ZeroAddress } from "ethers";
import nock from "nock";
import * as bitcoin from "bitcoinjs-lib";

describe("Gateway Tests", () => {
    it("should get chains", async () => {
        const gatewaySDK = new GatewaySDK("bob");
        assert.deepEqual(gatewaySDK.getChains(), Object.values(Chain));
    });

    it("should reject invalid chain", async () => {
        expect(() => {
            new GatewaySDK("bob-testnet");
        }).toThrowError("Invalid chain");
    });

    it("should get quote", async () => {
        const gatewaySDK = new GatewaySDK("mainnet");

        const mockQuote = {
            gatewayAddress: ZeroAddress,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: "",
            txProofDifficultyFactor: 3,
            strategyAddress: ZeroAddress,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/quote/${SYMBOL_LOOKUP[ChainId.BOB]["tbtc"].address}?satoshis=1000`)
            .times(5)
            .reply(200, mockQuote);

        assert.deepEqual(await gatewaySDK.getQuote({
            toChain: "BOB",
            toToken: "tBTC",
            toUserAddress: ZeroAddress,
            amount: 1000,
        }), mockQuote);
        assert.deepEqual(await gatewaySDK.getQuote({
            toChain: "bob",
            toToken: "tbtc",
            toUserAddress: ZeroAddress,
            amount: 1000,
        }), mockQuote);
        assert.deepEqual(await gatewaySDK.getQuote({
            toChain: 60808,
            toToken: "tbtc",
            toUserAddress: ZeroAddress,
            amount: 1000,
        }), mockQuote);
        assert.deepEqual(await gatewaySDK.getQuote({
            toChain: "BOB",
            toToken: SYMBOL_LOOKUP[ChainId.BOB]["tbtc"].address,
            toUserAddress: ZeroAddress,
            amount: 1000,
        }), mockQuote);
        assert.deepEqual(await gatewaySDK.getQuote({
            toChain: "BOB",
            toToken: "tBTC",
            toUserAddress: ZeroAddress,
            amount: 1000,
            gasRefill: 5,
        }), { ...mockQuote, fee: 15 });

        // get the total available without amount
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/quote/${SYMBOL_LOOKUP[ChainId.BOB]["tbtc"].address}`)
            .reply(200, mockQuote);
        assert.deepEqual(await gatewaySDK.getQuote({
            toChain: "BOB",
            toToken: SYMBOL_LOOKUP[ChainId.BOB]["tbtc"].address,
            toUserAddress: ZeroAddress,
        }), mockQuote);
    });

    it("should throw error on invalid token", async () => {
        const gatewaySDK = new GatewaySDK("mainnet");
        await expect(async () => {
            await gatewaySDK.getQuote({
                toChain: "BOB",
                toToken: "unknownToken",
                toUserAddress: ZeroAddress,
                amount: 1000,
            });
        }).rejects.toThrowError("Unknown output token");
    });

    it("should reject invalid network", async () => {
        const gatewaySDK = new GatewaySDK("testnet");
        await expect(async () => {
            await gatewaySDK.getQuote({
                toChain: "BOB",
                toToken: "tbtc",
                toUserAddress: ZeroAddress,
                amount: 1000,
            });
        }).rejects.toThrowError("Invalid output chain");

        await expect(async () => {
            await gatewaySDK.getQuote({
                toChain: "unknownChain",
                toToken: "tbtc",
                toUserAddress: ZeroAddress,
                amount: 1000,
            });
        }).rejects.toThrowError("Invalid output chain");
    });

    it("should start order", async () => {
        const gatewaySDK = new GatewaySDK("bob");

        const mockQuote = {
            gatewayAddress: ZeroAddress,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: "bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d",
            txProofDifficultyFactor: 3,
            strategyAddress: ZeroAddress,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post(`/order`)
            .reply(201, {
                uuid: "00000000-0000-0000-0000-000000000000",
                opReturnHash: "0x10e69ac36b8d7ae8eb1dca7fe368da3d27d159259f48d345ff687ef0fcbdedcd",
            });

        await expect(async () => {
            await gatewaySDK.startOrder(mockQuote, {
                toChain: "BOB",
                toToken: "tBTC",
                toUserAddress: "2N8DbeaBdjkktkRzaKL1tHj9FQELV7jA8Re",
                amount: 1000,
                fromChain: "Bitcoin",
                fromToken: "BTC",
                fromUserAddress: "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq",
            });
        }).rejects.toThrowError("Invalid user address");

        const result = await gatewaySDK.startOrder(mockQuote, {
            toChain: "BOB",
            toToken: "tBTC",
            toUserAddress: ZeroAddress,
            amount: 1000,
            fromChain: "Bitcoin",
            fromToken: "BTC",
            fromUserAddress: "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq",
        });

        assert.isDefined(result.psbtBase64);
        const psbt = bitcoin.Psbt.fromBase64(result.psbtBase64!);
        assert.deepEqual(
            psbt.txOutputs[0].script,
            bitcoin.script.compile([
                bitcoin.opcodes.OP_RETURN,
                Buffer.from(result.opReturnHash.slice(2), "hex")
            ])
        );
    });

    it("should get strategies", async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/strategies`)
            .reply(200, [{
                strategyAddress: ZeroAddress,
                inputTokenAddress: SYMBOL_LOOKUP[ChainId.BOB]["tbtc"].address,
                name: "",
                slug: "",
                type: "staking"
            }]);
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/quote/${SYMBOL_LOOKUP[ChainId.BOB]["tbtc"].address}?satoshis=1000&strategy=${ZeroAddress}`)
            .times(4)
            .reply(200, {
                gatewayAddress: ZeroAddress,
                dustThreshold: 1000,
                satoshis: 1000,
                fee: 10,
                bitcoinAddress: "",
                txProofDifficultyFactor: 3,
                strategyAddress: ZeroAddress,
            });

        const gatewaySDK = new GatewaySDK("bob");
        const strategies = await gatewaySDK.getStrategies();

        assert.lengthOf(strategies, 1);
        assert.isDefined(strategies[0].inputToken);
        assert.isNull(strategies[0].outputToken);

        const strategy = strategies[0];
        await gatewaySDK.getQuote({
            toUserAddress: ZeroAddress,
            amount: 1000,

            toChain: strategy.chain.chainId,
            toToken: strategy.inputToken.symbol,
            strategyAddress: strategy.address,
        });
    });

    it("should get tokens", async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/tokens?includeStrategies=false`)
            .reply(200, [ZeroAddress]);

        const gatewaySDK = new GatewaySDK("bob");
        assert.deepEqual(await gatewaySDK.getTokenAddresses(false), [ZeroAddress]);
    });
});
