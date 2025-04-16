// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.17;

import {IFullRelay} from "./IFullRelay.sol";

interface IFullRelayWithVerify is IFullRelay {
    function verifyProof(bytes calldata _header, bytes calldata _proof, bytes32 _txId, uint256 _index, uint8 _numConfs)
        external
        view;

    function verifyHeaderHash(bytes32 _headerHash, uint8 _numConfs) external view;
}
