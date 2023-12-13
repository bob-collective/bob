# BobWrappedBtc
[Git Source](https://github.com/bob-collective/bob/blob/9c01c7cfc14aec17ca362111dc5fef421675ccf1/src/swap/Wrapped.sol)

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

