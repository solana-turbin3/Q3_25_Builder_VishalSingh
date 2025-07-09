import { Keypair } from "@solana/web3.js";

let kp = Keypair.generate();

console.log(`You've generated a new Solana wallet:\n${kp.publicKey.toBase58()}`);

console.log("\nTo save your wallet, copy and paste the output of the following into a JSON file:");
console.log(`[${kp.secretKey.toString()}]`);

