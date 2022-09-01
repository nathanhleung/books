// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {Dex, SwappableToken} from "src/Level22/Dex.sol";
import {Level22} from "src/Level22/Level22.sol";

contract Level22Script is Script {
    function run() public {
        Dex dex = new Dex();
        SwappableToken token1 = new SwappableToken(
            address(dex),
            "TOKEN1",
            "TK1",
            110 * (10**18)
        );
        SwappableToken token2 = new SwappableToken(
            address(dex),
            "TOKEN2",
            "TK2",
            110 * (10**18)
        );
        dex.setTokens(address(token1), address(token2));
        dex.approve(address(dex), 2**256 - 1);
        dex.addLiquidity(address(token1), 100 * (10**18));
        dex.addLiquidity(address(token2), 100 * (10**18));

        Level22 level22 = new Level22(
            address(dex),
            address(token1),
            address(token2)
        );

        token1.transfer(address(level22), 10 * (10**18));
        token2.transfer(address(level22), 10 * (10**18));

        level22.main();
    }
}
