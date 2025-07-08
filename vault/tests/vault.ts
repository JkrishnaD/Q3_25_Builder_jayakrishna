import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { expect } from "chai";

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vault as Program<Vault>;

  const user = provider.wallet;

  const accountSpace = 8 + 1 + 1; // discriminator + vault_bump + state_bump

  let rentExemptLamports: number;

  before(async () => {
    rentExemptLamports = await provider.connection.getMinimumBalanceForRentExemption(0);

    console.log(`Rent-exempt amount for ${accountSpace} bytes:`, rentExemptLamports / anchor.web3.LAMPORTS_PER_SOL, "SOL");
  })

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

    // as the account initiated we need to have rent in the account
    expect(account_info).not.to.be.null
    expect(account_info.lamports).to.be.equal(rentExemptLamports)
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

    const expected_amounte = rentExemptLamports + 2 * anchor.web3.LAMPORTS_PER_SOL;
    // expected amount is to be 2 sol we added and the rent amount
    expect(account_info.lamports).to.be.equal(expected_amounte)
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

    // as the 2 sol is withdrawn we have just the rent excempt sol
    expect(account_info.lamports).to.be.equal(rentExemptLamports);
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

    //here the account is closed so there will be not account which is null
    expect(account_info).to.be.null
  })
});
