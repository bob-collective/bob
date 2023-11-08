// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import "../src/paymasters/OracleTokenPaymaster.sol";
import {DummyOracle} from "../src/paymasters/Oracle.sol";
// import {IERC2771} from "../lib/gsn/packages/contracts/src/interfaces/IERC2771Recipient.sol";
import {IRelayHub} from "../lib/gsn/packages/contracts/src/interfaces/IRelayHub.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract OracleTokenPaymasterScript is Script {
    function setUp() public {}

    function run() public {
    
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        IRelayHub relay = IRelayHub(vm.envAddress("RELAY_ADDRESS"));
        address forwarder = vm.envAddress("FORWARDER_ADDRESS");
        ERC20 token = ERC20(vm.envAddress("ERC_ADDRESS"));
        IOracle oracle = IOracle(vm.envAddress("ORACLE_ADDRESS"));
        // IERC2771 target = IERC2771(vm.envAddress("TARGET_ADDRESS"));

        vm.startBroadcast(deployerPrivateKey);
        DummyOracle nativeTokenOracle = new DummyOracle();
        OracleTokenPaymaster paymaster = new OracleTokenPaymaster(nativeTokenOracle);

        relay.depositFor{value: 1 ether}(address(paymaster));
        paymaster.setRelayHub(relay);
        paymaster.setTrustedForwarder(forwarder);

        nativeTokenOracle.setPrice(189100000000); // 1 eth = 1891usd

        paymaster.addOracle(token, token.decimals(), oracle);

//  cast send 0x3D7bA9A1c001b33Abd97648948A751401A546D0F "depositFor(address)" --rpc-url "https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz" --private-key b9176fa68b7c590eba66b7d1894a78fad479d6259e9a80d93b9871c232132c01 0x57249a410CD4730769B0D50e68564DFed5EaC80E --value 1ether &&
//  cast send 0x57249a410CD4730769B0D50e68564DFed5EaC80E "setRelayHub(address)" --rpc-url "https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz" --private-key b9176fa68b7c590eba66b7d1894a78fad479d6259e9a80d93b9871c232132c01 0x3D7bA9A1c001b33Abd97648948A751401A546D0F &&
//  cast send 0x57249a410CD4730769B0D50e68564DFed5EaC80E "setTrustedForwarder(address)" --rpc-url "https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz" --private-key b9176fa68b7c590eba66b7d1894a78fad479d6259e9a80d93b9871c232132c01 0x06F798121007F32d67AA43194E58d3e0e3689392 &&
//  cast send 0xFeCC3F37038999Ede8e58A3c9E5B0E9a16e7d5bC "approve(address,uint)" --rpc-url "https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz" --private-key b9176fa68b7c590eba66b7d1894a78fad479d6259e9a80d93b9871c232132c01 0x57249a410CD4730769B0D50e68564DFed5EaC80E 1ether
        

        vm.stopBroadcast();
    }
}
