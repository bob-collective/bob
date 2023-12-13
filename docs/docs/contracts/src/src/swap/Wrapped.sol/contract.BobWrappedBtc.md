# BobWrappedBtc
[Git Source](https://github.com/bob-collective/bob/blob/8c3fe55e3293351a02cff493e52a7ca562403658/src/swap/Wrapped.sol)

**Inherits:**
ERC20, ERC20Burnable, Ownable


## Functions
### constructor


```solidity
constructor() ERC20("Bob Wrapped BTC", "zBTC");
```

### sudoMint


```solidity
function sudoMint(address to, uint256 amount) public onlyOwner;
```

### sudoBurnFrom


```solidity
function sudoBurnFrom(address account, uint256 amount) public onlyOwner;
```

### sudoTransferFrom


```solidity
function sudoTransferFrom(address from, address to, uint256 amount) public onlyOwner;
```

