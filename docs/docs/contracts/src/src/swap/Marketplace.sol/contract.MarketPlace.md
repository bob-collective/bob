# MarketPlace
[Git Source](https://github.com/bob-collective/bob/blob/b2d54e5c3996ef2181be170d263691c7d860e253/src/swap/Marketplace.sol)


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

