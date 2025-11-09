// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use alloy_sol_types::sol;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{Address, FixedBytes, U256},
    prelude::*,
};

sol_storage! {
    #[entrypoint]
    pub struct GlobVars {
        address owner;
    }
}

sol! {
    event EmitMe();
}

/// Declare that `GlobVars` is a contract with the following external methods.
#[public]
impl GlobVars {
    #[constructor]
    pub fn constructor(&mut self) {
        self.owner.set(self.vm().msg_sender());
    }

    /// Adds the wei value from msg _value to the number in storage.
    #[payable]
    pub fn receive_funds(&self) -> U256 {
        self.vm().msg_value()
    }

    pub fn get_balance(&self) -> U256 {
        let this_contract = self.vm().contract_address();

        self.vm().balance(this_contract)
    }

    pub fn get_origin(&self) -> Address {
        self.vm().tx_origin()
    }

    pub fn get_ink(&self, gas: U256) -> U256 {
        let gas_u64 = gas.to::<u64>();

        U256::from(self.vm().gas_to_ink(gas_u64))
    }

    pub fn emit_event(&self) {
        log(self.vm(), EmitMe {});
    }

    pub fn keccek_hash(&self, preimage: U256) -> FixedBytes<32> {
        self.vm().native_keccak256(&preimage.to_be_bytes::<32>())
    }
}
