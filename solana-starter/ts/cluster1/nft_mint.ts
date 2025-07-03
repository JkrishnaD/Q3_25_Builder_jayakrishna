import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../turbin3-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi)
const metadataUri = "https://raw.githubusercontent.com/JkrishnaD/rug-day-assets/main/metadata/template.json"

;(async () => {
  const tx = await createNft(umi, {
    mint,
    uri: metadataUri,
    name: "JHONNY NFT",
    symbol: "Jhonny",
    sellerFeeBasisPoints: percentAmount(2), 
    creators: null, 
  }).sendAndConfirm(umi)

  const signature = base58.encode(tx.signature)
  console.log(`🔗 Transaction: https://explorer.solana.com/tx/${signature}?cluster=devnet`)
  console.log(`🏷️ Mint Address: ${mint.publicKey}`)
})()
