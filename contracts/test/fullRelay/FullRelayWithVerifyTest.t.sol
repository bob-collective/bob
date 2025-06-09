// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {FullRelayTestUtils} from "./FullRelayTestUtils.sol";
import {FullRelayWithVerify} from "../../src/relay/FullRelayWithVerify.sol";
import {IFullRelayWithVerify} from "../../src/relay/IFullRelayWithVerify.sol";
import {BitcoinTx} from "../../src/utils/BitcoinTx.sol";
import {WitnessTx} from "../../src/utils/WitnessTx.sol";

import {stdJson} from "forge-std/StdJson.sol";
import {console} from "forge-std/Test.sol";

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";

// Wrapper contract around BitcoinTx library, so we can test transaction reverts
contract BitcoinTxTester {
    function validateProof(
        IFullRelayWithVerify relay,
        uint256 minConfirmations,
        BitcoinTx.Info memory txInfo,
        BitcoinTx.Proof memory proof
    ) external view returns (bytes32 txHash) {
        return BitcoinTx.validateProof(relay, minConfirmations, txInfo, proof);
    }
}

contract WitnessTxTester {
    function validateWitnessProof(
        IFullRelayWithVerify relay,
        uint256 minConfirmations,
        WitnessTx.WitnessInfo memory txInfo,
        WitnessTx.WitnessProof memory proof
    ) external view returns (bytes32 txHash) {
        return WitnessTx.validateWitnessProof(relay, minConfirmations, txInfo, proof);
    }
}

contract FullRelayWithVerifyTest is FullRelayTestUtils {
    uint256 constant CHAIN_LENGTH = 18;
    bytes header =
        hex"0000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d38782b";
    bytes proof =
        hex"e35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a65d9c87d07de21d4dfe7b0a9f4a23cc9a58373e9e6931fefdb5afade5df54c91104048df1ee999240617984e18b6f931e2373673d0195b8c6987d7ff7650d5ce53bcec46e13ab4f2da1146a7fc621ee672f62bc22742486392d75e55e67b09960c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c821937847768d92837bae3832bb8e5a4ab4434b97e00a6c10182f211f592409068d6f5652400d9a3d1cc150a7fb692e874cc42d76bdafc842f2fe0f835a7c24d2d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64";
    bytes32 txToVerify = hex"48e5a1a0e616d8fd92b4ef228c424e0c816799a256c6a90892195ccfc53300d6";
    uint256 txId = 281;

    constructor() FullRelayTestUtils("headers.json", ".genesis.hex", ".genesis.height", ".genesis.digest_le") {
        relay.setAncestorOverride(true, true);
    }
}

contract FullRelayWithVerifyDirectTest is FullRelayWithVerifyTest {
    function testMalformedProofSupplied() public {
        vm.expectRevert(bytes("Bad merkle array proof"));
        relay.verifyProof(header, hex"00", txToVerify, txId, 0);
    }

    function testIncorrectProofSupplied() public {
        vm.expectRevert(bytes("Bad inclusion proof"));
        relay.verifyProof(
            header, hex"0000000000000000000000000000000000000000000000000000000000000000", txToVerify, txId, 0
        );
    }

    function testIncorrectTxSupplied() public {
        vm.expectRevert(bytes("Bad inclusion proof"));
        relay.verifyProof(header, proof, hex"00", txId, 0);
    }

    function testIncorrectTxIdSupplied() public {
        vm.expectRevert(bytes("Bad inclusion proof"));
        relay.verifyProof(header, proof, txToVerify, txId + 1, 0);
    }

    function testGCDDoesntConfirmHeader() public {
        relay.setAncestorOverride(false, false);
        vm.expectRevert(bytes("GCD does not confirm header"));
        relay.verifyProof(header, proof, txToVerify, txId, 0);
    }

    function testInsufficientConfirmations() public {
        vm.expectRevert(bytes("Insufficient confirmations"));
        relay.verifyProof(header, proof, txToVerify, txId, 9);
    }

    function testSuccessfullyVerify() public view {
        relay.verifyProof(header, proof, txToVerify, txId, 0);
    }
}

