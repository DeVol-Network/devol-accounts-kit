# Devol Accounts Kit

### Table of content

- [Introduction](#introduction)
- [Quick Start Guide for Devol Accounts Kit](#quick-start-guide-for-devol-accounts-kit)
    - [Setting Up Your Development Environment](#setting-up-your-development-environment)
    - [Initialize Your Project](#initialize-your-project)
    - [Add Dependencies](#add-dependencies)
    - [Sample Code to Interact with Devol Smart Contracts](#sample-code-to-interact-with-devol-smart-contracts)
    - [Run Your Application](#run-your-application)
- [Advanced](#advanced)
    - [Feature Configuration Options](#feature-configuration-options)

## Introduction

Welcome to the Devol Accounts Kit documentation. This comprehensive guide is designed to equip developers and system
administrators with the knowledge and tools necessary to effectively implement and manage the Devol smart contract
system.

## Quick Start

### Setting Up Your Development Environment

To begin, ensure your development environment is set up with the necessary tools and libraries:

1. **Install Rust:**
   Rust is essential for Solana development. Follow the official guide to install Rust on your system:
   [Install Rust](https://www.rust-lang.org/tools/install).

2. **Install the Solana CLI:**
   The Solana Command Line Tool is crucial for interacting with the Solana blockchain. Install it by following these
   instructions:
   [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools).

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
solana-client = "^1.18.3"
solana-sdk = "^1.18.3"
```

### Sample Code to Interact with Devol Smart Contracts

Below is a basic example to set up your client and interact with the Devol smart contracts:

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

fn main() {
    let rpc_url = String::from("https://api.mainnet-beta.solana.com/");
    let rpc_client = RpcClient::new(rpc_url);

    let admin_pub_key = Pubkey::from_str("ADMIN_PUBLIC_KEY").unwrap();
    let program_id = Pubkey::from_str("PROGRAM_ID").unwrap();

    let reader = DvlClient::new(rpc_client, 1, admin_pub_key, program_id);

    // Example: Fetching and printing the RootAccount
    let root_account = reader.get_account::<RootAccount>(()).expect("Failed to fetch the RootAccount");
    println!("Root Account: {:?}", root_account);
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
