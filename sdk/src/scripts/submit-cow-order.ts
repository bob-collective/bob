import { createPublicClient, createWalletClient, erc20Abi, http, recoverTypedDataAddress } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { base } from 'viem/chains';

const PRIVATE_KEY = process.env.PRIVATE_KEY as `0x${string}`;
if (!PRIVATE_KEY) throw new Error('PRIVATE_KEY required');

const account = privateKeyToAccount(PRIVATE_KEY);

const walletClient = createWalletClient({
    account,
    chain: base,
    transport: http(),
});

const publicClient = createPublicClient({
    chain: base,
    transport: http(),
});

const QUOTE_URL = 'https://api.cow.fi/base/api/v1/quote';
const ORDER_URL = 'https://api.cow.fi/base/api/v1/orders';

const WBTC = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c' as `0x${string}`;
const USDC = '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913' as `0x${string}`;
const VAULT_RELAYER = '0xC92E8bdf79f0507f65a392b0ab4667716BFE0110' as `0x${string}`;

async function parseJsonSafe(res: Response) {
    const text = await res.text();

    let json;
    try {
        json = JSON.parse(text);
    } catch {
        throw new Error(`API returned non-JSON\nStatus: ${res.status}\nBody:\n${text}`);
    }

    if (!res.ok) {
        throw new Error(`API request failed\nStatus: ${res.status}\nBody:\n${text}`);
    }

    return json;
}

async function main() {
    console.log('Trader:', account.address);

    const sellAmountBeforeFee = 20000n;

    const balance = await publicClient.readContract({
        address: WBTC,
        abi: erc20Abi,
        functionName: 'balanceOf',
        args: [account.address],
    });

    console.log('WBTC balance:', balance.toString());

    if (balance < sellAmountBeforeFee) {
        throw new Error('Insufficient WBTC');
    }

    const allowance = await publicClient.readContract({
        address: WBTC,
        abi: erc20Abi,
        functionName: 'allowance',
        args: [account.address, VAULT_RELAYER],
    });

    if (allowance < sellAmountBeforeFee) {
        console.log('Approving WBTC...');

        const hash = await walletClient.writeContract({
            address: WBTC,
            abi: erc20Abi,
            functionName: 'approve',
            args: [VAULT_RELAYER, sellAmountBeforeFee],
        });

        console.log('Approval tx:', hash);

        await publicClient.waitForTransactionReceipt({ hash });
    }

    console.log('Requesting quote...');

    const quoteRes = await fetch(QUOTE_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            sellToken: WBTC,
            buyToken: USDC,
            from: account.address,
            receiver: account.address,
            kind: 'sell',
            sellAmountBeforeFee: sellAmountBeforeFee.toString(),
            partiallyFillable: false,
            sellTokenBalance: 'erc20',
            buyTokenBalance: 'erc20',
            signingScheme: 'eip712',
        }),
    });

    const quoteData = await parseJsonSafe(quoteRes);

    if (!quoteData.quote) {
        throw new Error('Quote missing\n' + JSON.stringify(quoteData, null, 2));
    }

    const q = quoteData.quote;

    console.log('Quote received:', q);

    const sellAmountTotal = BigInt(q.sellAmount) + BigInt(q.feeAmount);

    const order = {
        sellToken: q.sellToken,
        buyToken: q.buyToken,
        receiver: q.receiver,
        sellAmount: sellAmountTotal,
        buyAmount: BigInt(q.buyAmount),
        validTo: q.validTo,
        appData: q.appData,
        feeAmount: 0n,
        kind: q.kind,
        partiallyFillable: q.partiallyFillable,
        sellTokenBalance: q.sellTokenBalance,
        buyTokenBalance: q.buyTokenBalance,
    };

    const domain = {
        name: 'Gnosis Protocol',
        version: 'v2',
        chainId: 8453,
        verifyingContract: '0x9008D19f58AAbD9eD0D60971565AA8510560ab41',
    };

    const types = {
        Order: [
            { name: 'sellToken', type: 'address' },
            { name: 'buyToken', type: 'address' },
            { name: 'receiver', type: 'address' },
            { name: 'sellAmount', type: 'uint256' },
            { name: 'buyAmount', type: 'uint256' },
            { name: 'validTo', type: 'uint32' },
            { name: 'appData', type: 'bytes32' },
            { name: 'feeAmount', type: 'uint256' },
            { name: 'kind', type: 'string' },
            { name: 'partiallyFillable', type: 'bool' },
            { name: 'sellTokenBalance', type: 'string' },
            { name: 'buyTokenBalance', type: 'string' },
        ],
    };

    console.log('Signing order...');

    const signature = await walletClient.signTypedData({
        account,
        domain,
        types,
        primaryType: 'Order',
        message: order,
    });

    console.log('Signature:', signature);

    const signer = await recoverTypedDataAddress({
        domain,
        types,
        primaryType: 'Order',
        message: order,
        signature,
    });

    console.log('Recovered signer:', signer);

    if (signer.toLowerCase() !== account.address.toLowerCase()) {
        throw new Error('Signature invalid');
    }

    console.log('Submitting order...');

    const orderPayload = {
        ...q,
        sellAmount: sellAmountTotal.toString(),
        feeAmount: '0',
        signingScheme: 'eip712',
        signature,
    };

    const orderRes = await fetch(ORDER_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(orderPayload),
    });

    const orderUid = await parseJsonSafe(orderRes);

    console.log('Order id:', orderUid);
    console.log('✅ Order submitted');
}

main().catch((err) => {
    console.error('❌ Script failed:', err);
    process.exit(1);
});
