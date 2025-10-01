import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("Bt9AAsmv7ocm2kJsusYrk2gG1Sm6Fy6rS6dRtiC8xFGX")

const connection = new Connection("https://api.devnet.solana.com");

async function transfer(from: Keypair, to: PublicKey,) {
    try {
        const balance = await connection.getBalance(from.publicKey);

        const tx = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance
            })
        )
        tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
        tx.feePayer = from.publicKey;
        const fee = (await connection.getFeeForMessage(tx.compileMessage(), "confirmed")).value || 0;

        tx.instructions.pop(); // remove the last instruction which is the transfer
        tx.add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance - fee
            })
        );
        const signature = await sendAndConfirmTransaction(connection, tx, [from]);

        console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (error) {
        console.error(`Oops something went wrong:${error}`);
    }
}

// transfer(from, to, 0.1)
transfer(from, to)
