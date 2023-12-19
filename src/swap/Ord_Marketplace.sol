// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {BitcoinTx} from "../bridge/BitcoinTx.sol";
import {IRelay} from "../bridge/IRelay.sol";
import {BridgeState} from "../bridge/BridgeState.sol";
import "forge-std/console.sol";

using SafeERC20 for IERC20;

contract OrdMarketplace {
    using BitcoinTx for BridgeState.Storage;

    mapping(uint256 => OrdinalSellOrder) public ordinalSellOrders;
    mapping(uint256 => AcceptedOrdinalSellOrder) public acceptedOrdinalSellOrders;

    uint256 nextOrdinalId;
    uint256 public constant REQUEST_EXPIRATION_SECONDS = 6 hours;

    BridgeState.Storage internal relay;

    constructor(IRelay _relay) {
        relay.relay = _relay;
        relay.txProofDifficultyFactor = 1; // will make this an arg later on
    }

    function setRelay(IRelay _relay) internal {
        relay.relay = _relay;
    }

    event placeOrdinalSellOrderEvent(
        uint256 indexed orderId, OrdinalId ordinalID, address sellToken, uint256 sellAmount
    );
    event acceptOrdinalSellOrderEvent(
        uint256 indexed id, uint256 indexed acceptId, BitcoinAddress bitcoinAddress, address ercToken, uint256 ercAmount
    );
    event proofOrdinalSellOrderEvent(uint256 id);
    event withdrawOrdinalSellOrderEvent(uint256 id);
    event cancelAcceptedOrdinalSellOrderEvent(uint256 id);

    struct OrdinalSellOrder {
        OrdinalId ordinalID;
        address sellToken;
        uint256 sellAmount;
        BitcoinTx.UTXO utxo;
        address requester;
        bool isOrderAccepted;
    }

    struct AcceptedOrdinalSellOrder {
        uint256 orderId;
        BitcoinAddress bitcoinAddress;
        address ercToken;
        uint256 ercAmount;
        address requester;
        address acceptor;
        uint256 acceptTime;
    }

    struct BitcoinAddress {
        bytes scriptPubKey;
    }

    struct OrdinalId {
        bytes32 txId;
        uint32 index;
    }

    function placeOrdinalSellOrder(
        OrdinalId calldata ordinalID,
        BitcoinTx.UTXO calldata utxo,
        address sellToken,
        uint256 sellAmount
    ) public {
        require(sellToken != address(0x0), "Invalid buying token");
        require(sellAmount > 0, "Buying amount should be greater than 0");

        uint256 id = nextOrdinalId++;

        ordinalSellOrders[id] = OrdinalSellOrder({
            ordinalID: ordinalID,
            sellToken: sellToken,
            sellAmount: sellAmount,
            utxo: utxo,
            requester: msg.sender,
            isOrderAccepted: false
        });

        emit placeOrdinalSellOrderEvent(id, ordinalID, sellToken, sellAmount);
    }

    function acceptOrdinalSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress) public returns (uint256) {
        OrdinalSellOrder storage order = ordinalSellOrders[id];
        require(order.isOrderAccepted == false, "Order Already Accepted");
        // "lock" sell token by transferring to contract
        IERC20(order.sellToken).safeTransferFrom(msg.sender, address(this), order.sellAmount);

        uint256 acceptId = nextOrdinalId++;

        acceptedOrdinalSellOrders[acceptId] = AcceptedOrdinalSellOrder({
            orderId: id,
            bitcoinAddress: bitcoinAddress,
            ercToken: order.sellToken,
            ercAmount: order.sellAmount,
            requester: order.requester,
            acceptor: msg.sender,
            acceptTime: block.timestamp
        });

        order.isOrderAccepted = true;

        emit acceptOrdinalSellOrderEvent(id, acceptId, bitcoinAddress, order.sellToken, order.sellAmount);

        return acceptId;
    }

    //    function reverseEndianness(bytes32 b) public returns (bytes32 result){
    ////        bytes32 memory _newValue = new bytes(_b.length);
    //        bytes memory newValue = new bytes(b.length);
    //        for (uint i = 0; i < b.length; i++) {
    //            console.log("i: ",i);
    //            console.log("b.length: ",i);
    //            newValue[b.length - i - 1] = b[i];
    //        }
    //        // Casting bytes to bytes32
    //        assembly {
    //            result := mload(add(newValue, 32))
    //        }
    //    }

    function proofOrdinalSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
        public
    {
        AcceptedOrdinalSellOrder storage accept = acceptedOrdinalSellOrders[id];
        require(accept.requester == msg.sender, "Sender not the requester");

        OrdinalSellOrder storage order = ordinalSellOrders[accept.orderId];

        relay.validateProof(transaction, proof);

        BitcoinTx.ensureTxInputSpendsUtxo(transaction.inputVector, order.utxo);

        //        bytes32 expectedTxHash = reverseEndianness(order.utxo.txHash);
        //
        //        uint256 _varIntDataLen;
        //        uint256 _nIns;
        //        bytes memory _vin = transaction.inputVector;
        //        BitcoinTx.UTXO memory utxo = order.utxo;
        //        (_varIntDataLen, _nIns) = BTCUtils.parseVarInt(_vin);
        //        require(_varIntDataLen != BTCUtils.ERR_BAD_ARG, "Read overrun during VarInt parsing");
        //
        //        uint256 _len = 0;
        //        uint256 _offset = 1 + _varIntDataLen;
        //
        //        for (uint256 i = 0; i < _nIns; i++) {
        //            bytes32 outpointTxHash = BTCUtils.extractInputTxIdLeAt(_vin,_offset);
        //            uint32 outpointIndex = BTCUtils.reverseUint32(uint32(BTCUtils.extractTxIndexLeAt(_vin,_offset)));
        //
        //            console.logBytes32(outpointTxHash);
        //            console.logBytes32(expectedTxHash);
        //            console.log(order.utxo.txOutputIndex);
        //            console.log(outpointIndex);
        //            // check if it matches tx
        //            if (expectedTxHash == outpointTxHash && order.utxo.txOutputIndex == outpointIndex) {
        //                revert(" ---------- Match ----------");
        //            }
        //
        //            _len = BTCUtils.determineInputLengthAt(_vin, _offset);
        //            require(_len != BTCUtils.ERR_BAD_ARG, "Bad VarInt in scriptSig");
        //            _offset = _offset + _len;
        //
        //        }

        // check if output to the buyer's address
        _checkBitcoinTxOutput(accept.bitcoinAddress, transaction);

        // ToDo: check that the correct satoshi is being spent to the buyer's address

        IERC20(accept.ercToken).safeTransfer(accept.requester, accept.ercAmount);

        delete ordinalSellOrders[accept.orderId];
        delete acceptedOrdinalSellOrders[id];
        emit proofOrdinalSellOrderEvent(id);
    }

    /**
     * Checks output script pubkey (recipient address) matches output script.
     * Reverts if bitcoin address is not found.
     *
     * @param bitcoinAddress Recipient's bitcoin address.
     * @param transaction Transaction fulfilling the order.
     */
    function _checkBitcoinTxOutput(BitcoinAddress storage bitcoinAddress, BitcoinTx.Info calldata transaction)
        private
    {
        // Prefixes scriptpubkey with its size to match script output data.
        bytes32 scriptPubKeyHash =
            keccak256(abi.encodePacked(uint8(bitcoinAddress.scriptPubKey.length), bitcoinAddress.scriptPubKey));

        // it will revert if no match for scriptPubKeyHash found in any outputScriptHash
        BitcoinTx.getTxOutputValue(scriptPubKeyHash, transaction.outputVector);
    }

    function withdrawOrdinalSellOrder(uint256 id) public {
        OrdinalSellOrder storage order = ordinalSellOrders[id];

        require(order.requester == msg.sender, "Sender not the requester");
        require(order.isOrderAccepted == false, "Order has already been accepted, cannot withdraw");

        delete ordinalSellOrders[id];

        emit withdrawOrdinalSellOrderEvent(id);
    }

    function cancelAcceptedOrdinalSellOrder(uint256 id) public {
        AcceptedOrdinalSellOrder storage order = acceptedOrdinalSellOrders[id];

        require(block.timestamp > order.acceptTime + REQUEST_EXPIRATION_SECONDS, "Request still valid");

        require(order.acceptor == msg.sender, "Sender not the acceptor");

        // give acceptor its tokens back
        IERC20(order.ercToken).safeTransfer(msg.sender, order.ercAmount);

        delete acceptedOrdinalSellOrders[id];

        emit cancelAcceptedOrdinalSellOrderEvent(id);
    }

    function getOpenOrdinalSellOrders() external view returns (OrdinalSellOrder[] memory, uint256[] memory) {
        uint256 numOpenOrders = 0;
        for (uint256 i = 0; i < nextOrdinalId; i++) {
            if (ordinalSellOrders[i].requester != address(0x0)) {
                numOpenOrders++;
            }
        }

        OrdinalSellOrder[] memory ret = new OrdinalSellOrder[](numOpenOrders);
        uint256[] memory identifiers = new uint256[](numOpenOrders);
        uint256 numPushed = 0;
        for (uint256 i = 0; i < nextOrdinalId; i++) {
            if (ordinalSellOrders[i].requester != address(0x0)) {
                ret[numPushed] = ordinalSellOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }

    function getOpenAcceptedOrdinalSellOrders()
        external
        view
        returns (AcceptedOrdinalSellOrder[] memory, uint256[] memory)
    {
        uint256 numOpenOrders = 0;
        for (uint256 i = 0; i < nextOrdinalId; i++) {
            if (acceptedOrdinalSellOrders[i].ercAmount > 0) {
                numOpenOrders++;
            }
        }

        AcceptedOrdinalSellOrder[] memory ret = new AcceptedOrdinalSellOrder[](numOpenOrders);
        uint256[] memory identifiers = new uint256[](numOpenOrders);
        uint256 numPushed = 0;
        for (uint256 i = 0; i < nextOrdinalId; i++) {
            if (acceptedOrdinalSellOrders[i].ercAmount > 0) {
                ret[numPushed] = acceptedOrdinalSellOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }
}
