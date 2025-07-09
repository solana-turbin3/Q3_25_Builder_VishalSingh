#[cfg(test)]
mod tests {
    // -----------------------------------
    // Step 1.1: Generate Keypair 
    // -----------------------------------
    /*
    use solana_sdk::signature::{Keypair, Signer};
    use std::{fs::File, io::Write};

    #[test]
    fn keygen() {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        let secret = keypair.to_bytes();

        println!("✅ Public Key: {}", pubkey);
        println!("Save this private key to a file (e.g. dev-wallet.json):");
        println!("{:?}", secret);

        let mut file = File::create("dev-wallet.json").expect("Failed to create file");
        write!(file, "{}", serde_json::to_string(&secret).unwrap()).unwrap();
    }
    */

    // -----------------------------------
    // Step 1.3: Phantom <-> Wallet Conversion 
    // -----------------------------------
    /*
    use std::io::{self, BufRead};
    use bs58;

    #[test]
    fn base58_to_wallet() {
        println!("🔐 Enter Phantom base58 private key:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("✅ Wallet format: {:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("🔐 Enter wallet byte array (e.g. [1,2,3,...]):");
        let stdin = io::stdin();
        let input = stdin.lock().lines().next().unwrap().unwrap();
        let wallet = input
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let base58 = bs58::encode(wallet).into_string();
        println!("✅ Phantom-compatible base58: {}", base58);
    }
    */

    // -----------------------------------
    // Step 2: Airdrop SOL 
    // -----------------------------------
    /*
    use solana_sdk::signature::{Keypair, Signer};
    use solana_client::rpc_client::RpcClient;
    use std::{fs, time::Duration, thread};

    #[test]
    fn airdrop() {
        let data = fs::read_to_string("dev-wallet.json").expect("Failed to read dev-wallet.json");
        let secret: Vec<u8> = serde_json::from_str(&data).expect("Invalid JSON");
        let keypair = Keypair::from_bytes(&secret).expect("Invalid key format");

        let client = RpcClient::new("https://api.devnet.solana.com".to_string());
        println!("⏳ Requesting 2 SOL Airdrop to {}", keypair.pubkey());

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000) {
            Ok(sig) => {
                println!("🔁 Awaiting confirmation...");
                thread::sleep(Duration::from_secs(5));
                let balance = client.get_balance(&keypair.pubkey()).expect("Check failed");
                println!("✅ Airdrop Success! Balance: {} SOL", balance as f64 / 1_000_000_000.0);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }
    */

    // -----------------------------------
    // Step 3: Transfer 0.1 SOL to Turbin3 wallet 
    // -----------------------------------
    /*
    use solana_sdk::{
        signature::{Keypair, Signer},
        system_instruction,
        transaction::Transaction,
    };
    use solana_client::rpc_client::RpcClient;
    use std::fs;

    #[test]
    fn transfer_sol() {
        let data = fs::read_to_string("dev-wallet.json").expect("Failed to read dev-wallet.json");
        let secret: Vec<u8> = serde_json::from_str(&data).expect("Invalid JSON");
        let keypair = Keypair::from_bytes(&secret).expect("Invalid key format");

        let client = RpcClient::new("https://api.devnet.solana.com".to_string());
        let to_pubkey = "CQmUNnGcPtJyJKyfvWw8LJF8mgpBjY92nYcToditBM56"
            .parse()
            .expect("Invalid recipient pubkey");

        let blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");

        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &keypair.pubkey(),
                &to_pubkey,
                100_000_000, // 0.1 SOL
            )],
            Some(&keypair.pubkey()),
            &[&keypair],
            blockhash,
        );

        match client.send_and_confirm_transaction(&tx) {
            Ok(sig) => {
                println!("✅ Transfer successful!");
                println!(
                    "🔗 View TX: https://explorer.solana.com/tx/{}?cluster=devnet",
                    sig
                );
            }
            Err(err) => {
                println!("❌ Transfer failed: {}", err);
            }
        }
    }
    */

    // -----------------------------------
    //  Step 4: Empty Wallet 
    // -----------------------------------
    // use solana_sdk::{
    //     signature::{Keypair, Signer},
    //     system_instruction,
    //     transaction::Transaction,
    // };
    // use solana_client::rpc_client::RpcClient;
    // use std::fs;

    // #[test]
    // fn empty_wallet() {
    //     let data = fs::read_to_string("dev-wallet.json").expect("Failed to read dev-wallet.json");
    //     let secret: Vec<u8> = serde_json::from_str(&data).expect("Invalid JSON");
    //     let keypair = Keypair::from_bytes(&secret).expect("Invalid key format");

    //     let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    //     let to_pubkey = "CQmUNnGcPtJyJKyfvWw8LJF8mgpBjY92nYcToditBM56"
    //         .parse()
    //         .expect("Invalid recipient pubkey");

    //     let balance = client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
    //     if balance < 5_000 {
    //         println!("❌ Insufficient balance to empty wallet.");
    //         return;
    //     }

    //     let amount_to_send = balance - 5_000; // Leave some lamports for tx fee
    //     let blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");

    //     let tx = Transaction::new_signed_with_payer(
    //         &[system_instruction::transfer(
    //             &keypair.pubkey(),
    //             &to_pubkey,
    //             amount_to_send,
    //         )],
    //         Some(&keypair.pubkey()),
    //         &[&keypair],
    //         blockhash,
    //     );

    //     match client.send_and_confirm_transaction(&tx) {
    //         Ok(sig) => {
    //             println!("✅ Wallet emptied!");
    //             println!(
    //                 "🔗 View TX: https://explorer.solana.com/tx/{}?cluster=devnet",
    //                 sig
    //             );
    //         }
    //         Err(err) => {
    //             println!("❌ Empty wallet failed: {}", err);
    //         }
    //     }
    // }



    #[cfg(test)]
mod tests {
    // -----------------------------------
    // Step 5: submit_rs 
    // -----------------------------------
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
        instruction::{Instruction, AccountMeta},
        system_program,
    };
    use std::str::FromStr;

    #[test]
    fn submit_rs() {
        //  Step 5.1: Load dev-wallet
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Failed to load dev-wallet");

        //  Step 5.2: Connect to Devnet
        let rpc_client = RpcClient::new("https://api.devnet.solana.com");

        //  Step 5.3: Setup constants
        let turbin3_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();

        // Step 5.3 (cont.): Create mint key
        let mint = Keypair::new();

        //  Step 5.4: Derive PDAs
        let (prereq_pda, _) = Pubkey::find_program_address(
            &[b"prereqs", signer.pubkey().as_ref()],
            &turbin3_program,
        );

        let (authority_pda, _) = Pubkey::find_program_address(
            &[b"collection", collection.as_ref()],
            &turbin3_program,
        );

        //  Step 5.5: Discriminator for submit_rs
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        // ✅ Step 5.6: Define account metas
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),       // user
            AccountMeta::new(prereq_pda, false),           // prereqs PDA
            AccountMeta::new(mint.pubkey(), true),         // mint key
            AccountMeta::new(collection, false),           // collection
            AccountMeta::new_readonly(authority_pda, false), // authority PDA
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program, false),
        ];
        // Step 5.7: Get blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get blockhash");

        //  Step 5.8: Create instruction
        let instruction = Instruction {
            program_id: turbin3_program,
            accounts,
            data,
        };

        //  Step 5.9: Create and sign transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );

        // Step 5.10: Send transaction
        let sig = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("submit_rs Success! View TX here:");
        println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
    }
}


}