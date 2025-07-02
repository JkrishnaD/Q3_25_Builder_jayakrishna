import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js";
import wallet from "./turbine3-wallet.json";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const decimals = 1_000_000;

const commitment: Commitment = "confirmed";

const connection = new Connection("https://api.devnet.solana.com", commitment);

const mint = new PublicKey("2abbQyUMsmXUp18yP7mHtGgxUfEed999y7UM2etLFVkH");

(async () => {
  try {
    // get or create an associate token account
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    console.log(`Succesfully ata is created ${ata.address.toBase58()}`);

    // mint token to ata
    const mtx = await mintTo(
      connection,
      keypair,
      mint,
      ata.address,
      keypair.publicKey,
      100 * decimals
    );

    console.log(` Mint ${mtx} is done to ${ata.address.toBase58()}`);
  } catch (error) {
    console.log(`Oops something went wrong : ${error}`);
  }
})();
