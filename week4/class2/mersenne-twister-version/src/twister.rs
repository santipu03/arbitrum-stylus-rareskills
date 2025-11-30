use stylus_sdk::prelude::*;

const W: u32 = 32;
const N: usize = 624;
const M: usize = 397;
const R: u32 = 31;

const A: u32 = 0x9908B0DF;
const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;
const L: u32 = 18;
const F: u32 = 1812433253;

const LOWER_MASK: u32 = (1 << R) - 1;
const UPPER_MASK: u32 = !LOWER_MASK;

sol_storage! {
    pub struct MT19937 {}
}

#[public]
impl MT19937 {
    pub fn rand32(&mut self, seed: u32) -> u32 {
        let mut mt = [0u32; N];
        let mut index: usize = N;

        mt[0] = seed;
        for i in 1..N {
            let prev = mt[i - 1];
            mt[i] = F
                .wrapping_mul(prev ^ (prev >> (W - 2)))
                .wrapping_add(i as u32);
        }

        if index >= N {
            for i in 0..N {
                let x = (mt[i] & UPPER_MASK) + (mt[(i + 1) % N] & LOWER_MASK);
                let mut x_a = x >> 1;
                if x % 2 != 0 {
                    x_a ^= A;
                }
                mt[i] = mt[(i + M) % N] ^ x_a;
            }
            index = 0;
        }

        let mut y = mt[index];
        y ^= (y >> U) & D;
        y ^= (y << S) & B;
        y ^= (y << T) & C;
        y ^= y >> L;

        index += 1;
        y
    }
}
