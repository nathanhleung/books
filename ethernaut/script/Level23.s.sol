// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {DexTwo, SwappableTokenTwo} from "src/Level23/DexTwo.sol";
import {Level23, MyToken} from "src/Level23/Level23.sol";

contract Level23Script is Script {
    function run() public {
        DexTwo dexTwo = new DexTwo();
        SwappableTokenTwo token1 = new SwappableTokenTwo(
            address(dexTwo),
            "TOKEN1",
            "TK1",
            110 * (10**18)
        );
        SwappableTokenTwo token2 = new SwappableTokenTwo(
            address(dexTwo),
            "TOKEN2",
            "TK2",
            110 * (10**18)
        );
        dexTwo.setTokens(address(token1), address(token2));
        dexTwo.approve(address(dexTwo), 2**256 - 1);
        dexTwo.add_liquidity(address(token1), 100);
        dexTwo.add_liquidity(address(token2), 100);

        Level23 level23 = new Level23(address(dexTwo));

        token1.transfer(address(level23), 10 * (10**18));
        token2.transfer(address(level23), 10 * (10**18));

        level23.main();
    }
}
