extern crate rug;

/// Modular Square Roots-based Verifiable Delay Function (VDF) implementation.
pub mod vdf_mod_sqrt {
    use rug::Integer;

    /// Verifies that delay function from given `seed` was calculated and produced a `witness`
    pub fn verify(modulus: &Integer, seed: &Integer, num_steps: u64, witness: &Integer) -> bool {
        // Get instance of 2 in Integer format for performing of squares
        let square: Integer = 2u64.into();

        // Perform NUM_ITERS of sequential modular squares to perform a verification of the solution
        let mut result = witness.clone();
        for _ in 0..num_steps {
            // Perform a simple and fast modular squaring
            result.pow_mod_mut(&square, &modulus).unwrap();

            // Perform an iterating permutation in Fp
            let inv_result = -result;
            let r = inv_result.div_rem_floor_ref(&modulus);
            result = <(Integer, Integer)>::from(r).1;

            // Perform XOR of the result as a basic secure permutation
            // against possible modular square root short circuits
            result ^= 1;
            while result >= *modulus || result == 0 { // XOR in such a way that result won't exceed modulus
                result ^= 1;
            }
        }

        result == seed.clone().div_rem_floor(modulus.clone()).1
    }

    /// A verifiable delay function based on the
    /// slow sequential function based on permutations in Fp (sequential modulo square roots)
    /// It should be slow (or at least non-parallelizable) to compute but (very) fast to verify.
    pub fn eval(modulus: &Integer, seed: &Integer, num_steps: u64) -> Integer {
        // Allocate our own exponentiation moduli
        let modulus = modulus.clone();

        // Take seed by moduli p
        let mut x = Integer::from(seed.clone()
            .div_rem_floor(modulus.clone()).1);

        // Exponent for square root calculation
        let exponent = (modulus.clone() + 1) / 4;

        // Perform `NUM_ITERS` sequential modular square root computations
        for _ in 0..num_steps {
            // Perform XOR of the result as a basic secure permutation
            // against possible modular square root short circuits
            x ^= 1;
            while x >= modulus || x == 0 { // XOR in such a way that result won't exceed modulus
                x ^= 1;
            }

            // Perform a slow modular square root extraction
            x.pow_mod_mut(&exponent, &modulus).unwrap();
        }

        x
    }
}