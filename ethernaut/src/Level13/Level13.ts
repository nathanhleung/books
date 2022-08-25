import { ethers } from "ethers";

const abi = ["function main()"];

async function main() {
  const provider = new ethers.providers.JsonRpcProvider(
    "http://127.0.0.1:8545"
  );

  const level13Contract = new ethers.Contract(
    "0xd06a5ffdf3dcf36ba0e95e1adc7fe3d1f133a206",
    abi,
    provider
  );
  const signer = provider.getSigner();
  const level13ContractWithSigner = level13Contract.connect(signer);
  // 93031 gas works
  for (let i = 0; i < 8192; i += 1) {
    try {
      console.log(`Trying gas ${85000 + i}`);
      var options = { gasPrice: 1000000000, gasLimit: 85000 + i };
      const tx = await level13ContractWithSigner.main(options);
      const receipt = await tx.wait();
      console.log("Success");
      console.log(receipt);
      break;
    } catch (e) {
      // console.log(e);
    }
  }
}

main();
