import {
  Transaction,
  SystemProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  PublicKey
} from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));

const to = new PublicKey("CQmUNnGcPtJyJKyfvWw8LJF8mgpBjY92nYcToditBM56");

// Connect to Solana devnet
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: LAMPORTS_PER_SOL / 100 // 0.01 SOL
      })
    );

    transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    transaction.feePayer = from.publicKey;

    const signature = await sendAndConfirmTransaction(connection, transaction, [from]);

    console.log("✅ Transfer successful!");
    console.log(`🔗 View on Explorer: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error("❌ Transfer failed:", e);
  }
})();