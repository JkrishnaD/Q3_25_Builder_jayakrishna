import { Commitment, Connection, Keypair } from "@solana/web3.js";
import wallet from "./turbine3-wallet.json";
import { createMint } from "@solana/spl-token";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

console.log(keypair.publicKey.toBase58());

const commitment: Commitment = "confirmed";

const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
  try {
    // create a mint account to store the token mints
    const mint = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      null,
      6
    );
    console.log(`Succesfully created mint:${mint}`);
  } catch (error) {
    console.log(`Error while creating the mint ${error}`);
  }
})();
