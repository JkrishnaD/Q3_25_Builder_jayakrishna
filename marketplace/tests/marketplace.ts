import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Marketplace } from "../target/types/marketplace";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createNft,
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
  verifySizedCollectionItem,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createAssociatedTokenAccount,
  createMint,
  getMint,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  KeypairSigner,
  createSignerFromKeypair,
  generateSigner,
  keypairIdentity,
  percentAmount,
  publicKey,
} from "@metaplex-foundation/umi";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { assert } from "chai";
import { BN } from "bn.js";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("marketplace", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const umi = createUmi("https://api.devnet.solana.com");

  // user accounts
  const admin = provider.wallet;
  let seller: Keypair;
  let buyer: Keypair;

  // ata accounts
  let sellerAta: PublicKey;
  let buyerAta: PublicKey;

  // contract accounts
  let marketplacePda: PublicKey;
  let listingPda: PublicKey;

  const MARKETPLACE_NAME = "Testing";

  let treasuryPda: PublicKey;
  let vault: PublicKey;

  // mint accounts
  let rewardMintPda: PublicKey;

  let nftMint: KeypairSigner = generateSigner(umi);
  let collectionMint: KeypairSigner = generateSigner(umi);

  const metadataProgram = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );
  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(
    new Uint8Array(admin.payer.secretKey)
  );

  const creator = createSignerFromKeypair(umi, creatorWallet);

  before(async () => {
    umi.use(keypairIdentity(creator));
    umi.use(mplTokenMetadata());

    seller = Keypair.generate();
    buyer = Keypair.generate();

    const transferTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: admin.publicKey,
        toPubkey: seller.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await provider.sendAndConfirm(transferTx, [admin.payer]);

    const transferTx1 = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: admin.publicKey,
        toPubkey: buyer.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await provider.sendAndConfirm(transferTx1, [admin.payer]);

    // Mint Collection NFT
    await createNft(umi, {
      mint: collectionMint,
      name: "market",
      symbol: "market",
      uri: "https://arweave.net/123",
      sellerFeeBasisPoints: percentAmount(5.5),
      collectionDetails: { __kind: "V1", size: 10 },
      isCollection: true,
      tokenOwner: publicKey(creator.publicKey),
    }).sendAndConfirm(umi);
    console.log(
      `Created Collection NFT: ${collectionMint.publicKey.toString()}`
    );

    // Mint NFT into maker's ATA
    await createNft(umi, {
      mint: nftMint,
      name: "market",
      symbol: "market",
      uri: "https://arweave.net/123",
      sellerFeeBasisPoints: percentAmount(5.5),
      collection: { verified: false, key: collectionMint.publicKey },
      tokenOwner: publicKey(seller.publicKey),
    }).sendAndConfirm(umi);
    console.log(`Created NFT: ${nftMint.publicKey.toString()}`);

    // Verify Collection
    const collectionMetadata = findMetadataPda(umi, {
      mint: collectionMint.publicKey,
    });

    const collectionMasterEdition = findMasterEditionPda(umi, {
      mint: collectionMint.publicKey,
    });

    const nftMetadata = findMetadataPda(umi, { mint: nftMint.publicKey });

    await verifySizedCollectionItem(umi, {
      metadata: nftMetadata,
      collectionAuthority: creator,
      collectionMint: collectionMint.publicKey,
      collection: collectionMetadata,
      collectionMasterEditionAccount: collectionMasterEdition,
    }).sendAndConfirm(umi);

    console.log("Collection NFT Verified!");

    [marketplacePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"), Buffer.from(MARKETPLACE_NAME)],
      program.programId
    );

    [treasuryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), marketplacePda.toBuffer()],
      program.programId
    );

    [listingPda] = PublicKey.findProgramAddressSync(
      [
        marketplacePda.toBuffer(),
        new anchor.web3.PublicKey(nftMint.publicKey).toBuffer(),
      ],
      program.programId
    );

    [rewardMintPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("reward"), marketplacePda.toBuffer()],
      program.programId
    );

    sellerAta = await createAssociatedTokenAccount(
      provider.connection,
      seller,
      new anchor.web3.PublicKey(nftMint.publicKey),
      seller.publicKey
    );

    buyerAta = await createAssociatedTokenAccount(
      provider.connection,
      buyer,
      new anchor.web3.PublicKey(nftMint.publicKey),
      buyer.publicKey
    );
  });

  const program = anchor.workspace.marketplace as Program<Marketplace>;

  it("Should initiate marketplace!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initMarketplace(MARKETPLACE_NAME, 20)
      .accountsPartial({
        admin: admin.publicKey,
        marketplace: marketplacePda,
        rewardMint: rewardMintPda,
        treasury: treasuryPda,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const marketplaceAccount = await program.account.marketplace.fetch(
      marketplacePda
    );
    console.log(marketplaceAccount);
    assert.equal(
      marketplaceAccount.name,
      MARKETPLACE_NAME,
      "Marketplace name mismatch"
    );
    assert.equal(marketplaceAccount.fee, 20, "Marketplace fee mismatch");
    assert.ok(
      marketplaceAccount.admin.equals(admin.publicKey),
      "Marketplace admin mismatch"
    );
    const rewardMintAccount = await getMint(provider.connection, rewardMintPda);
    assert.equal(
      rewardMintAccount.decimals,
      6,
      "Reward mint decimals mismatch"
    );
    assert.ok(
      rewardMintAccount.mintAuthority?.equals(admin.publicKey),
      "Reward mint authority mismatch"
    );
    assert.ok(rewardMintAccount.isInitialized, "Reward mint not initialized");
  });

  it("Should initiate listing!", async () => {
    // getting the metadata and the edition details
    const nftMetadata = findMetadataPda(umi, { mint: nftMint.publicKey });
    const nftEdition = findMasterEditionPda(umi, { mint: nftMint.publicKey });

    const tx = await program.methods
      .listing(new BN(1e6))
      .accountsPartial({
        seller: seller.publicKey,
        sellerAta,
        listing: listingPda,
        marketplace: marketplacePda,
        metadata: new anchor.web3.PublicKey(nftMetadata[0]),
        sellerMint: nftMint.publicKey,
        collectionMint: collectionMint.publicKey,
        edition: new anchor.web3.PublicKey(nftEdition[0]),
        metadataProgram,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature:", tx);

    const listing = await program.account.listing.fetch(listingPda);
    console.log("Listing account details", listing);
  });
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
  console.log("Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}
