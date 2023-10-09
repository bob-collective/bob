pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

using SafeERC20 for IERC20;

contract BtcMarketPlace {
    mapping(uint256 => BtcBuyOrder) public btcBuyOrders;
    mapping(uint256 => AcceptedBtcBuyOrder) public acceptedBtcBuyOrders;
    mapping(uint256 => BtcSellOrder) public btcSellOrders;
    mapping(uint256 => AcceptedBtcSellOrder) public acceptedBtcSellOrders;

    uint256 nextOrderId;
    uint256 public constant REQUEST_EXPIRATION_SECONDS = 86400; // 1 day

    // todo: should we merge buy&sell structs? They're structurally identical except for the
    // bitcoinaddress location.

    event placeBtcSellOrderEvent(
        uint indexed orderId,
        uint256 amountBtc,
        address buyingToken,
        uint buyAmount
    );
    event acceptBtcSellOrderEvent(
        uint indexed id,
        uint indexed acceptId,
        BitcoinAddress bitcoinAddress,
        uint256 amountBtc,
        uint256 ercAmount,
        address ercToken
    );
    event proofBtcSellOrderEvent(uint id, TransactionProof _proof);
    event withdrawBtcSellOrderEvent(uint id);
    event cancelAcceptedBtcSellOrderEvent(uint id);
    event placeBtcBuyOrderEvent(
        uint256 amountBtc,
        BitcoinAddress bitcoinAddress,
        address sellingToken,
        uint saleAmount
    );
    event acceptBtcBuyOrderEvent(
        uint indexed orderId,
        uint indexed acceptId,
        uint256 amountBtc,
        uint256 ercAmount,
        address ercToken
    );
    event proofBtcBuyOrderEvent(uint id, TransactionProof _proof);
    event withdrawBtcBuyOrderEvent(uint id);
    event cancelAcceptedBtcBuyOrderEvent(uint id);

    struct BtcSellOrder {
        uint256 amountBtc;
        address askingToken;
        uint askingAmount;
        address requester;
    }

    struct AcceptedBtcSellOrder {
        uint orderId;
        BitcoinAddress bitcoinAddress;
        uint256 amountBtc;
        address ercToken;
        uint ercAmount;
        address requester;
        address accepter;
        uint acceptTime;
    }

    struct BtcBuyOrder {
        uint256 amountBtc;
        BitcoinAddress bitcoinAddress;
        address offeringToken;
        uint offeringAmount;
        address requester;
    }

    struct AcceptedBtcBuyOrder {
        uint orderId;
        uint256 amountBtc;
        address ercToken;
        uint ercAmount;
        address requester;
        address accepter;
        uint acceptTime;
    }

    struct BitcoinAddress {
        uint256 bitcoinAddress; // todo: use the right type
    }
    struct TransactionProof {
        // todo: fields here
        uint256 dummy;
    }

    function placeBtcSellOrder(
        uint256 amountBtc,
        address buyingToken,
        uint buyAmount
    ) public {
        require(buyingToken != address(0x0));
        require(amountBtc > 0);
        require(buyAmount > 0);

        uint256 id = nextOrderId++;
        btcSellOrders[id] = BtcSellOrder({
            amountBtc: amountBtc,
            askingToken: buyingToken,
            askingAmount: buyAmount,
            requester: msg.sender
        });

        emit placeBtcSellOrderEvent(id, amountBtc, buyingToken, buyAmount);
    }

    function acceptBtcSellOrder(
        uint id,
        BitcoinAddress calldata bitcoinAddress,
        uint256 amountBtc
    ) public returns (uint) {
        BtcSellOrder storage order = btcSellOrders[id];

        require(amountBtc > 0);
        require(amountBtc <= order.amountBtc);

        uint sellAmount = (amountBtc * order.askingAmount) / order.amountBtc;
        assert(sellAmount > 0);
        assert(order.askingAmount >= sellAmount);
        order.askingAmount -= sellAmount;
        order.amountBtc -= amountBtc;

        // "lock" selling token by transferring to contract
        IERC20(order.askingToken).safeTransferFrom(
            msg.sender,
            address(this),
            sellAmount
        );

        uint256 acceptId = nextOrderId++;
        acceptedBtcSellOrders[acceptId] = AcceptedBtcSellOrder({
            orderId: id,
            bitcoinAddress: bitcoinAddress,
            amountBtc: amountBtc,
            ercToken: order.askingToken,
            ercAmount: sellAmount,
            requester: order.requester,
            accepter: msg.sender,
            acceptTime: block.timestamp
        });

        emit acceptBtcSellOrderEvent(
            id,
            acceptId,
            bitcoinAddress,
            amountBtc,
            sellAmount,
            order.askingToken
        );

        return acceptId;
    }

    function proofBtcSellOrder(
        uint id,
        TransactionProof calldata _proof
    ) public {
        AcceptedBtcSellOrder storage accept = acceptedBtcSellOrders[id];

        require(accept.requester == msg.sender);

        // todo: check proof

        IERC20(accept.ercToken).safeTransfer(
            accept.requester,
            accept.ercAmount
        );

        delete acceptedBtcSellOrders[id];
        emit proofBtcSellOrderEvent(id, _proof);
    }

    function withdrawBtcSellOrder(uint id) public {
        BtcSellOrder storage order = btcSellOrders[id];

        require(order.requester == msg.sender);

        delete btcSellOrders[id];

        emit withdrawBtcSellOrderEvent(id);
    }

    function cancelAcceptedBtcSellOrder(uint id) public {
        AcceptedBtcSellOrder storage order = acceptedBtcSellOrders[id];

        require(
            block.timestamp > order.acceptTime + REQUEST_EXPIRATION_SECONDS
        );

        require(order.accepter == msg.sender);
        // give accepter its tokens back
        IERC20(order.ercToken).safeTransfer(msg.sender, order.ercAmount);

        delete acceptedBtcSellOrders[id];

        emit cancelAcceptedBtcSellOrderEvent(id);
    }

    function placeBtcBuyOrder(
        uint256 amountBtc,
        BitcoinAddress calldata bitcoinAddress,
        address sellingToken,
        uint saleAmount
    ) public {
        require(sellingToken != address(0x0));

        // "lock" selling token by transferring to contract
        IERC20(sellingToken).safeTransferFrom(
            msg.sender,
            address(this),
            saleAmount
        );

        uint256 id = nextOrderId++;
        btcBuyOrders[id] = BtcBuyOrder({
            amountBtc: amountBtc,
            bitcoinAddress: bitcoinAddress,
            offeringToken: sellingToken,
            offeringAmount: saleAmount,
            requester: msg.sender
        });

        emit placeBtcBuyOrderEvent(
            amountBtc,
            bitcoinAddress,
            sellingToken,
            saleAmount
        );
    }

    function acceptBtcBuyOrder(
        uint id,
        uint256 amountBtc
    ) public returns (uint) {
        BtcBuyOrder storage order = btcBuyOrders[id];

        require(amountBtc <= order.amountBtc);
        require(amountBtc > 0);

        // todo: make safe
        uint buyAmount = (amountBtc * order.offeringAmount) / order.amountBtc;

        assert(buyAmount > 0);

        assert(order.offeringAmount >= buyAmount);
        order.offeringAmount -= buyAmount;
        order.amountBtc -= amountBtc;

        AcceptedBtcBuyOrder memory accept = AcceptedBtcBuyOrder({
            orderId: id,
            amountBtc: amountBtc,
            ercToken: order.offeringToken,
            ercAmount: buyAmount,
            requester: order.requester,
            accepter: msg.sender,
            acceptTime: block.timestamp
        });

        uint acceptId = nextOrderId++;

        acceptedBtcBuyOrders[acceptId] = accept;

        emit acceptBtcBuyOrderEvent(
            id,
            acceptId,
            amountBtc,
            buyAmount,
            order.offeringToken
        );

        return acceptId;
    }

    function proofBtcBuyOrder(
        uint id,
        TransactionProof calldata _proof
    ) public {
        AcceptedBtcBuyOrder storage accept = acceptedBtcBuyOrders[id];

        require(accept.accepter == msg.sender);

        // todo: check proof

        IERC20(accept.ercToken).safeTransfer(accept.accepter, accept.ercAmount);

        delete acceptedBtcBuyOrders[id];

        emit proofBtcBuyOrderEvent(id, _proof);
    }

    function withdrawBtcBuyOrder(uint id) public {
        BtcBuyOrder storage order = btcBuyOrders[id];

        require(order.requester == msg.sender);

        // release the locked erc20s
        IERC20(order.offeringToken).safeTransfer(
            msg.sender,
            order.offeringAmount
        );

        delete btcBuyOrders[id];

        emit withdrawBtcBuyOrderEvent(id);
    }

    function cancelAcceptedBtcBuyOrder(uint id) public {
        AcceptedBtcBuyOrder storage accept = acceptedBtcBuyOrders[id];

        require(accept.requester == msg.sender);

        require(
            block.timestamp > accept.acceptTime + REQUEST_EXPIRATION_SECONDS
        );

        // release the locked erc20s
        IERC20(accept.ercToken).safeTransfer(msg.sender, accept.ercAmount);

        // note: we don't make the accepted amount available for new trades but if we want to,
        // we could implement that

        delete acceptedBtcBuyOrders[id];

        emit cancelAcceptedBtcBuyOrderEvent(id);
    }

    function getOpenBtcSellOrders()
        external
        view
        returns (BtcSellOrder[] memory, uint[] memory)
    {
        uint numOpenOrders = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (btcSellOrders[i].requester != address(0x0)) {
                numOpenOrders++;
            }
        }

        BtcSellOrder[] memory ret = new BtcSellOrder[](numOpenOrders);
        uint[] memory identifiers = new uint[](numOpenOrders);
        uint numPushed = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (btcSellOrders[i].requester != address(0x0)) {
                ret[numPushed] = btcSellOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }

    function getOpenAcceptedBtcSellOrders()
        external
        view
        returns (AcceptedBtcSellOrder[] memory, uint[] memory)
    {
        uint numOpenOrders = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (acceptedBtcSellOrders[i].amountBtc > 0) {
                numOpenOrders++;
            }
        }

        AcceptedBtcSellOrder[] memory ret = new AcceptedBtcSellOrder[](
            numOpenOrders
        );
        uint[] memory identifiers = new uint[](numOpenOrders);
        uint numPushed = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (acceptedBtcSellOrders[i].amountBtc > 0) {
                ret[numPushed] = acceptedBtcSellOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }

    function getOpenBtcBuyOrders()
        external
        view
        returns (BtcBuyOrder[] memory, uint[] memory)
    {
        uint numOpenOrders = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (btcBuyOrders[i].requester != address(0x0)) {
                numOpenOrders++;
            }
        }

        BtcBuyOrder[] memory ret = new BtcBuyOrder[](numOpenOrders);
        uint[] memory identifiers = new uint[](numOpenOrders);
        uint numPushed = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (btcBuyOrders[i].requester != address(0x0)) {
                ret[numPushed] = btcBuyOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }

    function getOpenAcceptedBtcBuyOrders()
        external
        view
        returns (AcceptedBtcBuyOrder[] memory, uint[] memory)
    {
        uint numOpenOrders = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (acceptedBtcBuyOrders[i].amountBtc > 0) {
                numOpenOrders++;
            }
        }

        AcceptedBtcBuyOrder[] memory ret = new AcceptedBtcBuyOrder[](
            numOpenOrders
        );
        uint[] memory identifiers = new uint[](numOpenOrders);
        uint numPushed = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (acceptedBtcBuyOrders[i].amountBtc > 0) {
                ret[numPushed] = acceptedBtcBuyOrders[i];
                identifiers[numPushed] = i;
                numPushed++;
            }
        }
        return (ret, identifiers);
    }
}