contract FullRelayWithVerifyThroughBitcoinTxTest is FullRelayWithVerifyTest {
    using BitcoinTx for FullRelayWithVerify;
    using BTCUtils for bytes;

    BitcoinTxTester bitcoinTxTester = new BitcoinTxTester();

    uint256 minConfirmations = 1;

    bytes32 expectedTxHash = hex"48e5a1a0e616d8fd92b4ef228c424e0c816799a256c6a90892195ccfc53300d6";
    BitcoinTx.Info txInfoStruct = BitcoinTx.Info({
        version: hex"01000000",
        inputVector: hex"011746bd867400f3494b8f44c24b83e1aa58c4f0ff25b4a61cffeffd4bc0f9ba300000000000ffffffff",
        outputVector: hex"024897070000000000220020a4333e5612ab1a1043b25755c89b16d55184a42f81799e623e6bc39db8539c180000000000000000166a14edb1b5c2f39af0fec151732585b1049b07895211",
        locktime: hex"00000000"
    });

    BitcoinTx.Proof proofStruct = BitcoinTx.Proof({
        merkleProof: proof,
        txIndexInBlock: 281,
        bitcoinHeaders: header,
        coinbasePreimage: hex"77b98a5e6643973bba49dda18a75140306d2d8694b66f2dcb3561ad5aff0b0c7",
        coinbaseProof: hex"dc20dadef477faab2852f2f8ae0c826aa7e05c4de0d36f0e63630429554884c371da5974b6f34fa2c3536738f031b49f34e0c9d084d7280f26212e39007ebe9ea0870c312745b58128a00a6557851e987ece02294d156f0020336e158928e8964292642c6c4dc469f34b7bacf2d8c42115bab6afc9067f2ed30e8749729b63e0889e203ee58e355903c1e71f78c008df6c3597b2cc66d0b8aae1a4a33caa775498e531cfb6af58e87db99e0f536dd226d18f43e3864148ba5b7faca5c775f10bc810c602e1af2195a34577976921ce009a4ddc0a07f605c96b0f5fcf580831ebbe01a31fa29bde884609d286dccfa5ba8e558ce3125bd4c3a19e888cf26852286202d2a7d302c75e0ff5ca8fe7299fb0d9d1132bf2c56c2e3b73df799286193d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64"
    });

    function testSuccessfullyVerify() public view {
        bytes32 txHash = bitcoinTxTester.validateProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct
        );
        assertEq(txHash, expectedTxHash);
    }

    function testIncorrectProofSupplied() public {
        BitcoinTx.Proof memory proofStruct2 = proofStruct;
        proofStruct2.merkleProof = hex"00";
        vm.expectRevert(bytes("Bad merkle array proof"));
        bitcoinTxTester.validateProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct2
        );
    }

    function testIncorrectInputVectorSupplied() public {
        BitcoinTx.Info memory txInfoStruct2 = txInfoStruct;
        txInfoStruct2.inputVector = hex"00";
        vm.expectRevert(bytes("Invalid input vector provided"));
        bitcoinTxTester.validateProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct2, proofStruct
        );
    }

    function testIncorrectOutputVectorSupplied() public {
        BitcoinTx.Info memory txInfoStruct2 = txInfoStruct;
        txInfoStruct2.outputVector = hex"00";
        vm.expectRevert(bytes("Invalid output vector provided"));
        bitcoinTxTester.validateProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct2, proofStruct
        );
    }

    function testIncorrectLocktimeSupplied() public {
        BitcoinTx.Info memory txInfoStruct2 = txInfoStruct;
        txInfoStruct2.locktime = hex"00000001";
        vm.expectRevert(bytes("Tx merkle proof is not valid for provided header and tx hash"));
        bitcoinTxTester.validateProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct2, proofStruct
        );
    }

    function testInvalidHeaderSupplied() public {
        BitcoinTx.Proof memory proofStruct2 = proofStruct;
        // 78 bytes long header
        proofStruct2.bitcoinHeaders =
            hex"0000002073bd2184edd9c4fc76642ea6754ee40136970efc10c4190000000000000000000296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c216349c5ba11928170d3878";
        vm.expectRevert(bytes("Bad header block"));
        bitcoinTxTester.validateProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct2
        );
    }

    function testInvalidMerkleProofSupplied() public {
        BitcoinTx.Proof memory proofStruct2 = proofStruct;
        proofStruct2.merkleProof = hex"00";
        vm.expectRevert(bytes("Bad merkle array proof"));
        bitcoinTxTester.validateProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct2
        );
    }

    function testGCDDoesntConfirmHeader() public {
        relay.setAncestorOverride(false, false);
        vm.expectRevert(bytes("GCD does not confirm header"));
        bitcoinTxTester.validateProof(IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct);
    }

    function testInsufficientConfirmations() public {
        uint256 minConfirmations2 = 9;
        vm.expectRevert(bytes("Insufficient confirmations"));
        bitcoinTxTester.validateProof(
            IFullRelayWithVerify(address(relay)), minConfirmations2, txInfoStruct, proofStruct
        );
    }
}

