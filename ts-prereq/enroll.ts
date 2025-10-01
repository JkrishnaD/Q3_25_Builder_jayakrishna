import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

const MPL_CORE_PROGRAM_ID = new PublicKey(
    "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
);

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed",
});

const program: Program<Turbin3Prereq> = new Program(IDL, provider);

const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];

const [account_key, account_bump] = PublicKey.findProgramAddressSync(
    account_seeds,
    program.programId
);

const mintCollection = new PublicKey(
    "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2"
);

const mintTs = Keypair.generate(); // check this once as this need to be mintcollection key

const authority = PublicKey.findProgramAddressSync(
    [Buffer.from("collection"), mintCollection.toBuffer()],
    program.programId
)[0];

async function close() {
    try {
        const tx = await program.methods
            .close()
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .signers([keypair])
            .rpc();
        console.log(
            `Success! Check out your TX here: https://explorer.solana.com/tx/${tx}?cluster=devnet`
        );
    } catch (error) {
        console.error(`Oops something went wrong: ${error}`);
    }
}
// close();
async function enroll() {
    try {
        const tx = await program.methods
            .initialize("JKrishnaD")
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .signers([keypair])
            .rpc();

        console.log(`Success! Check out your TX here:
                https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    } catch (error) {
        console.log(`Oops something went wrong: ${error}`);
    }
}

// enroll();

async function submit() {
    try {
        const tx = await program.methods.submitTs()
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                collection: mintCollection,
                authority,
                mint: mintTs.publicKey,
                mpl_core_program: MPL_CORE_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .signers([keypair, mintTs])
            .rpc();
        console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    } catch (error) {
        console.error(`Oops, something went wrong: ${error}`);
    }
}

submit();
