# DummyOracle
[Git Source](https://github.com/bob-collective/bob/blob/82f2904bc9683a0c36a15ec6e164256dd25fd4c2/src/paymasters/Oracle.sol)

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

