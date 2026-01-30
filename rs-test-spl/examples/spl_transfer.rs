use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    transaction::Transaction,
    signer::keypair,
};
use solana_system_interface::instruction::create_account;
use spl_associated_token_account_interface::{
  instruction::create_associated_token_account,address::get_associated_token_address_with_program_id
};
use spl_token_interface::{
    id as token_program_id,
    instruction::{transfer_checked, initialize_mint, mint_to},
    state::Mint,

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

    let to_keypair = Keypair::new();


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


    let from_ata = get_associated_token_address_with_program_id(
        &signer.pubkey(),
        &mint.pubkey(),
        &token_program_id()
    );

    let to_ata = get_associated_token_address_with_program_id(
        &to_keypair.pubkey(),
        &mint.pubkey(),
        &token_program_id()
    );

    let from_create_ata_instruction = create_associated_token_account(
        &signer.pubkey(),
        &signer.pubkey(),
        &mint.pubkey(),
        &token_program_id()
    );

    let to_create_ata_instruction = create_associated_token_account(
        &signer.pubkey(),
        &to_keypair.pubkey(),
        &mint.pubkey(),
        &token_program_id()
    );

    let transaction = Transaction::new_signed_with_payer(
        &[from_create_ata_instruction, to_create_ata_instruction],
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;
    println!("Transaction signature: {}", transaction_signature);
    println!("Associated token address: {}", from_ata);
    println!("Associated token address: {}", to_ata);

    let amount_to_mint = 10000000000; //9 decimal places = 10 tokens

    let mint_to_instruction = mint_to(
        &token_program_id(),
        &mint.pubkey(),
        &from_ata,
        &signer.pubkey(),
        &[&signer.pubkey()],
        amount_to_mint,
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_instruction],
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );


    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;
    println!("Transaction signature: {}", transaction_signature);
    println!("Amount minted: {}", amount_to_mint);


    let amount_to_transfer = 1000000000; //9 decimal places = 1 token


    let transfer_instruction = transfer_checked(
        &token_program_id(),
        &from_ata,
        &mint.pubkey(),
        &to_ata,
        &signer.pubkey(),
        &[&signer.pubkey()],
        amount_to_transfer,
        9
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;
    println!("Transaction signature: {}", transaction_signature);
    println!("Amount transferred: {}", amount_to_transfer);


    Ok(())
}
