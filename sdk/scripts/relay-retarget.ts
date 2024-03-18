import { DefaultElectrsClient } from "../src/electrs";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { exec } from "child_process";

const args = yargs(hideBin(process.argv))
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
    .option("relay-addr", {
        description: "Relay address",
        type: "string",
        demandOption: true,
    })
    .argv;

main().catch((err) => {
    console.log("Error thrown by script:");
    console.log(err);
});

async function delay(ms: number) {
    return await new Promise(resolve => setTimeout(resolve, ms));
}

async function main(): Promise<void> {
    const electrs = new DefaultElectrsClient(args["network"]);

    let privateKey: string;
    if (args["private-key"]) {
        privateKey = args["private-key"];
    } else if (args["dev"]) {
        privateKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    } else {
        throw new Error("No private key");
    }

    const relayAddress = args["relay-addr"];
    const rpcUrl = "http://localhost:8545";

    while (true) {
        const currentEpoch = await new Promise<number>((resolve, reject) => {
            exec(`cast call ${relayAddress} "currentEpoch() (uint256)" 0 --rpc-url '${rpcUrl}'`,
                (err: any, stdout: string, _stderr: string) => {
                    if (err) reject(`Failed to run command: ${err}`);
                    resolve(Number.parseInt(stdout));
                });
        });
    
        const nextEpoch = currentEpoch + 1;
        const nextRetargetHeight = nextEpoch * 2016;

        console.log(`nextEpoch: ${nextEpoch}`);
    
        // TODO: catch error and retry
        const beforeRetarget = await electrs.getBlockHeaderAt(nextRetargetHeight - 1);
        const afterRetarget = await electrs.getBlockHeaderAt(nextRetargetHeight);

        console.log(`beforeRetarget: ${beforeRetarget}`);
        console.log(`afterRetarget: ${afterRetarget}`);
    
        exec(`RELAY_ADDRESS=${relayAddress} RETARGET_HEADERS=${beforeRetarget}${afterRetarget} PRIVATE_KEY=${privateKey} forge script ../script/RelayRetarget.s.sol:RelayRetargetScript --rpc-url '${rpcUrl}' --broadcast`,
            (err: any, stdout: string, stderr: string) => {
                if (err) {
                    throw new Error(`Failed to run command: ${err}`);
                }

                // console.log(`${stdout}`);
            });

        await delay(10000);
    }
}