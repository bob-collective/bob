// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.8.17;

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {BytesLib} from "@bob-collective/bitcoin-spv/BytesLib.sol";
import {SegWitUtils} from "@bob-collective/bitcoin-spv/SegWitUtils.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import {BitcoinTx} from "./BitcoinTx.sol";
import {ILightRelay} from "../relay/ILightRelay.sol";
import {IFullRelayWithVerify} from "../relay/IFullRelayWithVerify.sol";

library WitnessTx {
    using BTCUtils for bytes;
    using BytesLib for bytes;
    using SegWitUtils for bytes;
    using ValidateSPV for bytes32;

    bytes1 constant SEGWIT_MARKER = hex"00";
    bytes1 constant SEGWIT_FLAG = hex"01";

    /// @notice Represents a Bitcoin transaction with the witness data.
    struct WitnessInfo {
        /// @notice Bitcoin transaction info.
        BitcoinTx.Info info;
        /// @notice Bitcoin transaction witness data.
        /// @dev Serialized according to the Bitcoin format.
        bytes witnessVector;
    }

    /// @notice Represents data needed to perform a Bitcoin SPV proof with witness data.
    struct WitnessProof {
        /// @dev From the coinbase witness data.
        bytes32 witnessNonce;
        /// @notice The *witness* merkle root of the payment.
        bytes32 paymentMerkleRoot;
        /// @notice Payment proof, includes the coinbase proof. The coinbasePreimage field can be left empty.
        BitcoinTx.Proof paymentProof;
        /// @notice Coinbase transaction.
        /// @dev Needed to extract the witness commitment.
        BitcoinTx.Info coinbaseTx;
    }

    /// @notice Validates the SPV proof of the Bitcoin transaction with witness data.
    ///         Reverts in case the validation or proof verification fail.
    /// @param txInfo Bitcoin transaction data.
    /// @param proof Bitcoin proof data.
    /// @return wTxHash Proven 32-byte transaction hash.
    function validateWitnessProof(WitnessInfo memory txInfo, WitnessProof memory proof)
        internal
        view
        returns (bytes32 wTxHash)
    {
        require(proof.coinbaseTx.outputVector.validateVout(), "Invalid coinbase output vector provided");
        require(txInfo.info.inputVector.validateVin(), "Invalid payment input vector provided");
        require(txInfo.info.outputVector.validateVout(), "Invalid payment output vector provided");

        // Due to a vulnerability in the Bitcoin SPV proof verification detailed here:
        // https://bitslog.com/2018/06/09/leaf-node-weakness-in-bitcoin-merkle-tree-design/
        // we must ensure that both the coinbase and payment tx are the same length.
        require(
            proof.paymentProof.merkleProof.length == proof.paymentProof.coinbaseProof.length,
            "Tx not on same level of merkle tree as coinbase"
        );

        bytes32 coinbaseTxHash = abi.encodePacked(
            proof.coinbaseTx.version,
            proof.coinbaseTx.inputVector,
            proof.coinbaseTx.outputVector,
            proof.coinbaseTx.locktime
        ).hash256View();

        bytes32 root = proof.paymentProof.bitcoinHeaders.extractMerkleRootLE();

        require(
            coinbaseTxHash.prove(root, proof.paymentProof.coinbaseProof, 0),
            "Coinbase merkle proof is not valid for provided header and hash"
        );

        bytes32 paymentWTxId = abi.encodePacked(
            txInfo.info.version,
            SEGWIT_MARKER,
            SEGWIT_FLAG,
            txInfo.info.inputVector,
            txInfo.info.outputVector,
            txInfo.witnessVector,
            txInfo.info.locktime
        ).hash256View();

        require(
            paymentWTxId.prove(
                proof.paymentMerkleRoot, proof.paymentProof.merkleProof, proof.paymentProof.txIndexInBlock
            ),
            "Tx witness merkle proof is not valid for provided header and tx hash"
        );

        // witnessCommitment = SHA256(witnessMerkleRoot || witnessNonce)
        bytes32 witnessCommitment = abi.encodePacked(proof.paymentMerkleRoot, proof.witnessNonce).hash256View();

        // extract coinbase commitment from tx out
        bytes32 coinbaseWitnessCommitment = proof.coinbaseTx.outputVector.extractWitnessCommitment();
        require(coinbaseWitnessCommitment == witnessCommitment, "Invalid commitment");

        return paymentWTxId;
    }

    /// @notice Validates the witness SPV proof using the light relay.
    /// @param relay Bitcoin relay providing the current Bitcoin network difficulty.
    /// @param txProofDifficultyFactor The number of confirmations required on the Bitcoin chain.
    /// @param txInfo Bitcoin transaction data.
    /// @param proof Bitcoin proof data.
    /// @return wTxHash Proven 32-byte transaction hash.
    function validateWitnessProof(
        ILightRelay relay,
        uint256 txProofDifficultyFactor,
        WitnessInfo memory txInfo,
        WitnessProof memory proof
    ) internal view returns (bytes32 wTxHash) {
        wTxHash = validateWitnessProof(txInfo, proof);

        // Checks that the header chain is valid with respect to the difficulty stored in the light relay
        BitcoinTx.evaluateProofDifficulty(relay, txProofDifficultyFactor, proof.paymentProof.bitcoinHeaders);
    }

    /// @notice Validates the witness SPV proof using the full relay.
    /// @param relay Bitcoin full relay contract.
    /// @param minConfirmations The minimum number of confirmations required on the Bitcoin chain.
    /// @param txInfo Bitcoin transaction data.
    /// @param proof Bitcoin proof data.
    /// @return wTxHash Proven 32-byte transaction hash.
    function validateWitnessProof(
        IFullRelayWithVerify relay,
        uint256 minConfirmations,
        WitnessInfo memory txInfo,
        WitnessProof memory proof
    ) internal view returns (bytes32 wTxHash) {
        wTxHash = validateWitnessProof(txInfo, proof);

        // Checks that the header is valid with respect to the chain stored in the full relay
        bytes32 headerHash = proof.paymentProof.bitcoinHeaders.hash256();
        relay.verifyHeaderHash(headerHash, uint8(minConfirmations));
    }
}
