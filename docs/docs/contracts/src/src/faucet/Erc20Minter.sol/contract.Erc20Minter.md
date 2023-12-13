# Erc20Minter
[Git Source](https://github.com/bob-collective/bob/blob/8c3fe55e3293351a02cff493e52a7ca562403658/src/faucet/Erc20Minter.sol)

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

