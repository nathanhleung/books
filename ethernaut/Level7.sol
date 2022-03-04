// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Level7 {
    constructor() public {}

    function main(address payable forceAddr) external payable {
        selfdestruct(forceAddr);
    }
}
