// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./Motorbike.sol";

contract Level25 {
    Motorbike motorbike;
    Engine engine;

    constructor(address payable motorbikeAddr) public {
        motorbike = Motorbike(motorbikeAddr);
        engine = Engine(motorbikeAddr);
    }

    function main() public {}
}
