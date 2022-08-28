// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {Solver} from "src/Level18/Solver.sol";

contract Level18Script is Script {
    function run() public {
        Solver solver = new Solver();
        solver.whatIsTheMeaningOfLife();
    }
}
