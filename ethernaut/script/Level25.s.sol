// SPDX-License-Identifier: UNLICENSED
pragma solidity <0.7.0;

import "forge-std/Script.sol";
import {Motorbike, Engine} from "src/Level25/Motorbike.sol";
import {Level25} from "src/Level25/Level25.sol";

contract Level25Script is Script {
    function run() public {
        Engine engine = new Engine();
        Motorbike motorbike = new Motorbike(address(engine));

        Level25 level25 = new Level25(address(motorbike), address(engine));
        level25.main();
    }
}
