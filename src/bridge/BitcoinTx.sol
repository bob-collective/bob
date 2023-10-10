// SPDX-License-Identifier: GPL-3.0-only

// Forked from https://github.com/keep-network/tbtc-v2

pragma solidity 0.8.17;

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {BytesLib} from "@bob-collective/bitcoin-spv/BytesLib.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import "./BridgeState.sol";

/// @title Bitcoin transaction
/// @notice Allows to reference Bitcoin raw transaction in Solidity.
/// @dev See https://developer.bitcoin.org/reference/transactions.html#raw-transaction-format
///
///      Raw Bitcoin transaction data:
///
///      | Bytes  |     Name     |        BTC type        |        Description        |
///      |--------|--------------|------------------------|---------------------------|
///      | 4      | version      | int32_t (LE)           | TX version number         |
///      | varies | tx_in_count  | compactSize uint (LE)  | Number of TX inputs       |
///      | varies | tx_in        | txIn[]                 | TX inputs                 |
///      | varies | tx_out_count | compactSize uint (LE)  | Number of TX outputs      |
///      | varies | tx_out       | txOut[]                | TX outputs                |
///      | 4      | lock_time    | uint32_t (LE)          | Unix time or block number |
///
//
///      Non-coinbase transaction input (txIn):
///
///      | Bytes  |       Name       |        BTC type        |                 Description                 |
///      |--------|------------------|------------------------|---------------------------------------------|
///      | 36     | previous_output  | outpoint               | The previous outpoint being spent           |
///      | varies | script_bytes     | compactSize uint (LE)  | The number of bytes in the signature script |
///      | varies | signature_script | char[]                 | The signature script, empty for P2WSH       |
///      | 4      | sequence         | uint32_t (LE)          | Sequence number                             |
///
///
///      The reference to transaction being spent (outpoint):
///
///      | Bytes | Name  |   BTC type    |               Description                |
///      |-------|-------|---------------|------------------------------------------|
///      |    32 | hash  | char[32]      | Hash of the transaction to spend         |
///      |    4  | index | uint32_t (LE) | Index of the specific output from the TX |
///
///
///      Transaction output (txOut):
///
///      | Bytes  |      Name       |     BTC type          |             Description              |
///      |--------|-----------------|-----------------------|--------------------------------------|
///      | 8      | value           | int64_t (LE)          | Number of satoshis to spend          |
///      | 1+     | pk_script_bytes | compactSize uint (LE) | Number of bytes in the pubkey script |
///      | varies | pk_script       | char[]                | Pubkey script                        |
///
///      compactSize uint format:
///
///      |                  Value                  | Bytes |                    Format                    |
///      |-----------------------------------------|-------|----------------------------------------------|
///      | >= 0 && <= 252                          | 1     | uint8_t                                      |
///      | >= 253 && <= 0xffff                     | 3     | 0xfd followed by the number as uint16_t (LE) |
///      | >= 0x10000 && <= 0xffffffff             | 5     | 0xfe followed by the number as uint32_t (LE) |
///      | >= 0x100000000 && <= 0xffffffffffffffff | 9     | 0xff followed by the number as uint64_t (LE) |
///
///      (*) compactSize uint is often references as VarInt)
///
///      Coinbase transaction input (txIn):
///
///      | Bytes  |       Name       |        BTC type        |                 Description                 |
///      |--------|------------------|------------------------|---------------------------------------------|
///      | 32     | hash             | char[32]               | A 32-byte 0x0  null (no previous_outpoint)  |
///      | 4      | index            | uint32_t (LE)          | 0xffffffff (no previous_outpoint)           |
///      | varies | script_bytes     | compactSize uint (LE)  | The number of bytes in the coinbase script  |
///      | varies | height           | char[]                 | The block height of this block (BIP34) (*)  |
///      | varies | coinbase_script  | none                   |  Arbitrary data, max 100 bytes              |
///      | 4      | sequence         | uint32_t (LE)          | Sequence number
///
///      (*)  Uses script language: starts with a data-pushing opcode that indicates how many bytes to push to
///           the stack followed by the block height as a little-endian unsigned integer. This script must be as
///           short as possible, otherwise it may be rejected. The data-pushing opcode will be 0x03 and the total
///           size four bytes until block 16,777,216 about 300 years from now.
library BitcoinTx {
    using BTCUtils for bytes;
    using BTCUtils for uint256;
    using BytesLib for bytes;
    using ValidateSPV for bytes;
    using ValidateSPV for bytes32;

    /// @notice Represents Bitcoin transaction data.
    struct Info {
        /// @notice Bitcoin transaction version.
        /// @dev `version` from raw Bitcoin transaction data.
        ///      Encoded as 4-bytes signed integer, little endian.
        bytes4 version;
        /// @notice All Bitcoin transaction inputs, prepended by the number of
        ///         transaction inputs.
        /// @dev `tx_in_count | tx_in` from raw Bitcoin transaction data.
        ///
        ///      The number of transaction inputs encoded as compactSize
        ///      unsigned integer, little-endian.
        ///
        ///      Note that some popular block explorers reverse the order of
        ///      bytes from `outpoint`'s `hash` and display it as big-endian.
        ///      Solidity code of Bridge expects hashes in little-endian, just
        ///      like they are represented in a raw Bitcoin transaction.
        bytes inputVector;
        /// @notice All Bitcoin transaction outputs prepended by the number of
        ///         transaction outputs.
        /// @dev `tx_out_count | tx_out` from raw Bitcoin transaction data.
        ///
        ///       The number of transaction outputs encoded as a compactSize
        ///       unsigned integer, little-endian.
        bytes outputVector;
        /// @notice Bitcoin transaction locktime.
        ///
        /// @dev `lock_time` from raw Bitcoin transaction data.
        ///      Encoded as 4-bytes unsigned integer, little endian.
        bytes4 locktime;
        // This struct doesn't contain `__gap` property as the structure is not
        // stored, it is used as a function's calldata argument.
    }

    /// @notice Represents data needed to perform a Bitcoin SPV proof.
    struct Proof {
        /// @notice The merkle proof of transaction inclusion in a block.
        bytes merkleProof;
        /// @notice Transaction index in the block (0-indexed).
        uint256 txIndexInBlock;
        /// @notice Single byte-string of 80-byte bitcoin headers,
        ///         lowest height first.
        bytes bitcoinHeaders;
        // This struct doesn't contain `__gap` property as the structure is not
        // stored, it is used as a function's calldata argument.
    }

    /// @notice Represents info about an unspent transaction output.
    struct UTXO {
        /// @notice Hash of the transaction the output belongs to.
        /// @dev Byte order corresponds to the Bitcoin internal byte order.
        bytes32 txHash;
        /// @notice Index of the transaction output (0-indexed).
        uint32 txOutputIndex;
        /// @notice Value of the transaction output.
        uint64 txOutputValue;
        // This struct doesn't contain `__gap` property as the structure is not
        // stored, it is used as a function's calldata argument.
    }

    /// @notice Represents Bitcoin signature in the R/S/V format.
    struct RSVSignature {
        /// @notice Signature r value.
        bytes32 r;
        /// @notice Signature s value.
        bytes32 s;
        /// @notice Signature recovery value.
        uint8 v;
        // This struct doesn't contain `__gap` property as the structure is not
        // stored, it is used as a function's calldata argument.
    }

    /// @notice Validates the SPV proof of the Bitcoin transaction.
    ///         Reverts in case the validation or proof verification fail.
    /// @param txInfo Bitcoin transaction data.
    /// @param proof Bitcoin proof data.
    /// @return txHash Proven 32-byte transaction hash.
    function validateProof(
        BridgeState.Storage storage self,
        Info memory txInfo,
        Proof memory proof
    ) internal view returns (bytes32 txHash) {
        require(
            txInfo.inputVector.validateVin(),
            "Invalid input vector provided"
        );
        require(
            txInfo.outputVector.validateVout(),
            "Invalid output vector provided"
        );

        txHash = abi
            .encodePacked(
                txInfo.version,
                txInfo.inputVector,
                txInfo.outputVector,
                txInfo.locktime
            )
            .hash256View();

        require(
            txHash.prove(
                proof.bitcoinHeaders.extractMerkleRootLE(),
                proof.merkleProof,
                proof.txIndexInBlock
            ),
            "Tx merkle proof is not valid for provided header and tx hash"
        );

        evaluateProofDifficulty(self, proof.bitcoinHeaders);

        return txHash;
    }

    /// @notice Evaluates the given Bitcoin proof difficulty against the actual
    ///         Bitcoin chain difficulty provided by the relay oracle.
    ///         Reverts in case the evaluation fails.
    /// @param bitcoinHeaders Bitcoin headers chain being part of the SPV
    ///        proof. Used to extract the observed proof difficulty.
    function evaluateProofDifficulty(
        BridgeState.Storage storage self,
        bytes memory bitcoinHeaders
    ) internal view {
        IRelay relay = self.relay;
        uint256 currentEpochDifficulty = relay.getCurrentEpochDifficulty();
        uint256 previousEpochDifficulty = relay.getPrevEpochDifficulty();

        uint256 requestedDiff = 0;
        uint256 firstHeaderDiff = bitcoinHeaders
            .extractTarget()
            .calculateDifficulty();

        if (firstHeaderDiff == currentEpochDifficulty) {
            requestedDiff = currentEpochDifficulty;
        } else if (firstHeaderDiff == previousEpochDifficulty) {
            requestedDiff = previousEpochDifficulty;
        } else {
            revert("Not at current or previous difficulty");
        }

        uint256 observedDiff = bitcoinHeaders.validateHeaderChain();

        require(
            observedDiff != ValidateSPV.getErrBadLength(),
            "Invalid length of the headers chain"
        );
        require(
            observedDiff != ValidateSPV.getErrInvalidChain(),
            "Invalid headers chain"
        );
        require(
            observedDiff != ValidateSPV.getErrLowWork(),
            "Insufficient work in a header"
        );

        require(
            observedDiff >= requestedDiff * self.txProofDifficultyFactor,
            "Insufficient accumulated difficulty in header chain"
        );
    }

    /// @notice Extracts public key hash from the provided P2PKH or P2WPKH output.
    ///         Reverts if the validation fails.
    /// @param output The transaction output.
    /// @return pubKeyHash 20-byte public key hash the output locks funds on.
    /// @dev Requirements:
    ///      - The output must be of P2PKH or P2WPKH type and lock the funds
    ///        on a 20-byte public key hash.
    function extractPubKeyHash(BridgeState.Storage storage, bytes memory output)
        internal
        pure
        returns (bytes20 pubKeyHash)
    {
        bytes memory pubKeyHashBytes = output.extractHash();

        require(
            pubKeyHashBytes.length == 20,
            "Output's public key hash must have 20 bytes"
        );

        pubKeyHash = pubKeyHashBytes.slice20(0);

        // The output consists of an 8-byte value and a variable length script.
        // To extract just the script, we ignore the first 8 bytes.
        uint256 scriptLen = output.length - 8;

        // The P2PKH script is 26 bytes long.
        // The P2WPKH script is 23 bytes long.
        // A valid script must have one of these lengths,
        // and we can identify the expected script type by the length.
        require(
            scriptLen == 26 || scriptLen == 23,
            "Output must be P2PKH or P2WPKH"
        );

        if (scriptLen == 26) {
            // Compare to the expected P2PKH script.
            bytes26 script = bytes26(output.slice32(8));

            require(
                script == makeP2PKHScript(pubKeyHash),
                "Invalid P2PKH script"
            );
        }

        if (scriptLen == 23) {
            // Compare to the expected P2WPKH script.
            bytes23 script = bytes23(output.slice32(8));

            require(
                script == makeP2WPKHScript(pubKeyHash),
                "Invalid P2WPKH script"
            );
        }

        return pubKeyHash;
    }

    /// @notice Build the P2PKH script from the given public key hash.
    /// @param pubKeyHash The 20-byte public key hash.
    /// @return The P2PKH script.
    /// @dev The P2PKH script has the following byte format:
    ///      <0x1976a914> <20-byte PKH> <0x88ac>. According to
    ///      https://en.bitcoin.it/wiki/Script#Opcodes this translates to:
    ///      - 0x19: Byte length of the entire script
    ///      - 0x76: OP_DUP
    ///      - 0xa9: OP_HASH160
    ///      - 0x14: Byte length of the public key hash
    ///      - 0x88: OP_EQUALVERIFY
    ///      - 0xac: OP_CHECKSIG
    ///      which matches the P2PKH structure as per:
    ///      https://en.bitcoin.it/wiki/Transaction#Pay-to-PubkeyHash
    function makeP2PKHScript(bytes20 pubKeyHash)
        internal
        pure
        returns (bytes26)
    {
        bytes26 P2PKHScriptMask = hex"1976a914000000000000000000000000000000000000000088ac";

        return ((bytes26(pubKeyHash) >> 32) | P2PKHScriptMask);
    }

    /// @notice Build the P2WPKH script from the given public key hash.
    /// @param pubKeyHash The 20-byte public key hash.
    /// @return The P2WPKH script.
    /// @dev The P2WPKH script has the following format:
    ///      <0x160014> <20-byte PKH>. According to
    ///      https://en.bitcoin.it/wiki/Script#Opcodes this translates to:
    ///      - 0x16: Byte length of the entire script
    ///      - 0x00: OP_0
    ///      - 0x14: Byte length of the public key hash
    ///      which matches the P2WPKH structure as per:
    ///      https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#P2WPKH
    function makeP2WPKHScript(bytes20 pubKeyHash)
        internal
        pure
        returns (bytes23)
    {
        bytes23 P2WPKHScriptMask = hex"1600140000000000000000000000000000000000000000";

        return ((bytes23(pubKeyHash) >> 24) | P2WPKHScriptMask);
    }
}
