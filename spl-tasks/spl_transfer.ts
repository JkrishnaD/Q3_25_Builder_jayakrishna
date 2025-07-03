import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "./turbine3-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const commitment: Commitment = "confirmed";

const connection = new Connection("https://api.devnet.solana.com", commitment);

const mint = new PublicKey("2abbQyUMsmXUp18yP7mHtGgxUfEed999y7UM2etLFVkH");

const to = new PublicKey("E2YSjDvQhXHV62FTV5JtiW99JNDJ93z6Q9cG3DC3KFJ1");

(async () => {
  try {
    const ata_from = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    const ata_to = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );
    const tx = await transfer(
      connection,
      keypair,
      ata_from.address,
      ata_to.address,
      keypair.publicKey,
      1 * 1_000_000
    );

    console.log(`Transaction succesfull : ${tx}`);
  } catch (error) {
    console.log(`Oops something went wrong:${error}`);
  }
})();
