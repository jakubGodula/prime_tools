//! # prime_tools
//!
//! `prime_tools` is a collection of utilities to make working with
//! prime numbers a bit easier.

use std::collections::HashMap;
use math::round;
extern crate bit_vec;
use bit_vec::BitVec;

/// Generates an ordered list of prime numbers less than x.
///
/// Uses the Sieve of Eratosthenes under the covers.
/// # Examples
///
/// ```
/// let x = 11;
/// let answer = prime_tools::get_primes_less_than_x(x);
///
/// assert_eq!(vec![2, 3, 5, 7], answer);
/// ```
///
/// ```
/// let x = 12;
/// let answer = prime_tools::get_primes_less_than_x(x);
///
/// assert_eq!(vec![2, 3, 5, 7, 11], answer);
/// ```
pub fn get_primes_less_than_x(x: u32) -> Vec<u32> {
    let mut primes = Vec::new();

    let prime_map = get_prime_bit_map(x as u64);
    for i in 0..x as usize {
        if prime_map[i] {
            primes.push(i as u32);
        }
    }

    primes
}


/// Creates a map of prime factors -> prime factor counts. 
/// 
/// To be used with get_primes_less_than_x.
///
/// Note: This will misbehave if any primes `< √x` are not included in `primes`.
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// let primes = prime_tools::get_primes_less_than_x(12);
///
/// let mut result = HashMap::new();
/// result.insert(2, 3);
/// result.insert(3, 1);
/// result.insert(5, 1);
///
/// assert_eq!(
///     prime_tools::get_prime_factors_with_counts(
///            120, 
///            &primes
///        ),
///     result
/// );
/// ```
///
/// ```
/// use std::collections::HashMap;
/// let primes = prime_tools::get_primes_less_than_x(11);
///
/// let mut result = HashMap::new();
/// result.insert(101, 1);
///
/// assert_eq!(
///     prime_tools::get_prime_factors_with_counts(
///            101,
///            &primes
///        ),
///     result
/// );
/// ```
pub fn get_prime_factors_with_counts(x: u32, primes: &Vec<u32>) -> HashMap<u32, u32> {
    let mut factor_counts = HashMap::new();
    let mut primes_index = 0;
    let mut drop_x = x;

    while drop_x > 1 && primes_index < primes.len() {
        let prime = primes[primes_index];
        let mut prime_count = 0;        
        
        while drop_x % prime == 0 {
            prime_count += 1;
            drop_x = drop_x / prime;
        }

        if prime_count != 0 {
            factor_counts.insert(prime, prime_count);
        }
        primes_index += 1;
    }

    if factor_counts.len() == 0 {
        // We didn't find any prime factors: x must be a prime.
        factor_counts.insert(x, 1);
    }

    factor_counts
}

/// Figures out if a u32 is prime.
///
/// This is pretty fast: I've benchmarked it at 2.7 seconds to process 1 million random `u32`s.
///
/// Todo: use fermat's little theorem to make this faster. 
///
/// ```
/// assert_eq!(
///     prime_tools::is_u32_prime(982_451_653),
///     true
/// );
/// ```
///
/// ```
/// assert_eq!(
///     prime_tools::is_u32_prime(5_083),
///     false
/// );
/// ```
///
/// ```
/// assert_eq!(
///     prime_tools::is_u32_prime(1),
///     false
/// );
/// ```
pub fn is_u32_prime(x: u32) -> bool {
    if x < 2 { return false; }
    (!is_u32_definitely_composite(x)) && is_u32_definately_prime(x)
}

/// Figures out if a u64 is prime.
///
/// This is pretty slow: I've benchmarked it at 26 seconds to process only 200 random `u64`s. :(
///
/// Todo: use fermat's little theorem to make this faster.
///
/// ```
/// assert_eq!(
///     prime_tools::is_u64_prime(23_423_412_349),
///     true
/// );
/// ```
///
/// ```
/// assert_eq!(
///     prime_tools::is_u64_prime(23_423_414_138),
///     false
/// );
/// ```
///
/// ```
/// assert_eq!(
///     prime_tools::is_u64_prime(1),
///     false
/// );
/// ```
pub fn is_u64_prime(x: u64) -> bool {
    if x < 2 { return false; }
    (!is_u64_definitely_composite(x)) && is_u64_definately_prime(x)
}


