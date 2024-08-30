import { EsploraClient } from "../esplora";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { exec } from "node:child_process";

const args = yargs(hideBin(process.argv))
    .env("RELAY")
    .option("private-key", {
        description: "Private key to submit with",
        type: "string",
    })
    .option("dev", {
        description: "Deploy the contracts locally",
        type: "boolean",
    })
    .option("network", {
        description: "Bitcoin network to use",
        type: "string",
        demandOption: true,
    })
    .option("rpc-url", {
        description: "ETH RPC URL",
        type: "string",
    })
    .option("relay-address", {
        description: "Relay address",
        type: "string",
        demandOption: true,
    }).argv;

main().catch((err) => {
    console.log("Error thrown by script:");
    console.log(err);
    process.exit(1);
});

function range(size: number, startAt = 0) {
    return [...Array(size).keys()].map((i) => i + startAt);
}

async function getRetargetHeaders(esploraClient: EsploraClient, nextRetargetHeight: number, proofLength: number) {
    const beforeRetarget = await Promise.all(
        range(proofLength, nextRetargetHeight - proofLength).map((height) => esploraClient.getBlockHeaderAt(height)),
    );
    const afterRetarget = await Promise.all(
        range(proofLength, nextRetargetHeight).map((height) => esploraClient.getBlockHeaderAt(height)),
    );
    return beforeRetarget.concat(afterRetarget).join("");
}

async function main(): Promise<void> {
    const esploraClient = new EsploraClient(args["network"]);

    let privateKey: string;
    if (args["private-key"]) {
        privateKey = args["private-key"];
    } else if (args["dev"]) {
        privateKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    } else {
        throw new Error("No private key");
    }

    const relayAddress = args["relay-address"];

    let rpcUrl: string;
    if (args["dev"]) {
        rpcUrl = "http://localhost:8545";
    } else if (args["rpc-url"] == "testnet") {
        rpcUrl = "https://bob-sepolia.rpc.gobob.xyz/";
    } else if (args["rpc-url"] == "mainnet") {
        rpcUrl = "https://rpc.gobob.xyz/";
    } else {
        rpcUrl = args["rpc-url"];
    }

    const currentEpoch = await new Promise<number>((resolve, reject) => {
        exec(
            `cast call ${relayAddress} "currentEpoch() (uint256)" --rpc-url '${rpcUrl}'`,
            (err: any, stdout: string, _stderr: string) => {
                if (err) reject(`Failed to run command: ${err}`);
                resolve(Number.parseInt(stdout));
            },
        );
    });
    console.log(`Current epoch: ${currentEpoch}`);

    const proofLength = await new Promise<number>((resolve, reject) => {
        exec(
            `cast call ${relayAddress} "proofLength() (uint256)" --rpc-url '${rpcUrl}'`,
            (err: any, stdout: string, _stderr: string) => {
                if (err) reject(`Failed to run command: ${err}`);
                resolve(Number.parseInt(stdout));
            },
        );
    });
    console.log(`Proof length: ${proofLength}`);

    const nextEpoch = currentEpoch + 1;
    const nextRetargetHeight = nextEpoch * 2016;

    console.log(`Next epoch: ${nextEpoch}`);
    console.log(`Next retarget height: ${nextRetargetHeight}`);

    try {
        await esploraClient.getBlockHash(nextRetargetHeight + proofLength);
    } catch (e) {
        console.log(`Cannot retarget without ${proofLength} headers after ${nextRetargetHeight}. Exiting.`);
        return;
    }

    const retargetHeaders = await getRetargetHeaders(esploraClient, nextRetargetHeight, proofLength);

    let env = {
        RELAY_ADDRESS: relayAddress,
        RETARGET_HEADERS: retargetHeaders,
        PRIVATE_KEY: privateKey,
    };

    exec(
        `forge script ../script/RelayRetarget.s.sol:RelayRetargetScript --rpc-url '${rpcUrl}' --broadcast --priority-gas-price 1`,
        { env: { ...process.env, ...env } },
        (err: any, stdout: string, stderr: string) => {
            if (err) {
                throw new Error(`Failed to run command: ${err}`);
            }

            // the *entire* stdout and stderr (buffered)
            console.log(`stdout: ${stdout}`);
            console.log(`stderr: ${stderr}`);
        },
    );
}
