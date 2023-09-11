// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC20} from "lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract Bridge is ERC20 {
    uint256 public number;
    uint256 public collateralThreshold;
    uint256 nextRedeemId;
    mapping(uint256 => RedeemRequest) public redeemRequests; // cant have struct as key, nor tupple
    mapping(address => uint256) public suppliedCollateral;
    uint256 totalCollateral;

    struct RedeemRequest {
        bool open; // actually redundant now that we have accepterAddress
        uint256 amountZbtc;
        uint256 amountBtc;
        address requesterAddress;
        address accepterAddress;
        BitcoinAddress bitcoinAddress;
    }
    struct BitcoinAddress {
        uint256 bitcoinAddress; // todo: use the right type
    }
    struct TransactionProof {
        // todo: fields here
        uint256 dummy;
    }

    constructor(uint256 threshold) ERC20("Our ZBTC", "ZBTC") {
        collateralThreshold = threshold;
    }

    /// lock COL in exchange for zBTC and cCOL
    function mintZbtc() public payable {
        uint256 collateral = msg.value; // this is the amount of eth sent to the contract
        uint256 mintedAmount = colToBtc(collateral) / collateralThreshold;
        _mint(msg.sender, mintedAmount);

        suppliedCollateral[msg.sender] += collateral;
        totalCollateral += collateral;
    }

    /// request zBTC to be redeemed for given amount of BTC.
    function requestRedeem(
        uint256 amountZbtc,
        uint256 amountBtc,
        BitcoinAddress calldata bitcoinAddress
    ) public {
        // lock Zbtc by transfering it to the contract address
        _transfer(msg.sender, address(this), amountZbtc);
        require(amountZbtc != 0);

        uint256 id = nextRedeemId++;
        redeemRequests[id] = RedeemRequest({
            open: true,
            amountZbtc: amountZbtc,
            amountBtc: amountBtc,
            requesterAddress: msg.sender,
            accepterAddress: address(0),
            bitcoinAddress: bitcoinAddress
        });
    }

    function acceptRedeem(uint256 id) public {
        RedeemRequest storage redeemRequest = redeemRequests[id];
        require(redeemRequest.open);

        // todo: protocol should probably require some sort of collateral deposit here

        redeemRequest.open = false;
        redeemRequest.accepterAddress = msg.sender;
    }

    // not documented, but presumably required
    function cancelRedeem(uint256 id) public {}

    function executeRedeem(
        uint256 id,
        TransactionProof calldata transactionProof
    ) public {
        // todo: check proof

        // move the zbtc thta was locked to whoever accepted the redeem
        RedeemRequest storage redeemRequest = redeemRequests[id];

        // check that the storage item exists and has been accepted
        require(!redeemRequest.open);
        require(redeemRequest.amountZbtc != 0);

        _transfer(
            address(this),
            redeemRequest.accepterAddress,
            redeemRequest.amountZbtc
        );

        delete redeemRequests[id];
        // clean up storage
    }

    function liquidate(uint256 amountZbtc) public {
        // burn the zbtc erc20
        _burn(msg.sender, amountZbtc);

        // transfer eth to caller
        uint256 collateral = btcToCol(amountZbtc);
        address payable callerAddress = payable(msg.sender);
        callerAddress.transfer(collateral);
    }

    function withdrawFreeCol() public {
        uint256 totalZbtc = totalSupply();
        uint256 requiredCol = btcToCol(totalZbtc * collateralThreshold);
        uint256 colFree = totalCollateral - requiredCol;
        uint256 withdrawal = colFree < suppliedCollateral[msg.sender]
            ? colFree
            : suppliedCollateral[msg.sender];
        suppliedCollateral[msg.sender] -= withdrawal;
        totalCollateral -= withdrawal;
    }

    function colToBtc(uint256 collateral) internal pure returns (uint) {
        return collateral / 5; // todo
    }

    function btcToCol(uint256 collateral) internal pure returns (uint) {
        return collateral * 5; // todo
    }
}
