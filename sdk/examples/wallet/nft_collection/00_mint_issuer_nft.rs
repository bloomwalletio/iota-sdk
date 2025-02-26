// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will mint the issuer NFT for the NFT collection.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example mint_issuer_nft
//! ```

use std::env::var;

use iota_sdk::{
    types::block::{
        output::{NftId, Output, OutputId},
        payload::transaction::{TransactionEssence, TransactionId},
    },
    wallet::{Account, MintNftParams, Result},
    Wallet,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    account.sync(None).await?;
    println!("Account synced!");

    // Set the stronghold password
    wallet
        .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Issue the minting transaction and wait for its inclusion
    println!("Sending NFT minting transaction...");
    let nft_mint_params = [MintNftParams::new()
        .with_immutable_metadata(b"This NFT will be the issuer from the awesome NFT collection".to_vec())];
    let transaction = account.mint_nfts(nft_mint_params, None).await?;
    wait_for_inclusion(&transaction.transaction_id, &account).await?;

    let TransactionEssence::Regular(essence) = transaction.payload.essence();
    for (output_index, output) in essence.outputs().iter().enumerate() {
        if let Output::Nft(nft_output) = output {
            // New minted nft id is empty in the output
            if nft_output.nft_id().is_null() {
                let output_id = OutputId::new(transaction.transaction_id, output_index as u16)?;
                let nft_id = NftId::from(&output_id);
                println!("New minted NFT id: {nft_id}");
            }
        }
    }

    Ok(())
}

async fn wait_for_inclusion(transaction_id: &TransactionId, account: &Account) -> Result<()> {
    println!(
        "Transaction sent: {}/transaction/{}",
        var("EXPLORER_URL").unwrap(),
        transaction_id
    );
    // Wait for transaction to get included
    let block_id = account
        .retry_transaction_until_included(transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
        block_id
    );
    Ok(())
}
