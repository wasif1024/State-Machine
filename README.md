# Rust State Machine

A minimal blockchain runtime implementation in Rust, inspired by [Substrate](https://substrate.io/) runtime architecture. This project demonstrates core concepts of a state transition machine with pallet-based modularity, similar to how Polkadot and other Substrate-based chains structure their runtimes.

## Overview

This state machine implements a simplified blockchain runtime featuring:

- **System Pallet**: Manages block numbers and account nonces
- **Balances Pallet**: Handles token balance tracking and transfers
- **Runtime Dispatch**: Extensible call dispatch system for executing extrinsics
- **Block Execution**: Process blocks containing multiple extrinsics with proper validation

The architecture follows Substrate's pallet pattern, where each module (pallet) is configurable through Rust traits, making it easy to compose and extend functionality.

## Features

- 🧩 **Modular Pallet Design**: Configurable runtime modules using trait-based architecture
- 🔄 **State Transition**: Block-based state machine with extrinsic processing
- 💰 **Balance Management**: Token transfer functionality with overflow protection
- 🔢 **Account System**: Nonce tracking for transaction ordering
- ✅ **Type-Safe**: Leverages Rust's type system for compile-time safety

## Architecture

The runtime is composed of pallets that implement `Config` traits, allowing for flexible configuration:

- **System**: Tracks block numbers and account nonces
- **Balances**: Manages token balances and transfers between accounts
- **Runtime**: Orchestrates pallet execution and call dispatch

## Example Usage

```rust
let mut runtime = Runtime::new();
runtime.balances.set_balance(&"alice".to_string(), 100);

let block = types::Block {
    header: types::Header { block_number: 1 },
    extrinsic: vec![
        types::Extrinsic { 
            caller: "alice".to_string(), 
            call: RuntimeCall::Balances(balances::Call::Transfer { 
                to: "bob".to_string(), 
                amount: 30 
            }) 
        },
    ],
};

runtime.execute_block(block).unwrap();
```

## Building

```bash
cargo build
cargo test
```

## License

This project is provided as-is for educational purposes, demonstrating blockchain runtime concepts inspired by Substrate's architecture.

---

*Inspired by the [Substrate](https://substrate.io/) runtime framework powering Polkadot and other parachains.*

