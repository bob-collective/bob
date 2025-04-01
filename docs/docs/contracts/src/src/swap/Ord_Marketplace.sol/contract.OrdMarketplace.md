# OrdMarketplace
[Git Source](https://github.com/bob-collective/bob/blob/1abe7d0a95cbaa62e47217036600733eae5f19f9/src/swap/Ord_Marketplace.sol)


## State Variables
### ordinalSellOrders

```solidity
mapping(uint256 => OrdinalSellOrder) public ordinalSellOrders;
```


### acceptedOrdinalSellOrders

```solidity
mapping(uint256 => AcceptedOrdinalSellOrder) public acceptedOrdinalSellOrders;
```


### nextOrdinalId

```solidity
uint256 nextOrdinalId;
```


### REQUEST_EXPIRATION_SECONDS

```solidity
uint256 public constant REQUEST_EXPIRATION_SECONDS = 6 hours;
```


### testLightRelay

```solidity
TestLightRelay internal testLightRelay;
```


### txProofDifficultyFactor

```solidity
uint256 internal txProofDifficultyFactor;
```


## Functions
### constructor


```solidity
constructor(TestLightRelay _relay);
```

### setRelay


```solidity
function setRelay(TestLightRelay _relay) internal;
```

### placeOrdinalSellOrder


```solidity
function placeOrdinalSellOrder(
    OrdinalId calldata ordinalID,
    BitcoinTx.UTXO calldata utxo,
    address sellToken,
    uint256 sellAmount
) public;
```

### acceptOrdinalSellOrder


```solidity
function acceptOrdinalSellOrder(uint256 id, BitcoinAddress calldata bitcoinAddress) public returns (uint256);
```

### proofOrdinalSellOrder


```solidity
function proofOrdinalSellOrder(uint256 id, BitcoinTx.Info calldata transaction, BitcoinTx.Proof calldata proof)
    public;
```

### _checkBitcoinTxOutput

Checks output script pubkey (recipient address) matches output script.
Reverts if bitcoin address is not found.


```solidity
function _checkBitcoinTxOutput(BitcoinAddress storage bitcoinAddress, BitcoinTx.Info calldata transaction)
    private
    view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`bitcoinAddress`|`BitcoinAddress`|Recipient's bitcoin address.|
|`transaction`|`BitcoinTx.Info`|Transaction fulfilling the order.|


### withdrawOrdinalSellOrder


```solidity
function withdrawOrdinalSellOrder(uint256 id) public;
```

### cancelAcceptedOrdinalSellOrder


```solidity
function cancelAcceptedOrdinalSellOrder(uint256 id) public;
```

### getOpenOrdinalSellOrders


```solidity
function getOpenOrdinalSellOrders() external view returns (OrdinalSellOrder[] memory, uint256[] memory);
```

### getOpenAcceptedOrdinalSellOrders


```solidity
function getOpenAcceptedOrdinalSellOrders()
    external
    view
    returns (AcceptedOrdinalSellOrder[] memory, uint256[] memory);
```

## Events
### placeOrdinalSellOrderEvent

```solidity
event placeOrdinalSellOrderEvent(uint256 indexed orderId, OrdinalId ordinalID, address sellToken, uint256 sellAmount);
```

### acceptOrdinalSellOrderEvent

```solidity
event acceptOrdinalSellOrderEvent(
    uint256 indexed id, uint256 indexed acceptId, BitcoinAddress bitcoinAddress, address ercToken, uint256 ercAmount
);
```

### proofOrdinalSellOrderEvent

```solidity
event proofOrdinalSellOrderEvent(uint256 id);
```

### withdrawOrdinalSellOrderEvent

```solidity
event withdrawOrdinalSellOrderEvent(uint256 id);
```

### cancelAcceptedOrdinalSellOrderEvent

```solidity
event cancelAcceptedOrdinalSellOrderEvent(uint256 id);
```

## Structs
### OrdinalSellOrder

```solidity
struct OrdinalSellOrder {
    OrdinalId ordinalID;
    address sellToken;
    uint256 sellAmount;
    BitcoinTx.UTXO utxo;
    address requester;
    bool isOrderAccepted;
}
```

### AcceptedOrdinalSellOrder

```solidity
struct AcceptedOrdinalSellOrder {
    uint256 orderId;
    BitcoinAddress bitcoinAddress;
    address ercToken;
    uint256 ercAmount;
    address requester;
    address acceptor;
    uint256 acceptTime;
}
```

### BitcoinAddress

```solidity
struct BitcoinAddress {
    bytes scriptPubKey;
}
```

### OrdinalId

```solidity
struct OrdinalId {
    bytes32 txId;
    uint32 index;
}
```

