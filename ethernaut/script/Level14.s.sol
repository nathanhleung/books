// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {GatekeeperTwo} from "src/Level14/GatekeeperTwo.sol";
import {Level14} from "src/Level14/Level14.sol";

contract Level14Script is Script {
    function run() public {
        // address gatekeeperTwo = 0x3352F8df70d134488C06d45E8dac8C1eA8b07c68;
        GatekeeperTwo gatekeeperTwo = new GatekeeperTwo();
        Level14 level14 = new Level14(address(gatekeeperTwo));
    }
}
