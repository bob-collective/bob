// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

using stdStorage for StdStorage;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
import {Bridge} from "../../src/swap/Bridge.sol";
import {Utilities} from "./Utilities.sol";

contract BridgeTest is Bridge, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;

    constructor() Bridge(2) {}

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(5);

        alice = users[0];
        vm.label(alice, "Alice");
        bob = users[1];
        vm.label(bob, "Bob");
    }

    function testMint() public {
        uint256 _amount = 1000;

        vm.deal(alice, _amount);
        vm.startPrank(alice);
        uint256 balanceBefore = address(this).balance;
        (bool _success,) = address(this).call{value: _amount}(abi.encodeWithSignature("mint()", alice));
        assertTrue(_success, "deposited payment.");

        // zbtc minted
        assertEqDecimal(wrapped.balanceOf(alice), _amount / 5 / 2, wrapped.decimals());
        // col has been transferred to contract
        assertEqDecimal(address(this).balance, balanceBefore + _amount, wrapped.decimals());
    }

    function testLiquidate() public {
        uint256 _amount = 1000;

        vm.deal(alice, _amount);
        vm.startPrank(alice);
        (bool _success,) = address(this).call{value: _amount}(abi.encodeWithSignature("mint()", alice));
        assertTrue(_success);

        // liquidate 25% of what was minted
        assertEqDecimal(wrapped.balanceOf(alice), 100, wrapped.decimals());
        this.liquidate(25);
        assertEqDecimal(wrapped.balanceOf(alice), 75, wrapped.decimals());
        assertEqDecimal(address(alice).balance, btcToCol(25), wrapped.decimals());
    }

    function testWithdraw() public {
        uint256 _amount = 1000;

        vm.deal(alice, _amount);
        vm.startPrank(alice);
        (bool _success,) = address(this).call{value: _amount}(abi.encodeWithSignature("mint()", alice));
        assertTrue(_success);

        assertEq(collateralThreshold, 2);
        stdstore.target(address(this)).sig(this.collateralThreshold.selector).checked_write(1);
        assertEq(collateralThreshold, 1); // sanity check that writing to storage worked

        uint256 lockedBefore = suppliedCollateral[alice];
        this.withdraw();
        assertEq(suppliedCollateral[alice], lockedBefore / 2);
    }

    function testRedeem() public {
        uint256 amountCol = 1000;

        uint256 zbtcAmount = 100;
        uint256 btcAmount = 50;

        vm.deal(alice, amountCol);
        vm.startPrank(alice);
        (bool _success,) = address(this).call{value: amountCol}(abi.encodeWithSignature("mint()", alice));
        assertTrue(_success, "deposited payment.");

        BitcoinAddress memory btcAddress = BitcoinAddress({bitcoinAddress: 1});
        this.requestSwap(zbtcAmount, btcAmount, btcAddress);

        // Order memory order = orders(0);
        assertEq(orders[0].amountZbtc, zbtcAmount);
        assertEq(orders[0].amountBtc, btcAmount);
        assertTrue(orders[0].open);

        vm.startPrank(bob);
        this.acceptSwap(0);
        assertFalse(orders[0].open);

        TransactionProof memory proof = TransactionProof({dummy: 1});
        assertEqDecimal(wrapped.balanceOf(bob), 0, wrapped.decimals());
        this.executeSwap(0, proof);
        assertEqDecimal(wrapped.balanceOf(bob), zbtcAmount, wrapped.decimals());
    }
}