contract FullRelayWithVerifyThroughWitnessTxTest is FullRelayWithVerifyTest {
    using WitnessTx for FullRelayWithVerify;

    WitnessTxTester witnessTxTester = new WitnessTxTester();

    uint256 minConfirmations = 1;

    WitnessTx.WitnessInfo txInfoStruct = WitnessTx.WitnessInfo({
        info: BitcoinTx.Info({
            version: hex"01000000",
            inputVector: hex"011746bd867400f3494b8f44c24b83e1aa58c4f0ff25b4a61cffeffd4bc0f9ba300000000000ffffffff",
            outputVector: hex"024897070000000000220020a4333e5612ab1a1043b25755c89b16d55184a42f81799e623e6bc39db8539c180000000000000000166a14edb1b5c2f39af0fec151732585b1049b07895211",
            locktime: hex"00000000"
        }),
        witnessVector: hex"024730440220276e0ec78028582054d86614c65bc4bf85ff5710b9d3a248ca28dd311eb2fa6802202ec950dd2a8c9435ff2d400cc45d7a4854ae085f49e05cc3f503834546d410de012103732783eef3af7e04d3af444430a629b16a9261e4025f52bf4d6d026299c37c74"
    });

    bytes32 expectedWTxId = hex"72dab3080783b1b29d6de63724987f9ce0c1980cd1a501f68c1932221580d107";

    WitnessTx.WitnessProof proofStruct = WitnessTx.WitnessProof({
        witnessNonce: hex"0000000000000000000000000000000000000000000000000000000000000000",
        paymentMerkleRoot: hex"9fa45bc4b83457fdac0be52f099ef0fde5050eeeba145e1bf2dfe1d83c9eb615",
        paymentProof: BitcoinTx.Proof({
            bitcoinHeaders: header,
            merkleProof: hex"e35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a31bc4acab4ffe4dcd24084a1878f7317dee840d2d4e205e02ea9fc11607c72e2505d205b4d642eba1c43cead8da1574e0e8a93aa8642b51d5ca43f5214f1ed6eabaf6285d83f460b56fa9dd423882166fde09a8f8eb254066e6a0a4b4c0072160c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c828221415c3515b18a26ef99833ee24daa50652ea01ef021e3752765b6cb4d5a1ed37708d9cd7078665f071123a2c78ecb98eaf3a3434b643a72126e0d3ecd455112cbf3511561e8a0acd78901f1f2d05ad76726fd077e1b9cfd3943046a9295fa",
            txIndexInBlock: 281,
            // We dont need to pass the preimage of the coinbase tx since we verify the coinbase tx directly in validateWitnessProof
            coinbasePreimage: hex"",
            coinbaseProof: hex"dc20dadef477faab2852f2f8ae0c826aa7e05c4de0d36f0e63630429554884c371da5974b6f34fa2c3536738f031b49f34e0c9d084d7280f26212e39007ebe9ea0870c312745b58128a00a6557851e987ece02294d156f0020336e158928e8964292642c6c4dc469f34b7bacf2d8c42115bab6afc9067f2ed30e8749729b63e0889e203ee58e355903c1e71f78c008df6c3597b2cc66d0b8aae1a4a33caa775498e531cfb6af58e87db99e0f536dd226d18f43e3864148ba5b7faca5c775f10bc810c602e1af2195a34577976921ce009a4ddc0a07f605c96b0f5fcf580831ebbe01a31fa29bde884609d286dccfa5ba8e558ce3125bd4c3a19e888cf26852286202d2a7d302c75e0ff5ca8fe7299fb0d9d1132bf2c56c2e3b73df799286193d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64"
        }),
        coinbaseTx: BitcoinTx.Info({
            version: hex"02000000",
            inputVector: hex"010000000000000000000000000000000000000000000000000000000000000000ffffffff4b030443080419349c5b632f4254432e434f4d2ffabe6d6d00e8fb5dfa438482de2e8a272f018a96be1dc3562e8ddf95b75c20f74c02c7ff01000000000000006edce895133d000000000000ffffffff",
            outputVector: hex"03ade4c34a0000000016001497cfc76442fe717f2a3f0cc9c175f7561b6619970000000000000000266a24aa21a9ed6b39cac5af2f7bb8b98ffbfc9954299e7564dec1a78a9ac298a30901a0118e6700000000000000002952534b424c4f434b3af94ade2822a7a3415fe821805c3ed110da3ee58a21ca674534ffe65ee89461d7",
            locktime: hex"00000000"
        })
    });

    function testSuccessfullyVerify() public view {
        bytes32 wTxHash = witnessTxTester.validateWitnessProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct
        );
        assertEq(wTxHash, expectedWTxId);
    }

    function testIncorrectCoinbaseProofSupplied() public {
        WitnessTx.WitnessProof memory proofStruct2 = proofStruct;
        // different final byte
        proofStruct2.paymentProof.coinbaseProof =
            hex"dc20dadef477faab2852f2f8ae0c826aa7e05c4de0d36f0e63630429554884c371da5974b6f34fa2c3536738f031b49f34e0c9d084d7280f26212e39007ebe9ea0870c312745b58128a00a6557851e987ece02294d156f0020336e158928e8964292642c6c4dc469f34b7bacf2d8c42115bab6afc9067f2ed30e8749729b63e0889e203ee58e355903c1e71f78c008df6c3597b2cc66d0b8aae1a4a33caa775498e531cfb6af58e87db99e0f536dd226d18f43e3864148ba5b7faca5c775f10bc810c602e1af2195a34577976921ce009a4ddc0a07f605c96b0f5fcf580831ebbe01a31fa29bde884609d286dccfa5ba8e558ce3125bd4c3a19e888cf26852286202d2a7d302c75e0ff5ca8fe7299fb0d9d1132bf2c56c2e3b73df799286193d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d65";
        vm.expectRevert(bytes("Coinbase merkle proof is not valid for provided header and hash"));
        witnessTxTester.validateWitnessProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct2
        );
    }

    function testIncorrectPaymentProofSupplied() public {
        WitnessTx.WitnessProof memory proofStruct2 = proofStruct;
        // different final byte
        proofStruct2.paymentProof.merkleProof =
            hex"e35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a31bc4acab4ffe4dcd24084a1878f7317dee840d2d4e205e02ea9fc11607c72e2505d205b4d642eba1c43cead8da1574e0e8a93aa8642b51d5ca43f5214f1ed6eabaf6285d83f460b56fa9dd423882166fde09a8f8eb254066e6a0a4b4c0072160c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c828221415c3515b18a26ef99833ee24daa50652ea01ef021e3752765b6cb4d5a1ed37708d9cd7078665f071123a2c78ecb98eaf3a3434b643a72126e0d3ecd455112cbf3511561e8a0acd78901f1f2d05ad76726fd077e1b9cfd3943046a9295fb";
        vm.expectRevert(bytes("Tx witness merkle proof is not valid for provided header and tx hash"));
        witnessTxTester.validateWitnessProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct2
        );
    }

    function testInconsistentProofLengths() public {
        WitnessTx.WitnessProof memory proofStruct2 = proofStruct;
        proofStruct2.paymentProof.merkleProof = hex"00";
        vm.expectRevert(bytes("Tx not on same level of merkle tree as coinbase"));
        witnessTxTester.validateWitnessProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct2
        );
    }

    function testGCDDoesntConfirmHeader() public {
        relay.setAncestorOverride(false, false);
        vm.expectRevert(bytes("GCD does not confirm header"));
        witnessTxTester.validateWitnessProof(
            IFullRelayWithVerify(address(relay)), minConfirmations, txInfoStruct, proofStruct
        );
    }

    function testInsufficientConfirmations() public {
        uint256 minConfirmations2 = 9;
        vm.expectRevert(bytes("Insufficient confirmations"));
        witnessTxTester.validateWitnessProof(
            IFullRelayWithVerify(address(relay)), minConfirmations2, txInfoStruct, proofStruct
        );
    }
}
