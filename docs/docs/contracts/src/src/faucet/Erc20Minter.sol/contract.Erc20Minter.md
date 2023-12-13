# Erc20Minter
[Git Source](https://github.com/bob-collective/bob/blob/d9c9196f0c99ad631c4c8411f2d25decea2e634f/src/faucet/Erc20Minter.sol)

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

