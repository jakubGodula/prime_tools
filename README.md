
# prime_tools [![Crate](https://img.shields.io/crates/v/prime_tools.svg)](https://crates.io/crates/prime_tools) [![Build Status](https://travis-ci.org/danmedani/prime_tools.svg?branch=master)](https://travis-ci.org/danmedani/prime_tools)
This util provides a few tools for working with prime numbers.

Mostly for personal use with project euler problems. :)

```Rust
fn get_primes_less_than_x(x: u32) -> Vec<u32>
```
>Generates an ordered list of prime numbers from 2 up to x (exclusive)
>
>Uses the sieve of Eratosthenes under the covers.

```Rust
fn get_prime_factors_with_counts(x: u32, primes: &Vec<u32>) -> HashMap<u32, u32>
```
>To be used in conjunction with get_primes_less_than_x.
>Be sure to pass in `primes` at least up to sqrt(x).