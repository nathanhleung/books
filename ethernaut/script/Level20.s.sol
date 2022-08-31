// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {Denial} from "src/Level20/Denial.sol";
import {Level20} from "src/Level20/Level20.sol";

contract Level20Script is Script {
    function run() public {
        Denial denial = new Denial();
        // Deposit to `denial` contract
        address(denial).call{value: 100 * (10**18)}("");

        Level20 level20 = new Level20(payable(denial));
        level20.main();
    }
}
