pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

using SafeERC20 for IERC20;

contract MarketPlace {
    mapping(uint256 => Order) public ercErcOrders; // cant have struct as key, nor tupple

    event placeOrder(uint indexed orderId, Order order); // todo: should we index by requesteraddress?
    event acceptOrder(
        uint indexed orderId,
        address who,
        uint buyAmount,
        uint saleAmount
    );
    event withdrawOrder(uint indexed orderId);

    uint256 nextOrderId;

    struct Order {
        uint256 offeringAmount;
        address offeringToken;
        uint256 askingAmount;
        address askingToken;
        address requesterAddress;
    }

    function placeErcErcOrder(
        address sellingToken,
        uint saleAmount,
        address buyingToken,
        uint buyAmount
    ) public {
        require(sellingToken != address(0x0));
        require(buyingToken != address(0x0));

        // "lock" selling token by transferring to contract
        IERC20(sellingToken).safeTransferFrom(
            msg.sender,
            address(this),
            saleAmount
        );

        uint id = nextOrderId++;
        Order memory order = Order({
            offeringAmount: saleAmount,
            offeringToken: sellingToken,
            askingAmount: buyAmount,
            askingToken: buyingToken,
            requesterAddress: msg.sender
        });

        ercErcOrders[id] = order;

        emit placeOrder(id, order);
    }

    function acceptErcErcOrder(uint id, uint saleAmount) public {
        Order memory order = ercErcOrders[id];

        // make sure the storage item exists
        require(order.requesterAddress != address(0x0));

        require(saleAmount <= order.askingAmount);
        // todo: make this safe
        uint buyAmount = (saleAmount * order.offeringAmount) /
            order.askingAmount;

        assert(saleAmount > 0);
        assert(buyAmount > 0);
        assert(buyAmount <= order.offeringAmount);

        ercErcOrders[id].offeringAmount -= buyAmount;
        ercErcOrders[id].askingAmount -= saleAmount;

        IERC20(order.askingToken).safeTransferFrom(
            msg.sender,
            order.requesterAddress,
            saleAmount
        );
        IERC20(order.offeringToken).safeTransfer(msg.sender, buyAmount);

        emit acceptOrder(id, msg.sender, buyAmount, saleAmount);
    }

    function withdrawErcErcOrder(uint id) public {
        Order memory order = ercErcOrders[id];
        require(order.requesterAddress == msg.sender);

        IERC20(order.offeringToken).safeTransfer(
            msg.sender,
            order.offeringAmount
        );

        delete ercErcOrders[id];

        emit withdrawOrder(id);
    }

    function getOpenOrders() public view returns (Order[] memory) {
        uint numOpenOrders = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (ercErcOrders[i].offeringAmount > 0) {
                numOpenOrders++;
            }
        }
        Order[] memory ret = new Order[](numOpenOrders);
        uint numPushed = 0;
        for (uint i = 0; i < nextOrderId; i++) {
            if (ercErcOrders[i].offeringAmount > 0) {
                ret[numPushed] = ercErcOrders[i];
                numPushed++;
            }
        }
        return ret;
    }
}