/// Generates u64 primes between min (inclusive) and max (exclusive).
///
/// WARNING #1: This can be very slow if the max is greater than 10^17 ish,
/// or if the range is too large.
///
/// WARNING #2: This will break if the max is too much higher than 10^19 ish.
///
/// Uses a modified sieve of eratosthenes
///
/// ```
/// assert_eq!(
///     prime_tools::get_primes_between(11, 29),
///     vec![11, 13, 17, 19, 23]
/// );
/// ```
///
/// ```
/// assert_eq!(
///     prime_tools::get_primes_between(10, 30),
///     vec![11, 13, 17, 19, 23, 29]
/// );
/// ```
///
/// ```
/// assert_eq!(
///     prime_tools::get_primes_between(1, 10),
///     vec![2, 3, 5, 7]
/// );
/// ```
///
/// ```
/// assert_eq!(
///     prime_tools::get_primes_between(100_000_000_000, 100_000_000_200),
///     vec![100000000003, 100000000019, 100000000057, 100000000063, 100000000069, 100000000073, 100000000091, 100000000103, 100000000129, 100000000171, 100000000183, 100000000193]
/// );
/// ```
pub fn get_primes_between(min: u64, max: u64) -> Vec<u64> {
    let true_min = match min < 2 {
        true => 2,
        _ => min
    };

    let highest_factor = (max as f64).sqrt() as u32;
    let possible_prime_factors: Vec<u64> = get_primes_less_than_x(highest_factor + 1).iter().map(|&prime| prime as u64).collect();

    // the offset sieve
    let mut prime_map = BitVec::from_elem((max - true_min) as usize + 1, true);
    for prime in &possible_prime_factors {
        let multiplier = match true_min > *prime {
            true => true_min / prime,
            _ => 1
        };

        // Run val (a multiple of prime) from min to max, marking numbers as not prime.
        let mut val = multiplier * prime;

        // In the case that the prime is >= min, we'll want to avoid marking it as not prime
        if *prime >= true_min {
            val += prime;
        }

        if val < true_min {
            val += prime;
        }
        while val < max {
            prime_map.set((val - true_min) as usize, false);
            val += prime;
        }
    }

    let mut primes = Vec::new();
    for val in true_min..max {
        if prime_map[(val - true_min) as usize] {
            primes.push(val);
        }
    }
    primes
}


fn get_prime_bit_map(x: u64) -> BitVec {
    let mut prime_map = BitVec::from_elem(x as usize + 1, true);
    
    // 0 and 1 are not primes
    prime_map.set(0, false);
    prime_map.set(1, false);

    // sieve of eratosthenes
    for i in 2..=round::ceil((x as f64).sqrt(), 1) as usize {
        if prime_map[i] {
            for j in i.. {
                if i * j > x as usize {
                    break;
                }
                prime_map.set(i * j, false);
            }
        }
    }

    prime_map
}

fn is_u64_definately_prime(x: u64) -> bool {
    if x == 2 || x == 3 { true }
    if x % 2 == 0 || x % 3 == 0 { false }
    let mut i = 5;
    let mut w = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }
    return true;
}

// Todo: Implement this with fermat's little theorem
fn is_u64_definitely_composite(_x: u64) -> bool{
    return false;
}

fn is_u32_definately_prime(x: u32) -> bool {
    if x == 2 || x == 3 { return true; }
    if x % 2 == 0 || x % 3 == 0 { return false; }

    let mut i = 5;
    let mut w = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }
    return true;
}

