//! # prime_tools
//!
//! `prime_tools` is a collection of utilities to make working with
//! prime numbers a bit easier.

use std::collections::HashMap;
use math::round;
extern crate bit_vec;
use bit_vec::BitVec;

/// Generates a list of prime numbers less than x.
///
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
/// It's expected that this will be used in combination with get_primes_less_than_x.
///
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
/// 	prime_tools::get_prime_factors_with_counts(
///			120, 
///			&primes
///		),
/// 	result
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
/// 	prime_tools::get_prime_factors_with_counts(
///			101,
///			&primes
///		),
/// 	result
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


// #[cfg(test)]
// mod tests {
// 	use crate::*;

//     #[test]
//     fn primes_less_than_11() {
//         assert_eq!(
//         	get_primes_less_than_x(11), 
//         	vec![2, 3, 5, 7]
//     	);
//     }

//     #[test]
//     fn primes_less_than_12() {
//         assert_eq!(
//         	get_primes_less_than_x(12),
//         	vec![2, 3, 5, 7, 11]
//     	);
//     }

//     #[test]
//     fn primes_less_than_2() {
//         assert_eq!(
//         	get_primes_less_than_x(1),
//         	vec![]
//     	);
//     }

//     #[test]
//     fn primes_less_than_3() {
//         assert_eq!(
//         	get_primes_less_than_x(3),
//         	vec![2]
//     	);
//     }

//     #[test]
//     fn test_prime_factors_of_120() {
//     	let primes = get_primes_less_than_x(12);
//     	let mut result = HashMap::new();
//     	result.insert(2, 3);
//     	result.insert(3, 1);
//     	result.insert(5, 1);

//     	assert_eq!(
//     		get_prime_factors_with_counts(120, &primes),
//     		result
//     	);
//     }

//     #[test]
//     fn test_prime_factors_of_121() {
//     	let primes = get_primes_less_than_x(12);
//     	let mut result = HashMap::new();
//     	result.insert(11, 2);
    	
//     	assert_eq!(
//     		get_prime_factors_with_counts(121, &primes),
//     		result
//     	);
//     }

//     #[test]
//     fn test_prime_factors_of_11() {
//     	let primes = get_primes_less_than_x(4);
//     	let mut result = HashMap::new();
//     	result.insert(11, 1);
    	
//     	assert_eq!(
//     		get_prime_factors_with_counts(11, &primes),
//     		result
//     	);
//     }

//     #[test]
//     fn test_prime_factors_of_11_more_primes() {
//     	let primes = get_primes_less_than_x(12);
//     	let mut result = HashMap::new();
//     	result.insert(11, 1);
    	
//     	assert_eq!(
//     		get_prime_factors_with_counts(11, &primes),
//     		result
//     	);
//     }

    
// }
