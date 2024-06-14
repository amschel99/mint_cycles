//! # ICP to Cycles Conversion Library
//!
//! ## Overview
//!
//! This Rust library provides a function to convert ICP tokens into cycles programmatically. This functionality is essential for scenarios where you need to manage and deploy canisters dynamically using the Rust CDK.
//!
//! ## Use Case
//!
//! One primary use case is when building canisters for users on the fly. In such cases, you need to add cycles to the canisters programmatically and deploy them automatically. The `dfx` CLI tool isn't suitable for this scenario since it is designed for manual interactions. Our library offers a programmatic solution for converting ICP tokens into cycles.
//!
//! ## Functionality
//!
//! ### `mint_cycles`
//!
//! The library includes a single function: `mint_cycles`. This function converts ICP tokens held by a canister into cycles.
//!
//! #### Function Signature
//!
//! ```rust
//! fn mint_cycles(amount: Tokens)
//! ```
//!
//! ##### Parameters
//!
//! `amount`: This parameter is of type `Tokens`, defined as:
//!
//! ```rust
//! pub struct Tokens {
//!     e8s: u64,
//! }
//! ```
//!
//! Refer to `ic_ledger_types::Tokens` for more details.
//!
//! ### Assumptions
//!
//! - The canister calling `mint_cycles` holds some ICP tokens.
//! - Canisters can hold and transfer ICP tokens.
//!
//! Before calling `mint_cycles`, you can top up your main canister with ICP tokens using a wallet of your choice or any other method. This main canister can be used for automatically building and deploying other canisters.
//!
//! ### Example Workflow
//!
//! 1. **Top Up Main Canister**: Transfer ICP tokens to your main canister.
//! 2. **Convert Tokens to Cycles**: Call `mint_cycles` with the desired amount of tokens to convert them into cycles.
//! 3. **Deploy Canisters**: Use the cycles to build and deploy other canisters programmatically.
//!
//! ## Source Code
//!
//! For more details and to contribute, please refer to the source code.
//!
//! This library aims to simplify the process of converting ICP tokens into cycles for dynamic canister management and deployment. We hope this helps, and we look forward to your contributions.

mod cmc;
mod errors;
mod ledger;
mod transaction;
pub mod rust_declarations {
    pub mod cmc_service;
    pub mod icp_ledger_service;
}
pub use transaction::mint_cycles;
