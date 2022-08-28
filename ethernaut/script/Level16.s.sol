// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {Preservation, LibraryContract} from "src/Level16/Preservation.sol";
import {Level16} from "src/Level16/Level16.sol";

contract Level16Script is Script {
    function run() public {
        LibraryContract timeZone1Library = new LibraryContract();
        LibraryContract timeZone2Library = new LibraryContract();
        Preservation preservation = new Preservation(
            address(timeZone1Library),
            address(timeZone2Library)
        );
        Level16 level16 = new Level16(address(preservation));
        level16.main();
    }
}
