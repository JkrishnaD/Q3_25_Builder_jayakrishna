import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
import {
  PublicKey,
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  createAssociatedTokenAccount,
  createMint,
  getAssociatedTokenAddress,
  mintTo,
} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { expect } from "chai";

describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.nftStaking as Program<NftStaking>;

  const user = provider.wallet;
  let userAccountPda: anchor.web3.PublicKey;
  let configPda: anchor.web3.PublicKey;
  let stakeAccount: anchor.web3.PublicKey;
  let rewardMint: anchor.web3.PublicKey;
  let nftMint: anchor.web3.PublicKey;
  let userNftAccount: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;
  let vaultBump: number;
  let stakePda: PublicKey;

  const POINTS_PER_STAKE = 10;
  const MAX_UNSTAKE = 5;
  const FREEZE_PERIOD = 5;

  before(async () => {
    [userAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), user.publicKey.toBuffer()],
      program.programId
    );

    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );

    [rewardMint] = PublicKey.findProgramAddressSync(
      [Buffer.from("rewards"), configPda.toBuffer()],
      program.programId
    );

    nftMint = await createMint(
      provider.connection,
      user.payer,
      user.publicKey,
      user.publicKey,
      6
    );

    [stakePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("stake"), user.publicKey.toBuffer(), nftMint.toBuffer()],
      program.programId
    );

    [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), nftMint.toBuffer()],
      program.programId
    );

    userNftAccount = await createAssociatedTokenAccount(
      provider.connection,
      user.payer,
      nftMint,
      user.publicKey
    );

    await mintTo(
      provider.connection,
      user.payer,
      nftMint,
      userNftAccount,
      user.publicKey,
      1
    );
  });

  it("User initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializUser()
      .accountsPartial({
        user: user.publicKey,
        userAccount: userAccountPda,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const userAccount = await program.account.userAccount.fetch(userAccountPda);
    console.log(userAccount);

    expect(userAccount.points).equal(0);
    expect(userAccount.amountStaked).equal(0);
  });

  it("Config initialized!", async () => {
    const tx = await program.methods
      .initializeConfig(POINTS_PER_STAKE, MAX_UNSTAKE, FREEZE_PERIOD)
      .accountsPartial({
        admin: user.publicKey,
        config: configPda,
        rewardMint: rewardMint,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const configAccount = await program.account.stakeConfig.fetch(configPda);
    console.log(configAccount);
  });

  it("Stake nft!", async () => {
    const tx = await program.methods
      .stake()
      .accountsPartial({
        config: configPda,
        nftMint,
        stakeAccount,
        user: user.publicKey,
        userAccount: userAccountPda,
        userNftAta: userNftAccount,
        vaultPda,
        rent: SYSVAR_RENT_PUBKEY,
        clock: SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const userAccount = await program.account.userAccount.fetch(userAccountPda);
    console.log(userAccount);

    const vaultBalance = await provider.connection.getTokenAccountBalance(
      vaultPda
    );
    console.log("Vault token balance:", vaultBalance.value.uiAmount);

    const userBalance = await provider.connection.getTokenAccountBalance(
      userNftAccount
    );
    console.log("User NFT balance:", userBalance.value.uiAmount);
  });

  it("Unstake nft!", async () => {
    await new Promise((r) => setTimeout(r, 6000));
    const tx = await program.methods
      .unstake()
      .accountsPartial({
        config: configPda,
        nftMint,
        stakeAccount,
        user: user.publicKey,
        userAccount: userAccountPda,
        userNftAta: userNftAccount,
        vaultPda,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const vaultBalance = await provider.connection.getTokenAccountBalance(
      vaultPda
    );
    console.log("Vault token balance:", vaultBalance.value.uiAmount);

    const userBalance = await provider.connection.getTokenAccountBalance(
      userNftAccount
    );
    console.log("User NFT balance:", userBalance.value.uiAmount);
  });
});
