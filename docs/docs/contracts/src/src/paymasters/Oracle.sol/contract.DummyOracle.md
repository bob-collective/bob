# DummyOracle
[Git Source](https://github.com/bob-collective/bob/blob/9dd94230dd2abcab7dfb659e986743be10093c68/src/paymasters/Oracle.sol)

**Inherits:**
[IOracle](../../paymasters/Oracle.sol/interface.IOracle.md)


## State Variables
### price

```solidity
int256 public price;
```


## Functions
### decimals


```solidity
function decimals() external view returns (uint8);
```

### latestRoundData


```solidity
function latestRoundData()
    external
    view
    returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound);
```

### setPrice


```solidity
function setPrice(int256 _price) public;
```

