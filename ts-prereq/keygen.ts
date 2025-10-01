import { Keypair } from "@solana/web3.js";

let kp = Keypair.generate();

console.log(`You have generated a new solana wallet :${kp.publicKey.toBase58()}`)

console.log(`Your private key is: ${kp.secretKey.toString()}`);

