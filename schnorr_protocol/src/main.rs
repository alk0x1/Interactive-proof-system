use rand::Rng;
use num_bigint_dig::{BigInt, RandBigInt};
use sha2::{Sha256, Digest};

// Define the prime and generator
const P: &str = "23";
const G: &str = "5";

fn main() {
    // Convert P and G to BigInt
    let p = BigInt::parse_bytes(P.as_bytes(), 10).unwrap();
    let g = BigInt::parse_bytes(G.as_bytes(), 10).unwrap();

    // Prover's secret
    let x = BigInt::from(6); // This is the secret value

    // Calculate y = g^x mod p
    let y = g.modpow(&x, &p);
    println!("Public key (y): {}", y);

    // Prover's commitment
    let r = generate_random_bigint(&p);
    let t = g.modpow(&r, &p);
    println!("Prover's commitment (t): {}", t);

    // Verifier's challenge
    let c = generate_random_bigint(&p);
    println!("Verifier's challenge (c): {}", c);

    // Prover's response
    let s = (&r + &c * &x) % (&p - 1);
    println!("Prover's response (s): {}", s);

    // Verifier's verification
    let lhs = g.modpow(&s, &p);
    let rhs = (t * y.modpow(&c, &p)) % &p;
    println!("Verification: g^s mod p = {}", lhs);
    println!("t * y^c mod p = {}", rhs);

    if lhs == rhs {
        println!("Proof is accepted.");
    } else {
        println!("Proof is rejected.");
    }
}

fn generate_random_bigint(modulus: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    rng.gen_bigint_range(&BigInt::from(1), modulus)
}
