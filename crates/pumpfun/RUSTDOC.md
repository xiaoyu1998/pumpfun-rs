Pump.fun Solana Program SDK

This library provides a Rust interface for interacting with the Pump.fun Solana program.
Pump.fun is a Solana-based marketplace that enables users to create and distribute their own tokens, primarily memecoins.

# Getting Started

Add this crate to your project using cargo:

```sh
cargo add pumpfun
```

# Usage

The main entry point is the `PumpFun` struct which provides methods for interacting with the program:

> **Important:** You must create an Associated Token Account (ATA) for your wallet before buying tokens. This is required to receive the purchased tokens.

> **Coming Soon:** Automatic ATA creation will be added in a future release to streamline the buying process.

For instructions on creating an ATA, see the [Solana documentation](https://spl.solana.com/associated-token-account).

```rust,no_run
use anchor_client::{
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        transaction::Transaction,
    },
    Cluster,
};
use anchor_spl::{
    associated_token::{
        get_associated_token_address,
        spl_associated_token_account::instruction::create_associated_token_account,
    },
    token::spl_token,
};
use pumpfun::{accounts::BondingCurveAccount, utils::CreateTokenMetadata, PumpFun};

// Create a new PumpFun client
let payer: Keypair = Keypair::new();
let client: PumpFun<'_> = PumpFun::new(Cluster::Mainnet, &payer, None, None);

// Mint keypair
let mint: Keypair = Keypair::new();

// Create a new token
let metadata: CreateTokenMetadata = CreateTokenMetadata {
    name: "Lorem ipsum".to_string(),
    symbol: "LIP".to_string(),
    description: "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Quam, nisi.".to_string(),
    file: "/path/to/image.png".to_string(),
    twitter: None,
    telegram: None,
    website: Some("https://example.com".to_string()),
};
let signature: Signature = client.create(&mint, metadata).await?;
println!("Created token: {}", signature);

// Print the curve
let curve: BondingCurveAccount = client.get_bonding_curve_account(&mint.pubkey())?;
println!("{:?}", curve);

// Create an ATA
let ata: Pubkey = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
let tx: Transaction = Transaction::new_signed_with_payer(
    &[create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &spl_token::id(),
    )],
    Some(&payer.pubkey()),
    &[&payer],
    client.rpc.get_latest_blockhash().unwrap(),
);
let signature: Signature = client.rpc.send_and_confirm_transaction(&tx)?;
println!("ATA: {:?}, Signature: {:?}", ata, signature);

// Print amount of SOL, LAMPORTS, and TOKENS
let amount_sol: u64 = 1;
let amount_lamports: u64 = LAMPORTS_PER_SOL * amount_sol;
let amount_token: u64 = curve.get_buy_price(amount_lamports)?;
println!("Amount in SOL: {}", amount_sol);
println!("Amount in LAMPORTS: {}", amount_lamports);
println!("Amount in TOKENS: {}", amount_token);

// Buy tokens
let signature: Signature = client.buy(&mint.pubkey(), amount_lamports, Some(500)).await?;
println!("Bought tokens: {}", signature);

// Sell tokens
let signature: Signature = client.sell(&mint.pubkey(), amount_token, Some(500)).await?;
println!("Sold tokens: {}", signature);
```

# Features

- Create new tokens with metadata
- Buy tokens using SOL
- Sell tokens for SOL
- Query bonding curve and global state
- Calculate prices and slippage

# Architecture

The SDK is organized into several modules:

- `accounts`: Account structs for deserializing on-chain state
- `constants`: Program constants like seeds and public keys
- `error`: Custom error types for error handling
- `utils`: Helper functions and utilities

The main `PumpFun` struct provides high-level methods that abstract away the complexity of:

- Managing Program Derived Addresses (PDAs)
- Constructing and signing transactions
- Handling account lookups and deserialization
- Calculating prices and slippage
