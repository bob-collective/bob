# TestLightRelay
[Git Source](https://github.com/bob-collective/bob/blob/51411a1dc3e9caf1fbd213b112cb280e5b93e6df/src/relay/TestLightRelay.sol)

**Inherits:**
[LightRelay](../../relay/LightRelay.sol/contract.LightRelay.md)

TestLightRelay is a stub version of LightRelay intended to be
used on for testing network. It allows to set the relay's
difficulty based on arbitrary Bitcoin headers thus effectively
bypass the validation of difficulties of Bitcoin testnet blocks.
Since difficulty in Bitcoin testnet often falls to `1` it would not
be possible to validate blocks with the real LightRelay.

*Notice that TestLightRelay is derived from LightRelay so that the two
contracts have the same API and correct bindings can be generated.*


## Functions
### setDifficultyFromHeaders

Sets the current and previous difficulty based on the difficulty
inferred from the provided Bitcoin headers.


```solidity
function setDifficultyFromHeaders(bytes memory bitcoinHeaders) external onlyOwner;
```

