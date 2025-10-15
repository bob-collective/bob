import { Address, Hex } from 'viem';

export type OrderDetailsRaw = {
    version: string;
    data: {
        ethAmountToReceive: string;
        satsToSwapToEth: number;
        ethTransferGasLimit: string;
        strategyGasLimit: string;
        totalUserGasLimit: string;
        userGasPriceLimit: string;
        l1DataFee: string;
        extraSatsFee: string | null;
        extraSatsFeeRecipient: Address | null;
    };
};

export type OrderDetails = {
    /** @description Order version identifier */
    version: string;
    data: {
        /** @description The amount of ETH (in wei) that the user will receive */
        ethAmountToReceive: bigint;
        /** @description Amount of satoshis to be swapped to ETH */
        satsToSwapToEth: number;
        /** @description Estimated gas limit for the ETH transfer step */
        ethTransferGasLimit: bigint;
        /** @description Estimated gas limit for executing the strategy logic */
        strategyGasLimit: bigint;
        /** @description User gas limit to finalize transaction */
        totalUserGasLimit: bigint;
        /** @description Maximum gas price (in wei) the user is willing to pay */
        userGasPriceLimit: bigint;
        /** @description The estimated Layer 1 (L1) calldata cost in wei */
        l1DataFee: bigint;
        /** @description Optional additional fee in satoshis to be included */
        extraSatsFee: bigint | null;
        /** @description Optional recipient address for the extra fee (if any) */
        extraSatsFeeRecipient: Address | null;
    };
};

/** @dev Internal */
export type GatewayCreateOrderRequest = {
    gatewayAddress: Address;
    strategyAddress?: Address;
    satsToConvertToEth: number;
    userAddress: Address;
    gatewayExtraData?: Hex;
    strategyExtraData?: Hex;
    satoshis: number;
    campaignId?: string;
    orderDetails?: OrderDetails;
};

export type GatewayCreateOrderRequestPayload = {
    gatewayAddress: Address;
    strategyAddress?: Address;
    userAddress: Address;
    gatewayExtraData?: Hex;
    strategyExtraData?: Hex;
    satoshis: number;
    campaignId?: string;
    orderDetails?: OrderDetailsRaw;
};

/** @dev Internal */
export type GatewayCreateOrderResponse = {
    uuid: string;
    opReturnHash: string;
};

/** @dev The success type on create order */
export type GatewayStartOrder = GatewayCreateOrderResponse & {
    bitcoinAddress: string;
    satoshis: number;
    psbtBase64?: string;
};

export enum GatewayOrderType {
    Onramp = 'onramp',
    Offramp = 'offramp',
    CrossChainSwap = 'crosschain-swap',
}
