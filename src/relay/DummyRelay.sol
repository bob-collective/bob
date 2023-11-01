// SPDX-License-Identifier: GPL-3.0-only

// Forked from https://github.com/keep-network/tbtc-v2

pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";

import {BytesLib} from "@bob-collective/bitcoin-spv/BytesLib.sol";
import {BTCUtils} from "@bob-collective/bitcoin-spv/BTCUtils.sol";
import {ValidateSPV} from "@bob-collective/bitcoin-spv/ValidateSPV.sol";

import "../bridge/IRelay.sol";

/// @dev DEV-only!
contract DummyRelay is IRelay {
    function getCurrentEpochDifficulty() external view returns (uint256) {
        return 0;
    }

    function getPrevEpochDifficulty() external view returns (uint256) {
        return 0;
    }

    function difficultyCheckEnabled() external view returns (bool) {
        return false;
    }
}
