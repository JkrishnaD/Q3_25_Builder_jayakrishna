import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vault as Program<Vault>;

  const user = provider.wallet;

  const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const vault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    program.programId
  )[0];
  before(async () => { });

  it("Is account initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accountsPartial({
        user: user.publicKey,
        vault,
        vaultState,
      })
      .signers([])
      .rpc();
    console.log("Your transaction signature", tx);
    const account_info = await provider.connection.getAccountInfo(vault);
    console.log("account info", account_info);
  });

  it("Is money deposited", async () => {
    const tx = await program.methods
      .deposite(new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL))
      .accountsPartial({
        user: user.publicKey,
        vault,
        vaultState,
      })
      .rpc();

    console.log("Your transaction signature", tx);
    const account_info = await provider.connection.getAccountInfo(vault);
    console.log("account info", account_info);
  });

  it("Is money withdrawn", async () => {
    const tx = await program.methods
      .withdraw(new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL))
      .accountsPartial({
        user: user.publicKey,
        vault,
        vaultState
      })
      .rpc();

    console.log("Your transaction signature", tx);
    const account_info = await provider.connection.getAccountInfo(vault);
    console.log("your account info", account_info)
  });

  it("Is account closed", async () => {
    const tx = await program.methods.closeAccount().accountsPartial({
      user: user.publicKey,
      vault,
      vaultState
    }).rpc();
    
    console.log("Your transaction signature", tx);
    const account_info = await provider.connection.getAccountInfo(vault);
    console.log("your account info", account_info)
  })
});
