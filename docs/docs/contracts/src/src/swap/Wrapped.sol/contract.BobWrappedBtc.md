# BobWrappedBtc
[Git Source](https://github.com/bob-collective/bob/blob/6feef4b26921a0e62c67cac7076c04271613ba33/src/swap/Wrapped.sol)

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

