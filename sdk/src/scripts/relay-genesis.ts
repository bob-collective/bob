import { EsploraClient } from '../esplora';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { exec } from 'node:child_process';

const args = yargs(hideBin(process.argv))
    .option('init-height', {
        description: 'Height of the bitcoin chain to initialize the relay at',
        demandOption: true,
    })
    .option('private-key', {
        description: 'Private key to submit with',
        type: 'string',
    })
    .option('dev', {
        description: 'Deploy the contracts locally',
        type: 'boolean',
    })
    .option('testnet', {
        description: 'Use the testnet relay contract which can override the difficulty',
        type: 'boolean',
    })
    .option('proof-length', {
        description: 'The default proof length for retargets',
        type: 'number',
        default: 1,
    })
    .option('network', {
        description: 'Bitcoin network to use',
        type: 'string',
        demandOption: true,
    })
    .option('rpc-url', {
        description: 'ETH RPC URL',
        type: 'string',
    })
    .option('verifier-url', {
        description: 'Verifier URL',
        type: 'string',
    }).argv;

main().catch((err) => {
    console.log('Error thrown by script:');
    console.log(err);
    process.exit(1);
});

function range(size: number, startAt = 0) {
    return [...Array(size).keys()].map((i) => i + startAt);
}

async function getRetargetHeaders(esploraClient: EsploraClient, nextRetargetHeight: number, proofLength: number) {
    const beforeRetarget = await Promise.all(
        range(proofLength, nextRetargetHeight - proofLength).map((height) => esploraClient.getBlockHeaderAt(height))
    );
    const afterRetarget = await Promise.all(
        range(proofLength, nextRetargetHeight).map((height) => esploraClient.getBlockHeaderAt(height))
    );
    return beforeRetarget.concat(afterRetarget).join('');
}

async function main(): Promise<void> {
    const esploraClient = new EsploraClient(args['network']);

    let initHeight = args['init-height'];
    if (initHeight == 'latest') {
        const currentHeight = await esploraClient.getLatestHeight();
        initHeight = currentHeight - (currentHeight % 2016) - 2016;
        console.log(`Using block ${initHeight}`);
    }
    if (initHeight % 2016 != 0) {
        throw new Error('Invalid genesis height: must be multiple of 2016');
    }

    const genesis = await esploraClient.getBlockHeaderAt(initHeight);

    const proofLength = args['proof-length'];
    const nextRetargetHeight = initHeight + 2016;
    console.log(`Next retarget height: ${nextRetargetHeight}`);

    const retargetHeaders = await getRetargetHeaders(esploraClient, nextRetargetHeight, proofLength);

    let rpcUrl: string;
    let verifyOpts: string | undefined;
    if (args['dev']) {
        rpcUrl = 'http://localhost:8545';
    } else if (args['rpc-url'] == 'testnet') {
        rpcUrl = 'https://bob-sepolia.rpc.gobob.xyz/';
        verifyOpts = "--verify --verifier blockscout --verifier-url 'https://bob-sepolia.explorer.gobob.xyz/api'";
    } else if (args['rpc-url'] == 'mainnet') {
        rpcUrl = 'https://rpc.gobob.xyz/';
        verifyOpts = "--verify --verifier blockscout --verifier-url 'https://explorer.gobob.xyz/api'";
    } else {
        rpcUrl = args['rpc-url'];
        verifyOpts = `--verify --verifier blockscout --verifier-url ${args['verifier-url']}`;
    }

    let privateKey: string;
    if (args['private-key']) {
        privateKey = args['private-key'];
    } else if (args['dev']) {
        privateKey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';
    } else {
        throw new Error('No private key');
    }

    let env = [
        `GENESIS_PROOF_LENGTH=${proofLength}`,
        `GENESIS_HEIGHT=${initHeight}`,
        `GENESIS_HEADER=${genesis}`,
        `RETARGET_HEADERS=${retargetHeaders}`,
        `PRIVATE_KEY=${privateKey}`,
    ];

    if (args['testnet']) {
        env.push('TESTNET=true');
    }

    exec(
        `${env.join(' ')} forge script ../script/RelayGenesis.s.sol:RelayGenesisScript --rpc-url '${rpcUrl}' ${verifyOpts} --broadcast --priority-gas-price 1`,
        { maxBuffer: 1024 * 5000 },
        (err: any, stdout: string, stderr: string) => {
            if (err) {
                throw new Error(`Failed to run command: ${err}`);
            }

            // the *entire* stdout and stderr (buffered)
            console.log(`stdout: ${stdout}`);
            console.log(`stderr: ${stderr}`);
        }
    );
}
