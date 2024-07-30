extern crate rand;
extern crate num_bigint;
extern crate num_traits;

use rand::Rng;
use num_bigint::BigInt;
use num_traits::{One, Zero};

// Function to compute the greatest common divisor
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if *b == BigInt::zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

// Function to compute modular inverse
fn mod_inverse(a: &BigInt, n: &BigInt) -> Option<BigInt> {
    let one = BigInt::one();
    let mut t = BigInt::zero();
    let mut new_t = one.clone();
    let mut r = n.clone();
    let mut new_r = a.clone();

    while new_r != BigInt::zero() {
        let quotient = &r / &new_r;
        t = t - &quotient * &new_t;
        std::mem::swap(&mut t, &mut new_t);
        r = r - &quotient * &new_r;
        std::mem::swap(&mut r, &mut new_r);
    }

    if r > one {
        return None;
    }

    if t < BigInt::zero() {
        t = t + n;
    }

    Some(t)
}

fn main() {
    let mut rng = rand::thread_rng();

    // Assume p and q are two huge primes
    let p = BigInt::parse_bytes(b"61", 10).unwrap(); // Replace with actual large prime
    let q = BigInt::parse_bytes(b"53", 10).unwrap(); // Replace with actual large prime
    let n = &p * &q;

    // Assume x is a secret value with gcd(x, n) = 1
    let x = BigInt::parse_bytes(b"17", 10).unwrap(); // Replace with actual secret
    assert!(gcd(&x, &n) == BigInt::one());

    let x_squared = (&x * &x) % &n;
    println!("Cyprian's identity: n = {}, x^2 mod n = {}", n, x_squared);

    // Step 1: Cyprian selects a random unit u1 modulo n with gcd(u1, n) = 1
    let mut u1;
    loop {
        u1 = BigInt::from(rng.gen::<u64>()) % &n;
        if gcd(&u1, &n) == BigInt::one() {
            break;
        }
    }
    println!("Cyprian's random unit: u1 = {}", u1);

    let u1_inv = mod_inverse(&u1, &n).unwrap();
    let u2 = (&x * &u1_inv) % &n;
    assert!((&u1 * &u2) % &n == x);

    let x1 = (&u1 * &u1) % &n;
    let x2 = (&u2 * &u2) % &n;
    println!("Cyprian sends to Alex: x1 = {}, x2 = {}", x1, x2);

    // Step 3: Alex checks x1 * x2 == x^2 mod n
    assert!((&x1 * &x2) % &n == x_squared);

    // Step 4: Alex randomly asks for sqrt(x1) or sqrt(x2)
    let request = if rng.gen_bool(0.5) { "sqrt(x1)" } else { "sqrt(x2)" };
    println!("Alex requests: {}", request);

    // Step 5: Cyprian sends the quantity he requested
    let response = if request == "sqrt(x1)" { 
        println!("Cyprian's response: sqrt(x1) = {}", u1);
        u1.clone() 
    } else { 
        println!("Cyprian's response: sqrt(x2) = {}", u2);
        u2.clone() 
    };

    // Step 6: Alex verifies the response
    if request == "sqrt(x1)" {
        let verification = (&response * &response) % &n;
        println!("Verification: {}^2 mod {} = {}", response, n, verification);
        assert!(verification == x1);
    } else {
        let verification = (&response * &response) % &n;
        println!("Verification: {}^2 mod {} = {}", response, n, verification);
        assert!(verification == x2);
    }
    println!("Alex verifies the response successfully. Cyprian passes the proof.");

    // Repeat the challenges until Alex is satisfied
    // (This part is not shown here, but it would involve looping through the above steps multiple times)
}




