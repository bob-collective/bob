// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

import {Test, console2} from "forge-std/Test.sol";

import {BitcoinTx} from "../src/bridge/BitcoinTx.sol";

contract BitcoinTxTest is Test {
    function test_GetTxOutputValue() public {        
        // b1273a6c00eba20ee8837e445599d1362e005f6e1a8525802ba57bc515461a3a
        uint64 value = BitcoinTx.getTxOutputValue(
            keccak256(hex"16001493adab0a7a8cb7675db135c9c97e81942025c2c9"),
            hex"02c67d16000000000016001493adab0a7a8cb7675db135c9c97e81942025c2c9aea79b4200000000160014f60834ef165253c571b11ce9fa74e46692fc5ec1"
        );
        assertEq(value, 1473990);
    }
}
