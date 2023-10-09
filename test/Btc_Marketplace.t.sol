// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

using stdStorage for StdStorage;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
import {BtcMarketPlace} from "../src/swap/Btc_Marketplace.sol";
import {Utilities} from "./Utilities.sol";

contract ArbitaryErc20 is ERC20, Ownable {
    constructor(
        string memory name_,
        string memory symbol_
    ) ERC20(name_, symbol_) {}

    function sudoMint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}

contract MarketPlaceTest is BtcMarketPlace, Test {
    Utilities internal utils;
    address payable[] internal users;
    address internal alice;
    address internal bob;

    ArbitaryErc20 token1;

    constructor() BtcMarketPlace() {}

    function setUp() public {
        utils = new Utilities();
        users = utils.createUsers(5);

        alice = users[0];
        vm.label(alice, "Alice");
        bob = users[1];
        vm.label(bob, "Bob");

        token1 = new ArbitaryErc20("Some token", "TKN");
    }

    function testSellBtc() public {
        token1.sudoMint(bob, 100);

        vm.startPrank(alice);
        this.placeBtcSellOrder(1000, address(token1), 100);

        vm.startPrank(bob);
        token1.approve(address(this), 40);
        this.acceptBtcSellOrder(0, BitcoinAddress({bitcoinAddress: 0}), 40);

        vm.startPrank(alice);
        this.proofBtcSellOrder(1, TransactionProof({dummy: 0}));
    }

    function testWithdrawSellBtcOrder() public {
        token1.sudoMint(bob, 100);

        vm.startPrank(alice);
        this.placeBtcSellOrder(1000, address(token1), 100);

        vm.startPrank(bob);
        token1.approve(address(this), 40);
        this.acceptBtcSellOrder(0, BitcoinAddress({bitcoinAddress: 0}), 40);

        vm.startPrank(alice);
        this.withdrawBtcSellOrder(0);
        assertEq(token1.balanceOf(alice), 0);
        assertEq(token1.balanceOf(address(this)), 4);
    }

    function testCancelSellBtcOrder() public {
        token1.sudoMint(bob, 100);

        vm.startPrank(alice);
        this.placeBtcSellOrder(1000, address(token1), 100);

        vm.startPrank(bob);
        token1.approve(address(this), 40);
        this.acceptBtcSellOrder(0, BitcoinAddress({bitcoinAddress: 0}), 40);

        vm.warp(block.timestamp + REQUEST_EXPIRATION_SECONDS + 1);
        assertEq(token1.balanceOf(address(this)), 4);
        this.cancelAcceptedBtcSellOrder(1);
        assertEq(token1.balanceOf(bob), 100);
        assertEq(token1.balanceOf(alice), 0);
        assertEq(token1.balanceOf(address(this)), 0);
    }

    function testBuyBtc() public {
        token1.sudoMint(alice, 100);

        vm.startPrank(alice);
        token1.approve(address(this), 100);
        this.placeBtcBuyOrder(
            1000,
            BitcoinAddress({bitcoinAddress: 0}),
            address(token1),
            100
        );

        vm.startPrank(bob);
        this.acceptBtcBuyOrder(0, 40);

        this.proofBtcBuyOrder(1, TransactionProof({dummy: 0}));
        assertEq(token1.balanceOf(alice), 0);
        assertEq(token1.balanceOf(address(this)), 96);
        assertEq(token1.balanceOf(bob), 4);
    }

    function testWithdrawBuyBtc() public {
        token1.sudoMint(alice, 100);

        vm.startPrank(alice);
        token1.approve(address(this), 100);
        this.placeBtcBuyOrder(
            1000,
            BitcoinAddress({bitcoinAddress: 0}),
            address(token1),
            100
        );

        vm.startPrank(bob);
        this.acceptBtcBuyOrder(0, 40);

        assertEq(token1.balanceOf(address(this)), 100);

        vm.startPrank(alice);
        this.withdrawBtcBuyOrder(0);
        assertEq(token1.balanceOf(address(this)), 4); // 4 remains locked for the accepted part
        assertEq(token1.balanceOf(alice), 96); // 96 unlocked
    }

    function testcancelAcceptedBtcBuyOrder() public {
        token1.sudoMint(alice, 100);

        vm.startPrank(alice);
        token1.approve(address(this), 100);
        this.placeBtcBuyOrder(
            1000,
            BitcoinAddress({bitcoinAddress: 0}),
            address(token1),
            100
        );

        vm.startPrank(bob);
        this.acceptBtcBuyOrder(0, 40);

        assertEq(token1.balanceOf(address(this)), 100);

        vm.startPrank(alice);
        vm.warp(block.timestamp + REQUEST_EXPIRATION_SECONDS + 1);
        this.cancelAcceptedBtcBuyOrder(1);
        assertEq(token1.balanceOf(alice), 4); // 4 returned to alice
        assertEq(token1.balanceOf(address(this)), 96); // remainder remains available
    }
}
