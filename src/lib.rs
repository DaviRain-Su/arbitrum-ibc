//! Implements a IBC protocol for Arbitrum Stylus, providing a Solidity ABI-equivalent
//! Rust implementation of the Counter contract example provided by Foundry.
//! Warning: this code is a template only and has not been audited.

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

pub mod ibc_types;

use ibc_types::height::Height;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    pub struct Context {
        uint256 number;
        Height height;
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Context {
    // Gets the number from storage.
    pub fn revision_heiht(&self) -> Result<U256, Vec<u8>> {
        Ok(U256::from(self.height.revision_height.get().as_limbs()[0]))
    }

    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }
}
