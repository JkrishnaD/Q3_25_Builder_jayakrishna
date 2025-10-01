import { Connection, PublicKey } from "@solana/web3.js";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json"

const connection = new Connection("https://api.devnet.solana.com");
const keyPair = Keypair.fromSecretKey(new Uint8Array(wallet));

async function airdrop(publicKey: String, amount: number) {
    try {
        const tx = await connection.requestAirdrop(new PublicKey(publicKey), amount * LAMPORTS_PER_SOL);
        console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    } catch (e) {
        console.log(`Oops something went wrong:${e}`);
    }
}

airdrop(keyPair.publicKey.toBase58(), 2);