// Todo: Implement this with fermat's little theorem
fn is_u32_definitely_composite(_x: u32) -> bool{
    return false;
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn primes_less_than_11() {
        assert_eq!(
            get_primes_less_than_x(11), 
            vec![2, 3, 5, 7]
        );
    }

    #[test]
    fn primes_less_than_12() {
        assert_eq!(
            get_primes_less_than_x(12),
            vec![2, 3, 5, 7, 11]
        );
    }

    #[test]
    fn primes_less_than_2() {
        assert_eq!(
            get_primes_less_than_x(1),
            vec![]
        );
    }

    #[test]
    fn primes_less_than_3() {
        assert_eq!(
            get_primes_less_than_x(3),
            vec![2]
        );
    }

    #[test]
    fn test_prime_factors_of_120() {
        let primes = get_primes_less_than_x(12);
        let mut result = HashMap::new();
        result.insert(2, 3);
        result.insert(3, 1);
        result.insert(5, 1);

        assert_eq!(
            get_prime_factors_with_counts(120, &primes),
            result
        );
    }

    #[test]
    fn test_prime_factors_of_121() {
        let primes = get_primes_less_than_x(12);
        let mut result = HashMap::new();
        result.insert(11, 2);
        
        assert_eq!(
            get_prime_factors_with_counts(121, &primes),
            result
        );
    }

    #[test]
    fn test_prime_factors_of_11() {
        let primes = get_primes_less_than_x(4);
        let mut result = HashMap::new();
        result.insert(11, 1);
        
        assert_eq!(
            get_prime_factors_with_counts(11, &primes),
            result
        );
    }

    #[test]
    fn test_prime_factors_of_11_more_primes() {
        let primes = get_primes_less_than_x(12);
        let mut result = HashMap::new();
        result.insert(11, 1);
        
        assert_eq!(
            get_prime_factors_with_counts(11, &primes),
            result
        );
    }

    #[test]
    fn test_sieve_vs_spot_check_integration() {
        let max_val = 10_000;
        let primes_using_sieve = get_primes_less_than_x(max_val);

        let mut primes_using_primality = Vec::new();
        for val in 1..max_val {
            if is_u32_prime(val) {
                primes_using_primality.push(val);
            }
        }
        assert_eq!(
            primes_using_sieve.len(),
            primes_using_primality.len()
        );
        assert_eq!(
            primes_using_sieve[0],
            primes_using_primality[0]
        );
        assert_eq!(
            primes_using_sieve[primes_using_sieve.len()-1],
            primes_using_primality[primes_using_primality.len()-1]
        );
    }

    #[test]
    fn test_get_primes_between_edge_cases() {
        assert_eq!(
            get_primes_between(3, 8),
            vec![3, 5, 7]
        );
        assert_eq!(
            get_primes_between(2, 7),
            vec![2, 3, 5]
        );
        assert_eq!(
            get_primes_between(2, 3),
            vec![2]
        );
        assert_eq!(
            get_primes_between(2, 4),
            vec![2, 3]
        );
        assert_eq!(
            get_primes_between(2, 2),
            vec![]
        );
        assert_eq!(
            get_primes_between(4, 6),
            vec![5]
        );
        assert_eq!(
            get_primes_between(5, 6),
            vec![5]
        );
        assert_eq!(
            get_primes_between(1, 3),
            vec![2]
        );
        assert_eq!(
            get_primes_between(0, 2),
            vec![]
        );
        assert_eq!(
            get_primes_between(100_000_000_000_000, 100_000_000_000_100),
            vec![100000000000031, 100000000000067, 100000000000097, 100000000000099]
        );

        let primes_under: Vec<u64> = get_primes_less_than_x(101).iter().map(|&x| x as u64).collect();
        assert_eq!(
            get_primes_between(2, 101),
            primes_under
        );

        let primes_under: Vec<u64> = get_primes_less_than_x(10).iter().map(|&x| x as u64).collect();
        assert_eq!(
            get_primes_between(0, 10),
            primes_under
        );
    }
}
