/* eslint @typescript-eslint/no-var-requires: "off" */

import { DefaultElectrsClient } from "../src/electrs";
const yargs = require("yargs/yargs");
const { hideBin } = require("yargs/helpers");
const { exec } = require('child_process');

const args = yargs(hideBin(process.argv))
    .option("init-height", {
        description: "Height of the bitcoin chain to initialize the relay at",
        type: "number",
        demandOption: true,
    })
    .option("private-key", {
        description: "Private key to submit with",
        type: "string",
        demandOption: true,
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
    .argv;

main().catch((err) => {
    console.log("Error thrown by script:");
    console.log(err);
});

async function main(): Promise<void> {
    const electrs = new DefaultElectrsClient(args["network"]);

    const initHeight = args["init-height"];
    if ((initHeight % 2016) != 0) {
        throw new Error("Invalid genesis height: must be multiple of 2016");
    }

    const genesis = await electrs.getBlockHeaderAt(initHeight);

    const beforeRetarget = await electrs.getBlockHeaderAt(initHeight + 2015);
    const afterRetarget = await electrs.getBlockHeaderAt(initHeight + 2016);

    console.log(`Genesis: ${genesis}`);
    console.log(`beforeRetarget: ${beforeRetarget}`);
    console.log(`afterRetarget: ${afterRetarget}`);

    let rpcUrl: string;
    let verifyOpts: string | undefined;
    if (args["dev"]) {
        rpcUrl = "http://localhost:8545";
    } else {
        rpcUrl = "http://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz";
        verifyOpts = "--verify --verifier blockscout --verifier-url 'https://explorerl2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz/api?'";
    }

    let privateKey: string;
    switch (args["private-key"]) {
        case "dev-0":
            privateKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
            break;
        default:
            privateKey = args["private-key"];
            break;
    }

    exec(`GENESIS_HEIGHT=${initHeight} GENESIS_HEADER=${genesis} RETARGET_HEADERS=${beforeRetarget}${afterRetarget} PRIVATE_KEY=${privateKey} forge script ../script/Relay.s.sol:RelayScript --rpc-url '${rpcUrl}' --chain 901 ${verifyOpts} --broadcast`,
        (err: any, stdout: any, stderr: any) => {
            if (err) {
                throw new Error(`Failed to run command: ${err}`);
            }

            // the *entire* stdout and stderr (buffered)
            console.log(`stdout: ${stdout}`);
        });
}
