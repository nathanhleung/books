// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {GatekeeperOne} from "src/Level13/GatekeeperOne.sol";
import {Level13} from "src/Level13/Level13.sol";

contract Level13Script is Script {
    function run() public {
        GatekeeperOne gatekeeperOne = new GatekeeperOne();
        Level13 level13 = new Level13(address(gatekeeperOne));
        level13.main();
    }
}
