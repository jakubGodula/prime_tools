use std::collections::HashMap;
use math::round;
extern crate bit_vec;
use bit_vec::BitVec;

fn main() {
    println!("Hello, world!");

    let max_num = 10_000;
	let primes = get_primes_less_than_x(max_num);
	println!("primes len = {}", primes.len());
	println!("first primes = {:?}", primes[0]);
	println!("{:#?}", get_prime_factors_with_counts(1200, &primes));
}

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

fn get_prime_factors_with_counts(x: u32, primes: &Vec<u32>) -> HashMap<u32, u32> {
	let mut factor_counts = HashMap::new();
	let mut primes_index = 0;
	let mut drop_x = x;

	while drop_x > 1 {
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
	factor_counts
}






