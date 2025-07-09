import bs58 from 'bs58';
// @ts-ignore
import promptSync from 'prompt-sync';
const prompt = promptSync();

function base58ToWallet() {
  const base58 = prompt("Enter your Phantom private key (Base58): ");
  const secretKey = bs58.decode(base58);
  console.log("\n🔑 Your wallet array for dev-wallet.json:\n");
  console.log(JSON.stringify(Array .from(secretKey)));
}

function walletToBase58() {
  const walletArrayStr = prompt("Paste your dev-wallet.json array:\n");
  const walletArray = JSON.parse(walletArrayStr);
  const base58 = bs58.encode(new Uint8Array(walletArray));
  console.log("\n🔐 Your Phantom Base58 private key:\n");
  console.log(base58);
}

const mode = prompt("Choose mode (1 = Base58 → Wallet, 2 = Wallet → Base58): ");

if (mode === "1") base58ToWallet();
else if (mode === "2") walletToBase58();
else console.log("❌ Invalid option.");