# BobWrappedBtc
[Git Source](https://github.com/bob-collective/bob/blob/82f2904bc9683a0c36a15ec6e164256dd25fd4c2/src/swap/Wrapped.sol)

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

