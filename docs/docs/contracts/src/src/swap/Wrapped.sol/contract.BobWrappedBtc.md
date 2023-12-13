# BobWrappedBtc
[Git Source](https://github.com/bob-collective/bob/blob/d9c9196f0c99ad631c4c8411f2d25decea2e634f/src/swap/Wrapped.sol)

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

