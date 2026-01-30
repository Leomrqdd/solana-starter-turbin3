use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    program_pack::Pack, signature::{Keypair, Signer}, signer::keypair, transaction::Transaction
};
use solana_system_interface::instruction::create_account;

use spl_token_interface::{
    id as token_program_id,
    instruction::{initialize_mint},
    state::{Mint},
};

const RPC_URL: &str = "https://api.devnet.solana.com";
const LAMPORTS_PER_SOL: u64 = 1000000000;

use std::env;
use std::path;


fn load_default_keypair() -> anyhow::Result<Keypair> {
    let home_path = env::var_os("HOME").unwrap();
    let default_keypair_path = ".config/solana/id.json"; // relative to HOME directory
    let default_keypair_path = path::PathBuf::from(home_path).join(default_keypair_path);

    let default_keypair =
        keypair::read_keypair_file(default_keypair_path).expect("error reading keypair from path");

    println!("loaded keypair address -> {:?}", default_keypair.pubkey());

    Ok(default_keypair)
}


#[tokio::main]
async fn main() -> Result<()> {
    // Create connection to local validator
    let client = RpcClient::new_with_commitment(
        String::from(RPC_URL),
        CommitmentConfig::confirmed(),
    );

    let blockhash = client.get_latest_blockhash().await?;

    let signer = load_default_keypair()?;
    let balance = client.get_balance(&signer.pubkey()).await?;
    println!("Balance: {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);

    let mint = Keypair::new();
    let space = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(space).await?;
    let create_account_instruction = create_account(
        &signer.pubkey(),
        &mint.pubkey(),
        rent,
        space as u64,
        &token_program_id(),
    );

    let initialize_mint_instruction = initialize_mint(
        &token_program_id(),
        &mint.pubkey(),
        &signer.pubkey(),
        None,
        9,
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&signer.pubkey()),
        &[&signer,&mint],
        blockhash,
    );

    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

    println!("Mint address: {}", mint.pubkey());
    println!("Transaction signature: {}", transaction_signature);

    let mint_account = client.get_account(&mint.pubkey()).await?;
    let mint = Mint::unpack(&mint_account.data)?;
    println!("Mint: {:?}", mint);

    Ok(())
}
