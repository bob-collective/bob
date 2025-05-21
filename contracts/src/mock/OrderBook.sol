pragma solidity ^0.8.17;

contract OrderBook {
    uint256 public number;

    event SubmitOrder(uint256 orderId, address sender, uint256 amount);

    function submitOrder(uint256 amount) public {
        emit SubmitOrder(block.number, msg.sender, amount);
    }
}
