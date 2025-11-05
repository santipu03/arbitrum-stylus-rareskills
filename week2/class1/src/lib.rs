// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use alloy_sol_types::sol;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{address, Address, U256},
    prelude::*,
};

sol_interface! {
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
    }
}

const ADDRESSES: [Address; 2] = [
    address!("5E1497dD1f08C87b2d8FE23e9AAB6c1De833D927"),
    address!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"),
];

// Define some persistent storage using the Solidity ABI.
// `PushBasedDistributor` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct PushBasedDistributor {}
}

// Declare events and Solidity error types
sol! {
    error TransferFailed();
    error InsufficientBalance();
}

#[derive(SolidityError)]
pub enum Error {
    TransferFailed(TransferFailed),
    InsufficientBalance(InsufficientBalance),
}

#[public]
impl PushBasedDistributor {
    /// Calculates the amount to distribute per recipient.
    fn get_amount_to_distribute(&self, _token: Address) -> Result<U256, Error> {
        // Get the contract's token balance
        let _token_instance = IERC20::new(_token);
        let balance: U256 = _token_instance
            .balance_of(self, self.vm().contract_address())
            .unwrap();

        let recipient_count = ADDRESSES.len();
        if balance <= U256::from(0) {
            return Err(Error::InsufficientBalance(InsufficientBalance {}));
        }

        // Calculate the share per recipient
        Ok(balance / U256::from(recipient_count))
    }

    /// Distributes tokens to all predefined addresses.
    pub fn distribute(&mut self, _token: IERC20) -> Result<(), Error> {
        let amount = self.get_amount_to_distribute(_token.address.clone())?;

        for &recipient in ADDRESSES.iter() {
            if _token.transfer(&mut *self, recipient, amount).is_err() {
                return Err(Error::TransferFailed(TransferFailed {}));
            }
        }

        Ok(())
    }
}