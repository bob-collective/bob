export type EvmAddress = string;

type GatewayQuote = {
    onramp_address: EvmAddress;
    dust_threshold: number;
    satoshis: number;
    fee: number;
    gratuity: string;
    bitcoin_address: string;
    tx_proof_difficulty_factor: number;
};

type GatewayOrderResponse = {
    onramp_address: EvmAddress;
    token_address: EvmAddress;
    txid: string;
    status: boolean;
    timestamp: number;
    tokens: string;
    satoshis: number;
    fee: number;
    tx_proof_difficulty_factor: number;
};

export class GatewayApiClient {
    private baseUrl: string;

    constructor(baseUrl: string) {
        this.baseUrl = baseUrl;
    }

    async getQuote(address: string, atomicAmount?: number | string): Promise<GatewayQuote> {
        const response = await fetch(`${this.baseUrl}/quote/${address}/${atomicAmount || ''}`, {
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            }
        });

        return await response.json();
    }

    // TODO: add error handling
    async createOrder(contractAddress: string, userAddress: EvmAddress, atomicAmount: number | string): Promise<string> {
        const response = await fetch(`${this.baseUrl}/order`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            },
            body: JSON.stringify({ onramp_address: contractAddress, user_address: userAddress, satoshis: atomicAmount })
        });

        if (!response.ok) {
            throw new Error('Failed to create order');
        }

        return await response.json();
    }

    async updateOrder(id: string, tx: string) {
        const response = await fetch(`${this.baseUrl}/order/${id}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            },
            body: JSON.stringify({ bitcoin_tx: tx })
        });

        if (!response.ok) {
            throw new Error('Failed to update order');
        }
    }

    async getOrders(userAddress: EvmAddress): Promise<GatewayOrderResponse[]> {
        const response = await fetch(`${this.baseUrl}/orders/${userAddress}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json'
            }
        });

        return response.json();
    }
}

