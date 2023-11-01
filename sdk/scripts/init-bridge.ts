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
    }).argv;
main().catch((err) => {
    console.log("Error thrown by script:");
    console.log(err);
});


async function main(): Promise<void> {
    const electrs = new DefaultElectrsClient("testnet");
    // args["parachain-endpoint"]

    const initHeight = args["init-height"];
    const privateKey = args["private-key"];
    if ((initHeight % 2016) != 0) {
        throw new Error("Invalid genesis height: must be multiple of 2016");
    }

    const genesis = await electrs.getBlockHeaderAt(initHeight);

    const beforeRetarget = await electrs.getBlockHeaderAt(initHeight + 2015);
    const afterRetarget = await electrs.getBlockHeaderAt(initHeight + 2016);

    console.log(`Genesis: ${genesis}`);
    console.log(`beforeRetarget: ${beforeRetarget}`);
    console.log(`afterRetarget: ${afterRetarget}`);
    

    exec(`GENESIS_HEIGHT=${initHeight} GENESIS_HEADER=${genesis} RETARGET_HEADERS=${beforeRetarget}${afterRetarget} PRIVATE_KEY=${privateKey} forge script script/Bridge.s.sol:BridgeScript --rpc-url 'https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz' --chain 901 --verify --verifier blockscout --verifier-url 'https://explorerl2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz/api?' --broadcast`, 
        (err: any, stdout: any, stderr: any) => {
        if (err) {
            throw new Error(`Failed to run command: ${err}`);
        }
      
        // the *entire* stdout and stderr (buffered)
        console.log(`stdout: ${stdout}`);
      });
}
