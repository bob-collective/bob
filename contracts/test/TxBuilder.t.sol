// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {BitcoinTx} from "../src/utils/BitcoinTx.sol";
import {BitcoinTxBuilder} from "../src/utils/TxBuilder.sol";

import {Test, console2} from "forge-std/Test.sol";

contract TxBuilderTest is Test {
    function test_BitcoinTxBuilder() public {
        bytes32 opReturnData = keccak256(abi.encodePacked(block.timestamp, block.prevrandao, msg.sender));

        BitcoinTxBuilder builder = new BitcoinTxBuilder();
        BitcoinTx.Info memory txInfo = builder.setScript(hex"146142b39c0073672dc382b89a42b29e06368bcabd").setValue(
            15000
        ).setOpReturn(abi.encodePacked(opReturnData)).build();

        BitcoinTx.TxOutputsInfo memory resultInfo =
            BitcoinTx.processTxOutputs(txInfo.outputVector, keccak256(hex"146142b39c0073672dc382b89a42b29e06368bcabd"));

        assertEq(opReturnData, resultInfo.hash);
    }
}
