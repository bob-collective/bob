// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

import {Test, console2} from "forge-std/Test.sol";

import {LightRelay} from "../src/relay/LightRelay.sol";
import {BitcoinTx} from "../src/bridge/BitcoinTx.sol";
import {BridgeState} from "../src/bridge/BridgeState.sol";

contract LightRelayTest is Test {
    using BitcoinTx for BridgeState.Storage;

    LightRelay public relay;
    BridgeState.Storage internal state;

    struct Header {
        bytes data;
        uint256 height;
    }

    Header genesisHeader;
    Header[4] retargetHeaders;
    Header[2] proofHeaders;

    constructor() public {
        genesisHeader = Header({
            data: hex"04000000473ed7b7ef2fce828c318fd5e5868344a5356c9e93b6040400000000000000004409cae5b7b2f8f18ea55f558c9bfa7c5f4778a1a53172a48fc57e172d0ed3d264c5eb56c3a40618af9bc1c7",
            height: 403200
        });

        retargetHeaders[0] = Header({
            data: hex"04000000ea6410972dfe65f4d93d376c1678148baff89b914654460600000000000000000275ca93e15993399290808da2da97bce309e5d0f559713622fca2b32bf6330e1e10fe56c3a4061860b02896",
            height: 405214
        });
        retargetHeaders[1] = Header({
            data: hex"040000005f5560a04006b8a4a6fc85c7a7816c36f89a6da5b03aaa000000000000000000e74d25fadc646f6e0e2b781da6d172170ed1213afb9ac6f05c5a5dc482830e520914fe56c3a40618a6a1ffc0",
            height: 405215
        });
        // **RETARGET**
        retargetHeaders[2] = Header({
            data: hex"04000000f7ef2881b8a0cb415ba81e889c79bc5f1b098167c95646030000000000000000a48869fe8d6777821fa85525139cb77d12c440c16182c637e943dfea7d937daa7b16fe56f49606185628272d",
            height: 405216
        });
        retargetHeaders[3] = Header({
            data: hex"0000003007fc81082e2d4d3b44e6ee00ad53a9d926213aee7394960600000000000000003a810e13e6253dd483a95f55cfc3adbc60e112f0d485832ea1d482661487e9dc411cfe56f496061837c4a22f",
            height: 405217
        });

        proofHeaders[0] = Header({
            data: hex"04000000e0879a33a87bf9481385adae91fa9e93713b932cbe8a09030000000000000000ee5ded948d805bb71bee5de25b447c42527898cac93eee1afe04663bb8204b358627fe56f4960618304a7db1",
            height: 405220
        });
        proofHeaders[1] = Header({
            data: hex"04000000c0de92e7326cb020b59ffc5998405e539863c57da088a7040000000000000000d8e7273d0198ba4f10dfd57d151327c32113fc244fd0587d161a5c5332a53651ed28fe56f4960618b24502cc",
            height: 405221
        });
    }

    function setUp() public {
        relay = new LightRelay();
        relay.genesis(
            genesisHeader.data,
            genesisHeader.height,
            2
        );
        state.relay = relay;
        state.txProofDifficultyFactor = 2;

        // we need at least one retarget
        vm.warp(1459492475);
        relay.retarget(abi.encodePacked(
            retargetHeaders[0].data,
            retargetHeaders[1].data,
            retargetHeaders[2].data,
            retargetHeaders[3].data
        ));
    }

    function test_Retarget() public {
        // actually submitted in setup, but check expected difficulty here
        assertEq(relay.getCurrentEpochDifficulty(), 166851513282);
    }

    function test_ValidateProof() public {
        // 2ef69769cc0ee81141c79552de6b91f372ff886216dbfa84e5497a16b0173e79
        state.validateProof(
            BitcoinTx.Info({
                version: hex"01000000",
                inputVector: hex"01996cf4e2f0016a1f092aaaba653c7eae5dd4b6eef1f9a2a94c64f34b2fecbd85010000006a47304402206f99da49ce586528ed8981842df30b4a5a91195fd2d83e440d4193fc16a944ec022055cfdf63a2c90638821f1b5ff1fdf77526163ae057a0d0de30a6e1d3009e7a29012102811832eef7216470f489991f1d87e36d2890755d2bbf827eb1e71804491506afffffffff",
                outputVector: hex"0200e9a435000000001976a914fd7e6999cd7e7114383e014b7e612a88ab6be68f88ac804a5d05000000001976a9145c1addbd0e4e78479e71fdca0555d2d44b67378e88ac",
                locktime: hex"00000000"
            }),
            BitcoinTx.Proof({
                merkleProof: hex"0465f99dbe384bbc5d86a5242712e4154958e4b01f595f14b76f873ec349e14a16b17770af2bb48c9b2ce4dddf4631866fe3753e6c54bdcf18dfb2d4fb9983ee58e4f3be92087c843b815bbe1d5d686dc972552f7ffda4342319ceb5bea67ab0f2e463ec8ce8e3f580c5e2470ef20c5b33398ab9fea5ccbd0b3e3f6211305edafa068a28c8ac634df5bbc8064357295373b97db2600745f23ad6ebc87b66b4a8685aa8ff8e69abc5029dbf4b2fa03f05680c7a2c491410b23a5a6b27c5a91b89dac8cdd16a4460ce8ac8d17491025d29336440a133867f938a7f41cc7a64f3f04ac3817c3eb6a6a11dc30850ca4e80f9abbd42268bcc626138bc01639a902713425e7d3aca45647001fb32ff396c07027c5b081325530e74f936e6c4a8078a05f9717efd315534a84d047ee2ff0b2b93159a2b98eabb578af67ef7540a58e488b9c587a994c1a9a86937ad343ea734b7427678e3e6ba0be8f5045ce47e541bbc",
                txIndexInBlock: 1,
                bitcoinHeaders: abi.encodePacked(
                    proofHeaders[0].data,
                    proofHeaders[1].data
                )
            })
        );
    }
}
