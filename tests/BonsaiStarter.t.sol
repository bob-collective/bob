// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

import {BonsaiTest} from "bonsai/BonsaiTest.sol";
import {IBonsaiRelay} from "bonsai/IBonsaiRelay.sol";
import {BonsaiStarter} from "contracts/BonsaiStarter.sol";

contract BonsaiStarterTest is BonsaiTest {
    function setUp() public withRelay {}

    function testMockCall() public {
        // Deploy a new starter instance
        BonsaiStarter starter = new BonsaiStarter(
            IBonsaiRelay(bonsaiRelay),
            queryImageId('FIBONACCI'));

        // Anticipate a callback request to the relay
        vm.expectCall(address(bonsaiRelay), abi.encodeWithSelector(IBonsaiRelay.requestCallback.selector));

        bytes memory pubkey = hex"027becf3c0642228336f8182a04081870bb8445453c6eac4960dd05622960a3bf7";
        bytes memory secureId = hex"0101010101010101010101010101010101010101010101010101010101010101";

        
        // Request the callback
        starter.calculateFibonacci(pubkey, secureId);

        // Anticipate a callback invocation on the starter contract
        vm.expectCall(address(starter), abi.encodeWithSelector(BonsaiStarter.storeResult.selector));
        // Relay the solution as a callback
        runPendingCallbackRequest();

        bytes memory expected = hex"039cb8d281b3aa8b4429c1b52c6d307b45fb29638288b9e4602b470c1d71c5ac80";
        // Validate the Fibonacci solution value
        bytes memory resultq = starter.result();
        assertEq(resultq, expected);
    }
}
