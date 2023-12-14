// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

using stdStorage for StdStorage;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
import {BtcMarketPlace} from "../../src/swap/Btc_Marketplace.sol";
import {Utilities} from "./Utilities.sol";
import {BitcoinTx} from "../../src/bridge/BitcoinTx.sol";
import {TestLightRelay} from "../../src/relay/TestLightRelay.sol";
import "../../src/swap/Ord_Marketplace.sol";

contract ArbitaryErc20 is ERC20, Ownable {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

contract OrdMarketPlaceTest is OrdMarketplace, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;

    ArbitaryErc20 token1;
    TestLightRelay testLightRelay;

    constructor() OrdMarketplace(testLightRelay) {}

    function test_dummy() public {}
}
