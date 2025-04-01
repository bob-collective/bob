// SPDX-License-Identifier: GPL-3.0-only

// Forked from https://github.com/keep-network/tbtc-v2s

pragma solidity ^0.8.17;

/// @title Interface for the Bitcoin Light relay
interface ILightRelay {
    event Genesis(uint256 blockHeight);
    event Retarget(uint256 oldDifficulty, uint256 newDifficulty);
    event ProofLengthChanged(uint256 newLength);
    event AuthorizationRequirementChanged(bool newStatus);
    event SubmitterAuthorized(address submitter);
    event SubmitterDeauthorized(address submitter);

    function retarget(bytes memory headers) external;

    function validateChain(bytes memory headers)
        external
        view
        returns (uint256 startingHeaderTimestamp, uint256 headerCount);

    function getBlockDifficulty(uint256 blockNumber) external view returns (uint256);

    /// @notice Returns the difficulty of the current epoch.
    function getCurrentEpochDifficulty() external view returns (uint256);

    /// @notice Returns the difficulty of the previous epoch.
    function getPrevEpochDifficulty() external view returns (uint256);

    function getRelayRange() external view returns (uint256 relayGenesis, uint256 currentEpochEnd);
}
