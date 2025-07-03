import {
  CreateMetadataAccountV3InstructionArgs,
  createMetadataAccountV3,
} from "@metaplex-foundation/mpl-token-metadata/dist/src/generated/instructions/createMetadataAccountV3";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

import wallet from "./turbine3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import {
  CreateMetadataAccountV3InstructionAccounts,
  DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";

const mint = publicKey("2abbQyUMsmXUp18yP7mHtGgxUfEed999y7UM2etLFVkH");

const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

(async () => {
  try {
    let accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint,
      mintAuthority: signer,
    };

    let data: DataV2Args = {
      name: "Jaya",
      symbol: "JK",
      collection: null,
      creators: null,
      sellerFeeBasisPoints: 0,
      uri: "",
      uses: null,
    };

    let args: CreateMetadataAccountV3InstructionArgs = {
      data,
      collectionDetails: null,
      isMutable: true,
    };

    let tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });
    let results = await tx.sendAndConfirm(umi);

    console.log(`NFT is created at : ${bs58.encode(results.signature)}`);
  } catch (error) {
    console.log(`Oops something went wrong:${error}`);
  }
})();
