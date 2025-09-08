import { assert, describe, it } from 'vitest';
import { SwingClient } from '../src/gateway/swing';

describe('Swing Tests', () => {
    it('should get quote', async () => {
        const swingClient = new SwingClient();

        const quote = await swingClient.getQuote({
            fromChain: 'bob',
            toChain: 'base',
            fromTokenAddress: '0xe75D0fB2C24A55cA1e3F96781a2bCC7bdba058F0',
            toTokenAddress: '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913',
            fromTokenSymbol: 'USDC.e',
            toTokenSymbol: 'USDC',
            tokenAmount: '1000000',
        });

        console.log('quote', quote);
    }, 120000);
});
