// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

// Modules and imports
mod nd;

use crate::nd::Nd;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{I64, U256, U64},
    prelude::*,
};

const DECIMALS: u32 = 2;
const SCALE: u32 = 10u32.pow(DECIMALS); // 10^2 scaling factor

sol_storage! {
    #[entrypoint]
    pub struct Monte {
        #[borrow]
        Nd normal_dist;
    }
}

#[public]
impl Monte {
    pub fn monte_carlo_option_price(
        &mut self,
        rand_val: U64,        // Starting random value (seed)
        time_to_exp: U64,     // Time until option expires (T)
        vol: U64,             // Volatility (σ) — how unpredictable the asset is
        start_price: U64,     // Current price (S0)
        strike_price: U64,    // Price you can buy the asset within the expiry time (K)
        num_simulations: U64, // How many scenarios to simulate
    ) -> U64 {
        let scale = I64::unchecked_from(SCALE);

        let mut payoff_sum = U64::ZERO;

        let sqrt_t = self.normal_dist.sqrt(time_to_exp); // sqrt(T)

        let rng_hash: U256 = self
            .vm()
            .native_keccak256(&rand_val.to_be_bytes_vec())
            .into();
        let random_value_u64: U256 = rng_hash >> 224; // u32::max = 4294967295
        let mut random_value = U64::from(random_value_u64.to::<u64>());

        // The simulation
        for _ in 0..num_simulations.to::<usize>() {
            // Generate a random value
            let z: I64 = self
                .normal_dist
                .generate_normal_random(random_value, 0u64, 100u64); // mean=0, std_dev=1

            // vol_term = (σ * sqrt(T) * z) / scale**2
            // Represents how the price could change due to volatility, time and randomness
            let vol_term =
                ((I64::unchecked_from(vol) * I64::unchecked_from(sqrt_t) * z) / scale) / scale;

            // ST = (S * exp(vol_term)) / scale;
            let simulated_price =
                (I64::unchecked_from(start_price) * self.normal_dist.exp(vol_term)) / scale;

            // Buy option payoff = max(ST - K, 0). For sell option payoff max(K - ST, 0)
            let payoff = if simulated_price > I64::unchecked_from(strike_price) {
                U64::try_from(simulated_price).unwrap() - strike_price
            } else {
                U64::ZERO
            };

            payoff_sum += payoff;

            // Update random_value so that the next iteration gets a new "random" number
            let rng_hash: U256 = self
                .vm()
                .native_keccak256(&random_value.to_be_bytes_vec())
                .into();

            let rng_u64: U256 = rng_hash >> 224; // u32::max = 4294967295
            random_value = U64::from(rng_u64.to::<u64>());
        }

        // Average payoff (the profit you would make if this simulation came true)
        payoff_sum / num_simulations
    }
}
