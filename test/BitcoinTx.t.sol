// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import {Test, console2} from "forge-std/Test.sol";

import {BitcoinTx} from "../src/bridge/BitcoinTx.sol";

contract BitcoinTxTest is Test {
    using BTCUtils for bytes;
    using ValidateSPV for bytes32;

    function test_GetTxHash() public {
        // b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735
        bytes32 txId = hex"3557632d64a45dd8ae2abc4e6d5d1a977e98db24c6a0ae186c265ed972011bb6";

        BitcoinTx.Info memory txInfo = BitcoinTx.Info({
            version: hex"01000000",
            inputVector: hex"01ace8423f874c95f5f9042d7cda6b9f0727251f3059ef827f373a56831cc621a30000000000fdffffff",
            outputVector: hex"01102700000000000022512037679ea62eab55ebfd442c53c4ad46b6b75e45d8a8fa9cb31a87d0df268b029a",
            locktime: hex"00000000"
        });

        bytes32 txHash =
            abi.encodePacked(txInfo.version, txInfo.inputVector, txInfo.outputVector, txInfo.locktime).hash256View();

        console2.logBytes32(txHash);
        assertEq(txId, txHash);
    }

    function test_ProveTx() public {
        bytes32 txId = hex"3557632d64a45dd8ae2abc4e6d5d1a977e98db24c6a0ae186c265ed972011bb6";

        bytes32 merkleRoot = hex"01c6691023f17fd78f2dffc85d9db21b84eb6e77352f494f9437168820dbfb90";
        bytes memory merkleProof =
            hex"ace8423f874c95f5f9042d7cda6b9f0727251f3059ef827f373a56831cc621a371db6dfce8daed1d809275e0862441b3cdfd314eceea5a79ee7aeec69cc70f614082c8b474ccf00906a1e61694fdf0b717790ac3bdf850b36afb8df107aca93b96e7dea43442a944a6ab4f8bed0d25d3d372a836a6042375bc57fee5c5425f67a3920a489b23f9133fc84d7987d990acc7c2569a81b547a5f65385856d90100e54ec14dd40c23c3cf1e61a2a16a03aea0e85d236942ad538262528d6748d20dc6ca7c40d75ba7b782bc3d1302633c6def1531573c6420b99840ecffc0125f8e0f12ec4aa1d74fd5ec8d9a57c154267cb6ff0276835592cb8500d8c3c5650e84b83e73e9094de0c2bdaa4d661a3b1adacfae0f3c0f8007ab1b2be8dbf32f073068979a263152d6c234ad0f4b70f697168502d62ead0c0194bcf77321a85a1e127afc4477dcc3c3636a7818601d9ff43f837b15ef74d387c688fc0a45b79aec0b6";
        uint256 txIndexInBlock = 407;
        require(
            txId.prove(merkleRoot, merkleProof, txIndexInBlock),
            "Tx merkle proof is not valid for provided header and tx hash"
        );
    }

    function test_GetTxOutputValue() public {
        // b1273a6c00eba20ee8837e445599d1362e005f6e1a8525802ba57bc515461a3a
        uint64 value = BitcoinTx.getTxOutputValue(
            keccak256(hex"16001493adab0a7a8cb7675db135c9c97e81942025c2c9"),
            hex"02c67d16000000000016001493adab0a7a8cb7675db135c9c97e81942025c2c9aea79b4200000000160014f60834ef165253c571b11ce9fa74e46692fc5ec1"
        );
        assertEq(value, 1473990);
    }
}
