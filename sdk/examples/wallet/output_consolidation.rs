// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will consolidate basic outputs from an account with only an AddressUnlockCondition by sending
//! them to the same address again.
//!
//! Make sure that `example.stronghold` and `example.walletdb` already exist by
//! running the `create_account` example!
//!
//! Rename `.env.example` to `.env` first, then run the command:
//! ```sh
//! cargo run --release --all-features --example output_consolidation
//! ```

use std::env::var;

use iota_sdk::{types::block::address::ToBech32Ext, wallet::Result, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses secrets in environment variables for simplicity which should not be done in production.
    dotenvy::dotenv().ok();

    let wallet = Wallet::builder()
        .with_storage_path(&var("WALLET_DB_PATH").unwrap())
        .finish()
        .await?;
    let account = wallet.get_account("Alice").await?;

    // Set the stronghold password
    wallet
        .set_stronghold_password(var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    // Sync account to make sure account is updated with outputs from previous examples
    account.sync(None).await?;
    println!("Account synced");

    // List unspent outputs before consolidation.
    // The output we created with example `03_get_funds` and the basic output from `09_mint_native_tokens` have only one
    // unlock condition and it is an `AddressUnlockCondition`, and so they are valid for consolidation. They have the
    // same `AddressUnlockCondition`(the first address of the account), so they will be consolidated into one
    // output.
    let outputs = account.unspent_outputs(None).await?;
    println!("Outputs BEFORE consolidation:");
    outputs.iter().enumerate().for_each(|(i, output_data)| {
        println!("OUTPUT #{i}");
        println!(
            "- address: {:?}\n- amount: {:?}\n- native tokens: {:?}",
            output_data.address.to_bech32_unchecked("rms"),
            output_data.output.amount(),
            output_data.output.native_tokens()
        )
    });

    println!("Sending consolidation transaction...");

    // Consolidate unspent outputs and print the consolidation transaction IDs
    // Set `force` to true to force the consolidation even though the `output_consolidation_threshold` isn't reached
    let transaction = account.consolidate_outputs(true, None).await?;
    println!("Transaction sent: {}", transaction.transaction_id);

    // Wait for the consolidation transaction to get confirmed
    let block_id = account
        .retry_transaction_until_included(&transaction.transaction_id, None, None)
        .await?;
    println!(
        "Transaction included: {}/block/{}",
        var("EXPLORER_URL").unwrap(),
        block_id
    );

    // Sync account
    account.sync(None).await?;
    println!("Account synced");

    // Outputs after consolidation
    let outputs = account.unspent_outputs(None).await?;
    println!("Outputs AFTER consolidation:");
    outputs.iter().enumerate().for_each(|(i, output_data)| {
        println!("OUTPUT #{i}");
        println!(
            "- address: {:?}\n- amount: {:?}\n- native tokens: {:?}",
            output_data.address.to_bech32_unchecked("rms"),
            output_data.output.amount(),
            output_data.output.native_tokens()
        )
    });

    Ok(())
}
