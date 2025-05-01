// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {BitcoinTx} from "./BitcoinTx.sol";
import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";

contract BitcoinTxBuilder {
    using BTCUtils for uint64;

    struct BitcoinTxConfig {
        bytes script;
        uint64 value;
        bool hasOpReturn;
        bytes opReturnData;
    }

    BitcoinTxConfig private config;

    function setScript(bytes memory _script) public returns (BitcoinTxBuilder) {
        config.script = _script;
        return this;
    }

    function setValue(uint64 _value) public returns (BitcoinTxBuilder) {
        config.value = _value;
        return this;
    }

    function setOpReturn(bytes memory _data) public returns (BitcoinTxBuilder) {
        config.hasOpReturn = true;
        config.opReturnData = _data;
        return this;
    }

    function build() public view returns (BitcoinTx.Info memory) {
        require(config.script.length > 0, "Script cannot be empty");
        if (config.hasOpReturn) {
            require(config.opReturnData.length <= 80, "OP_RETURN data too large"); // Bitcoin standard limit
        }

        bytes memory value = abi.encodePacked(bytes8(config.value.reverseUint64()));

        uint8 numOutputs = 1;
        if (config.hasOpReturn) {
            numOutputs += 1;
        }

        bytes memory outputVector = abi.encodePacked(numOutputs, value, config.script);

        if (config.hasOpReturn) {
            outputVector = abi.encodePacked(
                outputVector,
                hex"0000000000000000", // value (0)
                uint8(config.opReturnData.length + 2),
                hex"6a", // OP_RETURN
                uint8(config.opReturnData.length),
                config.opReturnData
            );
        }

        return BitcoinTx.Info({version: "", inputVector: "", outputVector: outputVector, locktime: ""});
    }
}
