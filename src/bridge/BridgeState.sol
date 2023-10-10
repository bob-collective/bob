// SPDX-License-Identifier: GPL-3.0-only

// Forked from https://github.com/keep-network/tbtc-v2

pragma solidity 0.8.17;

import "./IRelay.sol";

library BridgeState {
    struct Storage {
        // Bitcoin relay providing the current Bitcoin network difficulty.
        IRelay relay;
        // The number of confirmations on the Bitcoin chain required to
        // successfully evaluate an SPV proof.
        uint96 txProofDifficultyFactor;
    }
}
