pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

using SafeERC20 for IERC20;

contract MarketPlace {
    mapping(uint256 => BtcBuyOrder) public btcBuyOrders;
    mapping(uint256 => AcceptedBtcBuyOrder) public acceptedBtcBuyOrders;
    mapping(uint256 => BtcSellOrder) public btcSellOrders;
    mapping(uint256 => AcceptedBtcSellOrder) public acceptedBtcSellOrders;

    uint256 nextOrderId;

    struct BtcSellOrder {
        uint256 amountBtc;
        address askingToken;
        uint askingAmount;
        address requester;
    }

    struct AcceptedBtcSellOrder {
        BitcoinAddress bitcoinAddress;
        uint256 amountBtc;
        address ercToken;
        uint ercAmount;
        address requester;
        address accepter;
    }

    struct BtcBuyOrder {
        uint256 amountBtc;
        BitcoinAddress bitcoinAddress;
        address offeringToken;
        uint offeringAmount;
        address requester;
    }

    struct AcceptedBtcBuyOrder {
        uint256 amountBtc;
        address ercToken;
        uint ercAmount;
        address requester;
        address accepter;
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
    }

    function acceptBtcSellOrder(
        uint id,
        BitcoinAddress calldata bitcoinAddress,
        uint256 amountBtc
    ) public {
        BtcSellOrder storage order = btcSellOrders[id];

        require(amountBtc > 0);
        require(amountBtc <= order.amountBtc);

        uint sellAmount = (amountBtc * order.askingAmount) / order.amountBtc;
        assert(sellAmount > 0);

        // "lock" selling token by transferring to contract
        IERC20(order.askingToken).safeTransferFrom(
            msg.sender,
            address(this),
            sellAmount
        );

        uint256 id = nextOrderId++;
        acceptedBtcSellOrders[id] = AcceptedBtcSellOrder({
            bitcoinAddress: bitcoinAddress,
            amountBtc: amountBtc,
            ercToken: order.askingToken,
            ercAmount: sellAmount,
            requester: order.requester,
            accepter: msg.sender
        });
    }


    function proofBtcSellOrder(
        uint id,
        TransactionProof calldata _proof
    ) public {
        AcceptedBtcSellOrder storage accept = acceptedBtcSellOrders[id];

        require(accept.requester == msg.sender);

        // todo: check proof

        IERC20(accept.ercToken).safeTransfer(accept.requester, accept.ercAmount);

        delete acceptedBtcSellOrders[id];
    }

    function withdrawBtcSellOrder(uint id) public {
        BtcSellOrder storage order = btcSellOrders[id];

        require(order.requester == msg.sender);

        delete btcBuyOrders[id];
    }

    function cancelAcceptedBtcSellOrder(uint id) public {
        AcceptedBtcSellOrder storage order = acceptedBtcSellOrders[id];

        // todo: check that it's expired

        require(order.accepter == msg.sender);

        delete btcBuyOrders[id];
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
    }

    function acceptBtcBuyOrder(uint id, uint256 amountBtc) public {
        BtcBuyOrder storage order = btcBuyOrders[id];

        require(amountBtc <= order.offeringAmount);
        require(amountBtc > 0);

        // todo: make safe
        uint buyAmount = (amountBtc * order.offeringAmount) / order.amountBtc;

        assert(buyAmount > 0);

        assert(order.amountBtc >= buyAmount);
        order.amountBtc -= buyAmount;

        AcceptedBtcBuyOrder memory accept = AcceptedBtcBuyOrder({
            amountBtc: amountBtc,
            ercToken: order.offeringToken,
            ercAmount: buyAmount,
            requester: order.requester,
            accepter: msg.sender
        });

        uint acceptId = nextOrderId++;

        acceptedBtcBuyOrders[acceptId] = accept;
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
    }

    function cancelAcceptedBtcBuyOrder(uint id) public {
        AcceptedBtcBuyOrder storage accept = acceptedBtcBuyOrders[id];

        require(accept.requester == msg.sender);

        // todo: check that it's expired

        // release the locked erc20s
        IERC20(accept.ercToken).safeTransfer(msg.sender, accept.ercAmount);
    }
}
