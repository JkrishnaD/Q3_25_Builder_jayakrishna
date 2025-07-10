import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";

import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  Account,
  TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("Escrow program tests", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.escrow as Program<Escrow>;

  const connection = provider.connection;

  let maker: Keypair;
  let taker: Keypair;
  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerAtaA: Account;
  let makerAtaB: Account;
  let takerAtaA: Account
  let takerAtaB: Account
  let vault: PublicKey;
  let escrow: PublicKey;
  let seed: anchor.BN;

  before(async () => {
    seed = new anchor.BN(1);

    // assigning the addresses to both the maker and taker
    maker = new anchor.web3.Keypair();
    taker = new anchor.web3.Keypair();

    // airdropping the sol to both the accounts using helper function
    await airdrop(connection, maker.publicKey, 2);
    await airdrop(connection, taker.publicKey, 2);

    // creating the mints and associated token for both the maker and taker
    mintA = await createMint(connection, maker, maker.publicKey, null, 6);
    console.log("Address of mintA:", mintA.toBase58());

    // creating the second mint for taker
    mintB = await createMint(connection, taker, taker.publicKey, null, 6);
    console.log("Address of mintB:", mintB.toBase58());

    // creating the associated token account for both minta and mintb
    makerAtaA = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintA,
      maker.publicKey
    );
    console.log("Address of makerAtaA:", makerAtaA.address.toBase58());

    makerAtaB = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintB,
      maker.publicKey
    );
    console.log("Address of makerAtaA:", makerAtaA.address.toBase58());

    takerAtaA = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintA,
      taker.publicKey
    )
    console.log("Address of takerAtaA:", takerAtaA.address.toBase58());

    takerAtaB = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintB,
      taker.publicKey
    )
    // minting 10000 tokens to both the minta and mintb associated token accounts
    const minta_tx = await mintTo(
      connection,
      maker,
      mintA,
      makerAtaA.address,
      maker,
      10000 * 10 ** 6
    );
    console.log("Minted 10000 tokens to makerAtaA:", minta_tx);

    const mintb_tx = await mintTo(
      connection,
      taker,
      mintB,
      makerAtaB.address,
      taker,
      10000 * 10 ** 6
    );
    console.log("Minted 10000 tokens to mnakerAtaB:", mintb_tx);

    const takerAtaA_tx = await mintTo(
      connection,
      maker,
      mintA,
      takerAtaA.address,
      maker,
      10000 * 10 ** 6
    );
    console.log("Minted 10000 tokens to takerAtaA:", takerAtaA_tx);

    const takerAtaB_tx = await mintTo(
      connection,
      taker,
      mintB,
      takerAtaB.address,
      taker,
      10000 * 10 ** 6
    );
    console.log("Minted 10000 tokens to takerAtaB:", takerAtaB_tx);

    // deriving the required seeds based on our escrow program
    [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // the vault account which will hold the tokens
    vault = getAssociatedTokenAddressSync(
      mintA,
      escrow,
      true, // allow owner override
    );
  });

  console.log("✅ Vault Address: ", vault);


  it("Make escrow!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(seed, new BN(1e6), new anchor.BN(1e6)).accountsPartial({
      maker: maker.publicKey,
      escrow,
      vault,
      makerAtaA: makerAtaA.address,
      mintA: mintA,
      mintB: mintB,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([maker]).rpc();

    console.log("Make escrow signature", tx);

  });


  it("Take escrow!", async () => {
    const tx = await program.methods.take(new BN(1e6)).accountsPartial({
      escrow,
      taker: taker.publicKey,
      makerAtaB: makerAtaB.address,
      maker: maker.publicKey,
      mintA,
      mintB,
      takerAtaA: takerAtaA.address,
      takerAtaB: takerAtaB.address,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,

    }).signers([taker]).rpc();
    console.log("Take escrow signature", tx);
  })

  it("Refund escrow!", async () => {
    const tx = await program.methods.refund().accountsPartial({
      escrow,
      maker: maker.publicKey,
      makerAtaA: makerAtaA.address,
      mintA,
      systemProgram: anchor.web3.SystemProgram.programId,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([maker]).rpc()
    console.log("Refund signature", tx)
  })

});

async function airdrop(
  connection: anchor.web3.Connection,
  address: PublicKey,
  amount: number
) {
  const tx = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("Airdrop signature", tx);

  let confirmedAirdrop = await connection.confirmTransaction(tx, "confirmed");
  console.log(`Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}
