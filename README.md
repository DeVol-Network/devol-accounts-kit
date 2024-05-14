# Devol Accounts Kit

### Table of Contents

- [Introduction](#introduction)
- [Quick Start Guide](#quick-start-guide)
    - [Setting Up Your Development Environment](#setting-up-your-development-environment)
    - [Initialize Your Project](#initialize-your-project)
    - [Add Dependencies](#add-dependencies)
    - [Sample Code to Interact with Devol Smart Contracts](#sample-code-to-interact-with-devol-smart-contracts)
    - [Run Your Application](#run-your-application)
- [Advanced](#advanced)
    - [Feature Configuration Options](#feature-configuration-options)
- [Documentation](#documentation)

## Introduction

Welcome to the Devol Accounts Kit documentation. This comprehensive guide is designed to equip developers and system
administrators with the knowledge and tools necessary to effectively implement and manage the Devol smart contract
system.

## Quick Start Guide

### Setting Up Your Development Environment

To begin, ensure your development environment is set up with the necessary tools and libraries:

1. **Rust**
2. **Solana CLI**

### Initialize Your Project

Create a new Rust project where you will integrate the Devol Accounts Kit:

```bash
cargo new devol_integration
cd devol_integration
```

### Add Dependencies

Open your `Cargo.toml` file and add the necessary dependencies for working with the Solana blockchain and the Devol
Accounts Kit:

```toml
[dependencies]
devol-accounts-kit = { git = "https://github.com/DeVol-Network/devol-accounts-kit.git", tag = "0.2.1" }
```

### Sample Code to Interact with Devol Smart Contracts

Below is a basic example to set up your client and interact with the Devol smart contracts:

```rust
// Required imports
use std::str::FromStr;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use devol_accounts_kit::dvl_client::dvl_client::DvlClient;
use devol_accounts_kit::accounts::root::root_account::RootAccount;
use tokio; // Async runtime for handling non-blocking operations

#[tokio::main] // This attribute sets up the Tokio runtime for your async main
async fn main() {
    let rpc_url = String::from("https://api.mainnet-beta.solana.com/");
    let rpc_client = RpcClient::new(rpc_url);

    // Proper error handling with `match` for key parsing
    let admin_pub_key = match Pubkey::from_str("ADMIN_PUBLIC_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Failed to parse admin public key: {}", e);
            return;
        }
    };

    let program_id = match Pubkey::from_str("PROGRAM_ID") {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to parse program ID: {}", e);
            return;
        }
    };

    let reader = DvlClient::new(rpc_client, 1, admin_pub_key, program_id);

    // Handle asynchronous operations with await and proper error handling
    match reader.get_account::<RootAccount>(()).await {
        Ok(root_account) => println!("Root Account: {:?}", root_account),
        Err(e) => eprintln!("Failed to fetch the RootAccount: {}", e),
    };
}
```

### Run Your Application

To run your application and see the output:

```bash
cargo run
```

This simple setup will help you start interacting with the Devol smart contract and explore more complex transactions
and interactions as you progress in your development.

## Advanced

## Feature Configuration Options:

The Devol Accounts Kit is equipped with a set of features that cater to various use cases in blockchain application
development. The library is structured with a default feature set, which compiles the entire library including all
functionalities necessary for typical applications. Additionally, there is a specific feature designed for blockchain
environments:

- ### On-chain Feature
  The `on-chain` feature is specifically designed for scenarios where the network-dependent functionalities are not
  required. When this feature is enabled, it removes all network-related parts of the library, making it suitable for
  deployment within smart contracts themselves. This feature ensures that the library is optimized for on-chain
  operations, reducing the footprint and enhancing performance for smart contract execution.

## Documentation

For more detailed guidance [DeVol Network Documentation](https://sdk.devol.network)
