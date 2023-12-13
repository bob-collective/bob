# MarketPlace
[Git Source](https://github.com/bob-collective/bob/blob/9fd4522721442ac5e04e105bccf23b16c8ad31a6/src/swap/Marketplace.sol)


## State Variables
### ercErcOrders

```solidity
mapping(uint256 => Order) public ercErcOrders;
```


### nextOrderId

```solidity
uint256 public nextOrderId;
```


## Functions
### placeErcErcOrder


```solidity
function placeErcErcOrder(address sellingToken, uint256 saleAmount, address buyingToken, uint256 buyAmount) public;
```

### acceptErcErcOrder


```solidity
function acceptErcErcOrder(uint256 id, uint256 saleAmount) public;
```

### withdrawErcErcOrder


```solidity
function withdrawErcErcOrder(uint256 id) public;
```

### getOpenOrders


```solidity
function getOpenOrders() external view returns (Order[] memory);
```

## Events
### placeOrder

```solidity
event placeOrder(
    uint256 indexed orderId,
    address indexed requesterAddress,
    uint256 offeringAmount,
    address offeringToken,
    uint256 askingAmount,
    address askingToken
);
```

### acceptOrder

```solidity
event acceptOrder(uint256 indexed orderId, address indexed who, uint256 buyAmount, uint256 saleAmount);
```

### withdrawOrder

```solidity
event withdrawOrder(uint256 indexed orderId);
```

## Structs
### Order

```solidity
struct Order {
    uint256 offeringAmount;
    address offeringToken;
    uint256 askingAmount;
    address askingToken;
    address requesterAddress;
}
```

