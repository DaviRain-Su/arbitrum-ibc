use alloy_primitives::U64;
use ibc::core::ics02_client::error::ClientError;
use ibc::Height as IbcHeight;
/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::prelude::*;

// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint64 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    pub struct Height {
        uint64 revision_number;
        uint64 revision_height;
    }
}

impl Height {
    pub fn inner(&self) -> (U64, U64) {
        (self.revision_number.get(), self.revision_height.get())
    }

    pub fn revision_height(&self) -> U64 {
        self.revision_height.get()
    }

    pub fn revision_number(&self) -> U64 {
        self.revision_number.get()
    }

    pub fn get(&self) -> Result<IbcHeight, ClientError> {
        let binding = self.revision_number.get();
        let revision_number = binding.as_limbs();
        let binding = self.revision_number.get();
        let revision_height = binding.as_limbs();
        IbcHeight::new(revision_number[0], revision_height[0])
    }

    pub fn set(&mut self, height: IbcHeight) {
        self.revision_number
            .set(U64::from(height.revision_number()));
        self.revision_height
            .set(U64::from(height.revision_height()));
    }
}
