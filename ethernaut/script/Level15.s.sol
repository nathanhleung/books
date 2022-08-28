// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {NaughtCoin} from "src/Level15/NaughtCoin.sol";
import {Level15} from "src/Level15/Level15.sol";

contract Level15Script is Script {
    function run() public {
        NaughtCoin naughtCoin = new NaughtCoin(tx.origin);
        Level15 level15 = new Level15(address(naughtCoin));
        vm.startPrank(tx.origin);
        naughtCoin.approve(address(level15), uint256(-1));
        vm.stopPrank();
        level15.main();
    }
}
