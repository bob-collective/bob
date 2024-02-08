// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
//
// using stdStorage for StdStorage;
//
// import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";
// import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
// import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";
// import {Utilities} from "./swap/Utilities.sol";
// import {OracleTokenPaymaster} from "../src/OracleTokenPaymaster.sol";
// // import {}
// import "lib/gsn/packages/contracts/src/BasePaymaster.sol";
// import {Oracle} from "../src/Oracle.sol";
//
// contract ArbitaryErc20 is ERC20, Ownable {
//     constructor(
//         string memory name_,
//         string memory symbol_
//     ) ERC20(name_, symbol_) {}
//
//     function sudoMint(address to, uint256 amount) public onlyOwner {
//         _mint(to, amount);
//     }
// }
//
// contract OracleTokenPaymasterTest is OracleTokenPaymaster, Test {
//     Utilities internal utils;
//     address payable[] internal users;
//     address internal alice;
//     address internal bob;
//
//     ArbitaryErc20 token1;
//
//     constructor() OracleTokenPaymaster(new Oracle()) {}
//
//     function setUp() public {
//         utils = new Utilities();
//         users = utils.createUsers(5);
//
//         alice = users[0];
//         vm.label(alice, "Alice");
//         bob = users[1];
//         vm.label(bob, "Bob");
//
//         token1 = new ArbitaryErc20("Some token", "TKN");
//     }
//
//     function testQ() public {
//         token1.sudoMint(alice, 100);
//         vm.startPrank(alice);
//         token1.approve(address(this), 100);
//         vm.stopPrank();
//
//         IForwarder.ForwardRequest memory forwardRequest = IForwarder.ForwardRequest({
//             from: address(0x00),
//             to: alice,
//             value: 0,
//             gas: 0,
//             nonce: 0,
//             data: hex"",
//             validUntilTime: 0
//         });
//
//         GsnTypes.RelayData memory relayData = GsnTypes.RelayData({
//             maxFeePerGas: 0,
//             maxPriorityFeePerGas: 0,
//             transactionCalldataGasUsed: 0,
//             relayWorker: address(0x00),
//             paymaster: address(0x00),
//             forwarder: address(0x00),
//             paymasterData: abi.encode(token1),
//             clientId: 0
//         });
//
//         GsnTypes.RelayRequest memory relayRequest = GsnTypes.RelayRequest({
//             request: forwardRequest,
//             relayData: relayData
//         });
//
//         bytes memory signature = hex"";
//         bytes memory approvalData = hex"";
//         uint maxPossibleGas = 10000;
//         // this.__preRelayedCall(
//         //     relayRequest,
//         //     signature,
//         //     approvalData,
//         //     maxPossibleGas
//         // );
//
//         // address tokenAddress = abi.decode(abi.encode(address(0x4Cb8b11EfF23E56A4546787b418102eD4180B2e8)), (address));
//         // assertEq(hex"00", abi.encode(address(0x4Cb8b11EfF23E56A4546787b418102eD4180B2e8)));
//         address tokenAddress = abi.decode(hex"0000000000000000000000004cb8b11eff23e56a4546787b418102ed4180b2e8", (address));
//
//         // IERC20 a = this._getToken(hex"4Cb8b11EfF23E56A4546787b418102eD4180B2e8");
//         // IERC20 b = IERC20(address(0x00));
//         //
//         // assertEq(address(a), address(b));
//     }
//     // function testW() public {
//     //     bytes memory b = abi.encode(token1);
//     //     address a = abi.decode(b, (address));
//     // }
// }
