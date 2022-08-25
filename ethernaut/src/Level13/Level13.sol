// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./GatekeeperOne.sol";

contract Level13 {
    GatekeeperOne gatekeeperOneContract;

    constructor(address gatekeeperOneAddr) public {
        gatekeeperOneContract = GatekeeperOne(gatekeeperOneAddr);
    }

    function main() public {
        // Derived from the constraints of the require()
        // statements
        uint64 gatekey = 0xaabbccdd00000000;
        gatekey += uint64(uint16(tx.origin));
        bytes8 gatekey_bytes = bytes8(gatekey);

        gatekeeperOneContract.enter(gatekey_bytes);
    }
}
