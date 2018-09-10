extern crate elapsed;
extern crate rug;
extern crate vdf;

use self::elapsed::measure_time;

use vdf::vdf_mod_sqrt;
use rug::Integer;

/// Example modulus as a big prime number (M13 prime)
pub const MODULUS: &str = "6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151";

/// An example of usage of VDF with time measurements.
fn main() {
    const TEST_HASH: &str = "1eeb30c7163271850b6d018e8282093ac6755a771da6267edf6c9b4fce9242ba";
    const DIFFICULTY: u64 = 1_000_000; // Approx. 1 min. on MBP 13"

    let seed_hash =
        Integer::from_str_radix(TEST_HASH, 16).unwrap();

    let modulus = Integer::from_str_radix(MODULUS, 10).unwrap();
    let seed = Integer::from(seed_hash.div_rem_floor(modulus.clone()).1);
    println!("Challenge (seed) is: 0x{:x}", seed);

    println!("Evaluating VDF...");

    let (elapsed, witness) = measure_time(|| {
        vdf_mod_sqrt::eval(&modulus, &seed, DIFFICULTY)
    });
    println!("Response is: 0x{:x}, elapsed: {}", &witness, elapsed);

    println!("Verifying VDF...");

    let (elapsed, is_verified) = measure_time(|| {
        vdf_mod_sqrt::verify(&modulus, &seed, DIFFICULTY, &witness)
    });
    println!("Verified: {}, elapsed: {}", is_verified, elapsed);

    assert!(is_verified)
}
