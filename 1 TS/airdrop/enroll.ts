import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { Program, AnchorProvider, Wallet, Idl } from "@coral-xyz/anchor";
import { IDL } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const program = new Program(IDL as Idl, provider);

const [accountKey] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
  ],
  new PublicKey((IDL as any).address)
);

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

// 🪙 Create a new mint keypair (for NFT)
const mintTs = Keypair.generate();


// Step 1: Initialize your on-chain enrollment account

// (async () => {
//   try {
//     const txhash = await program.methods
//       .initialize("geeekyvishal") // your GitHub handle here
//       .accounts({
//         user: keypair.publicKey,
//         account: accountKey,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([keypair])
//       .rpc();

//     console.log(
//       `✅ initialize() success: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
//     );
//   } catch (e) {
//     console.error("❌ initialize() failed:", e);
//   }
// })();


// Step 2: Submit  TS prerequisites & mint NFT

(async () => {
  try {
    const txhash = await program.methods
      .submitTs()
      .accounts({
        user: keypair.publicKey,
        account: accountKey,
        mint: mintTs.publicKey,
        collection: mintCollection,
        authority: PublicKey.findProgramAddressSync(
          [Buffer.from("collection"), mintCollection.toBuffer()],
          new PublicKey((IDL as any).address)
        )[0],
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair, mintTs])
      .rpc();

    console.log(
      `✅ submitTs() success: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
    );
  } catch (e) {
    console.error("❌ submitTs() failed:", e);
  }
})();