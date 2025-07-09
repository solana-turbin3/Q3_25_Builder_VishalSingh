import {
  Connection,
  Keypair,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  PublicKey
} from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));

const to = new PublicKey("CQmUNnGcPtJyJKyfvWw8LJF8mgpBjY92nYcToditBM56");

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    const balance = await connection.getBalance(from.publicKey);

    const tx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance
      })
    );

    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    tx.feePayer = from.publicKey;

    // Estimate transaction fee
    const fee = (await connection.getFeeForMessage(tx.compileMessage(), "confirmed")).value || 0;

    tx.instructions.pop(); 
    tx.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee
      })
    );

    const sig = await sendAndConfirmTransaction(connection, tx, [from]);

    console.log("✅ Wallet emptied!");
    console.log(`🔗 View TX: https://explorer.solana.com/tx/${sig}?cluster=devnet`);
  } catch (e) {
    console.error("❌ Failed to empty wallet:", e);
  }
})();