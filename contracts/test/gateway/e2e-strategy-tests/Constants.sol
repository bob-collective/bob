// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/**
 * @title Commonly used constants.
 */
library Constants {
    // Used for providing a uniform interface for functions that deal with both ERC20 and native tokens.
    address constant WBTC_ADDRESS = 0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3;
    address constant WBTC_OFT_ADDRESS = 0x0555E30da8f98308EdB960aa94C0Db47230d2B9c;
    address constant TBTC_ADDRESS = 0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2;
    address constant DUMMY_SENDER = 0x999999cf1046e68e36E1aA2E0E07105eDDD1f08E;
    address constant DUMMY_RECEIVER = 0x0000000000000000000000000000000000000001;
}
