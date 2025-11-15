// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use alloy_sol_types::sol;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U64, prelude::*};

// Define some persistent storage using the Solidity ABI.
// `NthPrime` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct NthPrime {}
}

sol! {
    error IndexIsZero();
}

#[derive(SolidityError)]
pub enum Error {
    IndexIsZero(IndexIsZero),
}
/// Declare that `NthPrime` is a contract with the following external methods.
#[public]
impl NthPrime {
    // Get the Nth prime number
    pub fn get_prime(&self, index: U64) -> Result<U64, Error> {
        if index == U64::ZERO {
            return Err(Error::IndexIsZero(IndexIsZero {}));
        }

        let mut count: U64 = U64::ZERO;
        let mut number: U64 = U64::from(1);

        while count < index {
            number += U64::from(1);
            if self.is_prime(number.to::<u64>()) {
                count += U64::from(1);
            }
        }

        Ok(number)
    }

    // Check if a number is prime
    fn is_prime(&self, n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let mut i: u64 = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}
