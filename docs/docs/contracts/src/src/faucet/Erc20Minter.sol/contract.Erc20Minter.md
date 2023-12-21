# Erc20Minter
[Git Source](https://github.com/bob-collective/bob/blob/82f2904bc9683a0c36a15ec6e164256dd25fd4c2/src/faucet/Erc20Minter.sol)

**Inherits:**
Ownable


## State Variables
### nextTokenId

```solidity
uint256 nextTokenId;
```


### supportedErc20Addresses

```solidity
mapping(uint256 => address) supportedErc20Addresses;
```


## Functions
### addErc20


```solidity
function addErc20(address newErc20) public onlyOwner;
```

### mint


```solidity
function mint() public;
```

