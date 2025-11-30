use alloc::vec::Vec;

use stylus_sdk::{
    alloy_primitives::{I64, U64},
    prelude::*,
};

const DECIMALS: u32 = 2;
const SCALE: u32 = 10u32.pow(DECIMALS); // 10^2 scaling factor

sol_storage! {
    pub struct Nd {}
}

#[public]
impl Nd {
    pub fn ln(&self, x: U64) -> I64 {
        let ln2: i64 = 69; // ln(2) ≈ 0.693147, scaled by 100

        if x.is_zero() {
            panic!("ln(0) is undefined");
        }

        let scale = U64::from(SCALE);

        // Special case: ln(1.00) = 0
        if x == scale {
            return I64::ZERO;
        }

        // Handle x < 1 by using ln(x) = -ln(1/x)
        if x < scale {
            let scale_squared = scale * scale;
            let inverse = scale_squared / U64::from(x);
            return -self.ln(U64::from(inverse));
        }

        // Normalize x down by dividing by 2 until it's less than 2
        let mut k: u64 = 0;
        let mut x_norm = U64::from(x);

        while x_norm >= U64::from(200) {
            x_norm >>= 1;
            k += 1;
        }

        // let x_norm = U64::from(x_norm);
        let z = U64::from(x_norm) - scale;

        // ln(1 + z/scale) = z - z²/2 + z³/3 - ...
        let mut term = z;
        let mut sum = I64::unchecked_from(term);
        let mut i = 2;

        for _ in 0..10 {
            term = (term * z) / scale;
            let a = term / U64::from(i);
            let current = I64::unchecked_from(a);
            if i % 2 == 0 {
                sum -= current;
            } else {
                sum += current;
            }
            i += 1;
        }

        // let k_ln2 = I64::try_from(k as i64 * LN2).unwrap();
        let b = k as i64 * ln2;
        let k_ln2 = I64::try_from(b).unwrap();
        sum + k_ln2
    }

    /// Fixed-point approximation of sqrt(x) using Newton’s method
    pub fn sqrt(&self, x: U64) -> U64 {
        if x == U64::ZERO {
            return U64::ZERO;
        }

        let scale = U64::from(100);
        let half = U64::from(2);

        // Multiply input by scale to preserve fixed-point precision
        let scaled_x = x * scale;

        // Initial guess
        let mut z = (scaled_x + scale) / half;
        let mut y = scaled_x;

        // Newton-Raphson iteration
        while z < y {
            y = z;
            z = (scaled_x / z + z) / half;
        }

        y
    }

    pub fn exp(&self, x: I64) -> I64 {
        let scale = U64::from(SCALE);

        let is_negative = x < I64::ZERO;
        let abs_x = if is_negative { -x } else { x };
        let abs_x_u64 = U64::try_from(abs_x).unwrap();

        // Reduce x by computing exp(x / n)^n
        let n = 8u32;
        let reduced_x = abs_x_u64 / U64::from(n);

        let mut sum = scale;
        let mut term = scale;
        let mut denominator = U64::from(1);

        for i in 1..60 {
            term = (term * reduced_x) / scale;
            denominator *= U64::from(i);
            let current = term / denominator;

            if current == U64::ZERO {
                break;
            }

            sum += current;
        }

        let result = self.powu(sum, n); // still scaled

        if is_negative {
            // Return SCALE^2 / result to maintain fixed-point format
            let scale_squared = (scale * scale) / result;
            I64::unchecked_from(scale_squared)
        } else {
            I64::unchecked_from(result)
        }
    }

    pub fn powu(&self, base: U64, exp: u32) -> U64 {
        let mut result = U64::from(SCALE);
        let mut b = base;
        let mut e = exp;

        while e > 0 {
            if e % 2 == 1 {
                result = (result * b) / U64::from(SCALE);
            }
            b = (b * b) / U64::from(SCALE);
            e /= 2;
        }

        result
    }

    /// Fixed-point cosine approximation using Taylor series (returns I64)
    pub fn cos(&self, theta: U64) -> I64 {
        let scale = I64::unchecked_from(SCALE); // e.g., 100
        let theta_i = I64::unchecked_from(theta);

        let two_pi = I64::unchecked_from(628); // 2π * 100
        let pi = I64::unchecked_from(314); // π * 100
        let half_pi = I64::unchecked_from(157); // π/2 * 100

        // Normalize theta to [0, 2π)
        let mut x = theta_i % two_pi;
        if x < I64::ZERO {
            x += two_pi;
        }

        // Cosine is symmetric: cos(x) = cos(2π - x)
        if x > pi {
            x = two_pi - x;
        }

        // Reflect into [0, π/2]
        let negate = if x > half_pi {
            x = pi - x;
            true
        } else {
            false
        };

        // Now use Taylor series approximation around 0 for cos(x)
        let x2 = (x * x) / scale;

        let mut result = scale;
        let mut term = x2;
        result -= term / I64::unchecked_from(2); // -x²/2!
        term = (term * x2) / scale;
        result += term / I64::unchecked_from(24); // +x⁴/4!
        term = (term * x2) / scale;
        result -= term / I64::unchecked_from(720); // -x⁶/6!

        if negate {
            result = -result;
        }

        result.clamp(I64::unchecked_from(-100), I64::unchecked_from(100)) // keep it in [-1, 1] scaled
    }

    pub fn gen_pair(&self, random: U64) -> (U64, U64) {
        // let random_u32 = U32::from(random.to::<u32>());

        // Extract the high 16 bits by shifting right 16 bits
        let u1 = (random >> U64::from(16)) % U64::from(100);

        // Extract the low 16 bits by casting directly
        let u2 = random % U64::from(100);

        (
            u1.saturating_add(U64::from(1)),
            u2.saturating_add(U64::from(1)),
        )
    }

    /// Computes normal distribution using Box-Muller Transform (Fixed-Point)
    pub fn normal_distribution(&self, random: U64, mean: u64, std_dev: u64) -> I64 {
        let one = U64::from(SCALE);

        let (u1, u2) = self.gen_pair(random);

        // ln(u1) is negative, so take the negation
        let ln_u1 = self.ln(u1);

        // radius = sqrt(-2 * ln(u1))
        let radius_squared_signed = I64::unchecked_from(-2) * ln_u1;
        let radius_squared = U64::try_from(radius_squared_signed).unwrap();
        let radius: U64 = self.sqrt(radius_squared);
        let radius_signed = I64::unchecked_from(radius);

        // theta = (2π * u2 / scale)
        let two_pi_scaled = U64::from(628u32); // 2π * 1e2
        let theta = (u2 * two_pi_scaled) / one;

        let cos_theta = self.cos(theta);

        let standard_normal = radius_signed * cos_theta;

        // Apply mean and standard deviation
        let snd = (standard_normal - I64::unchecked_from(mean)) / I64::unchecked_from(std_dev);

        snd
    }

    pub fn generate_normal_random(&mut self, random: U64, mean: u64, std_dev: u64) -> I64 {
        self.normal_distribution(random, mean, std_dev)
    }
}
