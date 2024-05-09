# Devol Accounts Kit

### Table of content

- [Smart Contract](#smart-contract)
    - [Structure of Smart Contract](#structure-of-smart-contract)
    - [Types of Accounts](#types-of-accounts)
    - [Account Structure](#account-structure)
    - [Data Fields](#data-fields)
    - [General Workflow](#general-workflow)
- [Devol Accounts Kit Library](#devol-accounts-kit-library)
    - [Devol Accounts Kit Features](#devol-accounts-kit-features)

    - [DvlClient](#dvlclient)
        - [DvlClient Methods](#dvlclient-methods)
        - [Account Readers](#account-readers)
        - [Handling Errors and Optimizing RPC Calls](#handling-errors-and-optimizing-rpc-calls)
        - [Reading Parameters](#reading-parameters)
        - [Building Transactions to the Smart Contract](#building-transactions-to-the-smart-contract)
            - [Custom User Transactions](#custom-user-transactions)
        - [Transaction Constructor](#transaction-constructor)
        - [Using the `as_transaction_instruction` Method](#using-the-as_transaction_instruction-method)
        - [Sending the Transaction](#sending-the-transaction)

## Introduction

Welcome to the Devol Accounts Kit documentation. This guide provides a comprehensive overview of the functionalities and
structure of the Devol smart contract system. Here, you'll find detailed descriptions of the account types, methods for
interacting with the smart contract, and guidelines on constructing and sending transactions effectively. Whether you're
a developer looking to integrate Devol into your applications or an administrator managing backend operations, this
documentation will serve as your primary resource for understanding and utilizing the Devol platform efficiently.

## Smart Contract:

## Structure of Smart Contract

A smart contract represents a structure of connected accounts (you can think of this as separate database tables) where
each is derived through a specific algorithm based on parameters such as the smart contract's address, seed (a unique
specifier of account type), int_seed (a unique specifier for a specific branch of the smart contract), and in some
cases, the client's public key. Thus, accounts can be divided into two large groups: those specific to the smart
contract and those specific to the client. All accounts specific to the smart contract can be 'unveiled' from the tree
of accounts, where the root account will be the root-account. Client-specific accounts are constructed from the client's
wallet address.

## Types of Accounts

Accounts are divided into two major groups based on the key from which they are derived (PDA):

### Client-Specific (derived from the client's public key):

- **ClientAccount**: The main account of the client, containing information about the pools in which the client
  participates, purchased options, trading volume, and so forth.
- **PayoffLogAccount**: Information about all the payoffs for a specific client.

### Smart Contract-Specific (derived from the smart contract address):

Nesting is similar in smart contract.

- **RootAccount**: The root account, containing addresses of underlying principal accounts.

    - **OraclesAccount**: Information about oracles (prices of BTC, ETH, etc. in USDC or other currencies) used on the
      platform.

    - **MintsAccount**: Information about the base currencies supported by the platform.

        - **MintLogAccount**: Historical information about currency changes on the platform.

    - **WorkerAccount**: An entity that manages the trading pool, stores the distribution of strikes, their changing
      prices,
      range boundaries, and more.
        - **PoolsLogAccount**: Information about transactions in the current pool (expiries), a ring buffer that
          contains a
          limited number of the most recent transactions.
        - **PoolsTraceAccount**: Historical information about pools, including the settlement price at which they were
          completed.
        - **TasksLogAccount**: Information about the settings of the current pool (expiries), a ring buffer that
          contains a
          limited number of the most recent transactions.
        - **TasksTraceAccount**: Historical information about settings of pools.

    - **InstrumentsAccount**: Information about currency pairs that can be traded on the platform.

## Account Structure

Accounts are structures that can be accessed through pre-defined offsets. They were designed in such a way that they do
not require serialization through tools like Borsh or Anchor. In earlier versions, each parameter was placed at a
pre-determined offset. Currently, this approach is being revised to allow mapping of the data buffer directly to the
structure type on a Little Endian architecture.

All accounts are described in files located in the `src/accounts` folder and end with `_account`.

### AccountHeader

Every account (except for circular log buffers) has an `AccountHeader` used for data verification.

```rust
#[repr(C)]
pub struct AccountHeader {
    pub tag: u32,
    pub version: u32,
    pub root: Pubkey,
}
```

- `tag` indicates the "type" of account (e.g., an account serving pools or a specific type of log account).
- `version` indicates which version of the structure should be mapped to this account.
- `root` stores the public key of the root account to verify that this account is part of the account tree of this
  branch of the smart contract. The exception is the root-account, which stores the platform administrator’s public key
  in this field.

## Data Fields

Each account file contains descriptions of fields with their absolute offsets relative to the data buffer extracted from
the blockchain. This can be useful if you need to read not the entire account, but only a specific field or a nested
structure: in this case, you can make an RPC request to get a data slice with the required field.

```rust
pub const WORKER_ACCOUNT_TAG_OFFSET: usize = 0;
pub const WORKER_ACCOUNT_VERSION_OFFSET: usize = 4;
pub const WORKER_ACCOUNT_ROOT_ADDRESS_OFFSET: usize = 8;
pub const WORKER_ACCOUNT_ID_OFFSET: usize = 40;
pub const WORKER_ACCOUNT_STATE_OFFSET: usize = 44;
// ......
```

However, in most cases, it is more convenient to work directly with the structures:

```rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WorkerAccount {
    pub header: AccountHeader,
    //  40 bytes
    pub id: u32,
    //  4 bytes, WORKER_ACCOUNT_ID_OFFSET
    pub state: WorkerState,
    //  4 bytes, WORKER_ACCOUNT_STATE_OFFSET
    pub task_id: u32,
    //  4 bytes, WORKER_ACCOUNT_TASK_ID_OFFSET
    pub instr_id: u32,                      //  4 bytes, WORKER_ACCOUNT_INSTR_ID_OFFSET
    // ......
}
```

Because the original approach did not anticipate data mapping to structures, many fields ended up being misaligned. We
are working on correcting this situation; however, some fields may still be misaligned, so getters are suggested for
these fields (these data fields are intentionally declared private):

```rust
pub struct WorkerAccount {
    // ......
    pub duration: u32,
    //  4 bytes, WORKER_ACCOUNT_DURATION_OFFSET
    init_px: [u8; 8],
    //  8 bytes, WORKER_ACCOUNT_INIT_PX_OFFSET
    pub own_ps: u32,                        //  4 bytes, WORKER_ACCOUNT_OWN_PS_OFFSET
// ......
    impl WorkerAccount {
    #[inline(always)]
    pub fn get_init_px( & self ) -> i64 { i64::from_ne_bytes( self.init_px)
}
// ......
```

## General Workflow

To start trading, a user must create a client account (ClientAccount). Generally, this happens automatically during the
registration process on the platform via the frontend. Technically, it is possible to do this independently, but the
current SDK does not provide this functionality. As a result of registration, the user receives either a fully
functional (Power) or a limited (Light) account. The only difference between them is that the Power account has no daily
trading volume limits, while the Light account has a maximum trading volume of 650 USDC per day.

The next step is to transfer funds to the platform. All trading operations are conducted only with funds that have been
deposited into the system. After transferring funds, you can start trading either as a trader or a liquidity provider.

You can withdraw free funds (not invested in pool tokens, options, and not pledged as collateral) from the platform at
any time.

## Devol Accounts Kit Library:

The library provides high-level wrappers for convenient reading of accounts and composing transactions to the smart
contract.

## Devol Accounts Kit Features:

The Devol Accounts Kit is equipped with a set of features that cater to various use cases in blockchain application
development. The library is structured with a default feature set, which compiles the entire library including all
functionalities necessary for typical applications. Additionally, there is a specific feature designed for blockchain
environments:

- ### On-chain Feature
  The `on-chain` feature is specifically designed for scenarios where the network-dependent functionalities are not
  required. When this feature is enabled, it removes all network-related parts of the library, making it suitable for
  deployment within smart contracts themselves. This feature ensures that the library is optimized for on-chain
  operations, reducing the footprint and enhancing performance for smart contract execution.

## DvlClient

The most important class for low-level operations with the Devol smart contract is `DvlClient`. It is initialized with
the following fields:

- `rpc_client` - standard RPC client from the Solana library
- `int_seed` - unique branch number of the smart contract (for mainnet, this is 1)
- `admin_public_key` - public key of the administrator (for mainnet, this is
  7t13RqNfLcQMtdsGfo5rNoRE5REJyLzHbX8QuqEURbbJ)
- `program_id` - address of the smart contract (for mainnet - DVL8i4TR4TxtPwpvK4UyVrxc7VWWRtRG74XmKQ6xXpfq)

Based on the provided data, the client can determine which specific smart contract it is interacting with and will allow
reading accounts knowing only their type and sending transactions.

The general setup looks like this:

```rust
let rpc_client = RpcClient::new(String::from(RPC_URL));
let dvl_client = DvlClient::new(rpc_client, INT_SEED, ADMIN_PUBLIC_KEY, PROGRAM_ID);
```

## DvlClient Methods

DvlClient offers three methods for working with accounts:

- `get_account` - Makes a call to RPC, retrieves the account of the requested type, checks the data for correctness, and
  if all is well, returns the retrieved structure. If the required account depends on an upstream account in the account
  tree, recursive reading is performed. Thus, this method makes from one to several calls to RPC depending on the "
  depth" of the account.

- `get_account_by_public_key` - Does the same as `get_account`, but instead of recursive reading, it always makes
  exactly one request to RPC.

- `account_public_key` - Returns the public key of the required account without reading its data. It works similarly
  to `get_account` in terms of the number of RPC calls but makes one less read since the target account is not
  retrieved.

### Method for Sending Transactions:

- `send_transaction` - Based on parameters specific to the transaction, it prepares and sends it via `rpc_client`. The
  method is provided "as it is", and if it does not suit your needs, you may use your own. In the current
  implementation,
  the method accepts an object of type `DvlSendTransactionParams` with the following mandatory parameters:
    - `instructions` - a set of instructions
    - `signer` - public key of the signer
    - `signer_fn` - a function to sign the transaction before sending (we do not request the public key, only the
      function for signing)

Other parameters are optional and should be obvious from the context.

## Account Readers

### How to Read an Account

To read accounts, readers are used - these are a set of abstractions that ensure the account being read is in the
correct state, has the correct version, the right size, etc. To utilize a reader, you need an instance of a configured
DvlClient.

For example, reading the root account can be done in the following way:

```rust
let root_account = client.get_account::<RootAccount>(()) ?;
```

## Handling Errors and Optimizing RPC Calls

Note that if the operation fails, you will receive an error that can be printed and must be handled according to the
logic of your program. You can save the public key of an account and subsequently use it to read by the public key to
reduce the number of RPC calls and increase response speed:

```rust
let instruments_account_pubkey = root_account.instruments_address;
let instruments_account = dvl_client.get_account_by_public_key::<InstrumentsAccount>( & instruments_account_pubkey) ?;
```

## Reading Parameters

To read certain accounts, no parameters are required (as in the example with the root account), so an empty object must
be passed to the read function. However, some accounts, such as the `worker_account`, require an index to be passed for
reading (since there are many such accounts, the index serves as an additional element in locating a specific account):

```rust
let worker_7 = client.get_account::<WorkerAccount>(DvlIndexParam { id: 7 }) ?;
```

## Reading Parameters

When attempting to read a specific account, it is necessary to check which type of parameters are required for its
retrieval. Possible options include:

- `()` - No parameters are required.
- `DvlIndexParam` - The index of the account is required.
- `DvlClientParam` - Required for reading the Payoff log from a client account.
- `DvlClientParams` - Parameters needed for reading a client account.

## Building Transactions to the Smart Contract

In the `src/instructions_data` folder, there are structures needed to fill the buffer for specific transactions, and in
the `src/instructions_data/instructions.rs` file, there is a list of available smart contract instructions. However,
this is not the most convenient way to compile instructions, so the library provides constructors for instructions and
transactions.

Since this SDK is common for the smart contract, client applications, frontend, and administrative tools, you can view
all transactions available on the platform. However, you can only execute custom user transactions.

### Custom User Transactions

#### InstructionOptionTrade

Option trading or bucket-based trading. In this instruction, you need to specify the stake size in each bucket (positive
number for buying, negative for selling). Optionally, you can also specify:

- Basket (from zero to 4 positions): if specified, this deal will be reflected in the portfolio on the website (option
  trading); if not specified, it will not be (bucket trading).
- Maximum cost of the transaction (the final price may vary slightly at the time of the actual transaction).

#### InstructionLpTrade

Buying or selling pool tokens (participating in the pool as a liquidity provider). The only parameter is the amount of
pool tokens being bought or sold (positive number for buying, negative for selling).

#### InstructionPayoff

Receiving payout upon the expiration of the pool. Parameters:

- `worker_id`: the number of the worker account servicing the pool.
- `pool_id`: the sequential number of the pool (expiration). One worker services consecutive pools, so it is necessary
  to specify the pool number as well.

#### InstructionTransferToken

Transferring money for trading on the platform.

- `mint_id`: unique number of the currency according to MintsAccount (base currency).
- `amount`: quantity.

#### InstructionWithdrawToken

Withdrawing funds from the platform.

- `mint_id`: unique number of the currency according to MintsAccount (base currency).
- `amount`: quantity.

### Instruction Constructor (Data Fields Only)

Similar to reading an account, the library provides an abstract factory for constructing the data buffer of the required
instruction, requesting parameters specific to that instruction. There is no need to fill all the data fields yourself -
just prepare the specific instruction parameters and call the factory:

```rust
let trade_params = OptionTradeParams {
trade_qty: [0; BUCKETS_COUNT],
basket: None,
max_cost: None,
};
let instruction_data = DvlInstruction::new::<InstructionOptionTrade>(trade_params) ?;
```

## Transaction Constructor

Once you have prepared the instruction data, you can either use them to compose the instruction yourself by serializing
them using the built-in method `to_vec_le`:

```rust
let serialized_data = instruction_data.to_vec_le();
```

## Preparing the Transaction

After which, you should prepare the transaction.

However, the recommended method is using the `as_transaction_instruction`. Let's consider a complete basic example of
constructing a transaction from the moment of creating a `DvlClient` instance to getting a transaction ready for
sending:

```rust
// Construct Devol client:
let rpc_client = RpcClient::new(String::from(RPC_URL));
let dvl_client = DvlClient::new(rpc_client, INT_SEED, ADMIN_PUBLIC_KEY, PROGRAM_ID);
// Construct instruction data:
let trade_params = OptionTradeParams {
trade_qty: [0; BUCKETS_COUNT],
basket: None,
max_cost: None,
};
let instruction_data = DvlInstruction::new::<InstructionOptionTrade>(trade_params) ?;
// Get signer Keypair object
let signer_kp = Keypair::from_base58_string(SIGNER_PRIVATE_KEY);
// Construct trade params
let trade_params = OptionTradeDebugTransactionParams {
client_key: signer_kp.pubkey(),
worker_id: id_worker,
};
let instruction = instruction_data.as_transaction_instruction(dvl_client, & signer_kp, trade_params) ?;
```

## Using the `as_transaction_instruction` Method

The `as_transaction_instruction` method is available on any object derived from `DvlInstruction`. It takes `DvlClient`
as an argument to specify the transaction with the correct account keys, as these keys are PDAs (Program Derived
Addresses) and are linked to a specific branch of the smart contract.

## Sending the Transaction

Once you have the instruction, you need to add it to the final transaction and send it via the RPC client (you can use
the one passed to `DvlClient`). If desired, you can use the built-in method for sending transactions, but it is
provided "as is," and most likely, you will want to write your own:

```rust
let mut instructions = Vec::<Instruction>::new();
instructions.push( * instruction);
let signature = client.send_transaction(DvlSendTransactionParams {
instructions,
signer_kp,
commitment_config: Some(CommitmentConfig::finalized()),
compute_budget: Some(1400000),
compute_unit_price: Some(150),
verbose: Some(true),
max_retries: Some(5),
log_prefix: Some("[TRADE_TRANSACTION] "),
retry_delay: None,
}) ?;
```