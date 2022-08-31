// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {Shop} from "src/Level21/Shop.sol";
import {Level21} from "src/Level21/Level21.sol";

contract Level21Script is Script {
    function run() public {
        Shop shop = new Shop();

        console.logBool(shop.isSold());
        console.logUint(shop.price());

        Level21 level21 = new Level21(address(shop));
        level21.main();

        console.logBool(shop.isSold());
        console.logUint(shop.price());
    }
}
