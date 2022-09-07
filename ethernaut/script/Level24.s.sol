// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.6.0;

import "forge-std/Script.sol";
import {PuzzleProxy, PuzzleWallet} from "src/Level24/PuzzleProxy.sol";
import {Level24} from "src/Level24/Level24.sol";

contract Level24Script is Script {
    function run() public {
        PuzzleWallet puzzleWallet = new PuzzleWallet();
        bytes memory initData = abi.encodeWithSignature(
            "init(uint256)",
            1285822837045366655748890655763751218771536593865
        );
        uint256 initBalance = 1000000000000000;

        PuzzleProxy puzzleProxy = new PuzzleProxy(
            address(this),
            address(puzzleWallet),
            initData
        );
        PuzzleWallet proxiedPuzzleWallet = PuzzleWallet(address(puzzleProxy));
        proxiedPuzzleWallet.addToWhitelist(address(this));
        proxiedPuzzleWallet.deposit{value: initBalance}();

        Level24 level24 = new Level24(address(puzzleProxy));
        address desiredAdmin = 0x4a8631e84Dd2E5E31100bF4502Fea598626906ee;
        level24.main{value: initBalance}(desiredAdmin);

        console.logAddress(puzzleProxy.admin());
    }
}
