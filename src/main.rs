extern crate bit_vec;
use bit_vec::BitVec;

fn main() {
    println!("Hello, world!");

    let max_num = 1_000_000_000;
	let primes = get_primes_less_than_x(max_num);
	println!("primes len = {}", primes.len());
	println!("first primes = {:?}", primes[0]);
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
	for i in 2.. 2 + (x as f64).sqrt() as usize {
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
