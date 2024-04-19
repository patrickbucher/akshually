use std::cmp::Ordering;

/// Rounds the `value` with the specified `granularity`.
///
/// # Examples
///
/// ```
/// assert_eq!(akshually::math::round_to(1.93333, 0.05), 1.95);
/// ```
pub fn round_to(value: f64, granularity: f64) -> f64 {
    let scale_factor = 1.0 / granularity;
    let scaled_up = value * scale_factor;
    let rounded = scaled_up.round();
    let scaled_down = rounded / scale_factor;
    scaled_down
}

/// Provides an Iterator for prime numbers.
///
/// # Examples
///
/// ```
/// let mut primes = akshually::math::PrimeIterator::new();
/// assert_eq!(primes.next(), Some(2));
/// assert_eq!(primes.next(), Some(3));
/// assert_eq!(primes.next(), Some(5));
/// ```
pub struct PrimeIterator {
    i: u64,
    primes: Vec<u64>,
}

impl PrimeIterator {
    /// Creates a new `PrimeIterator`.
    pub fn new() -> Self {
        PrimeIterator {
            i: 2,
            primes: Vec::new(),
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    /// Computes and returns the next prime number.
    fn next(&mut self) -> Option<u64> {
        let mut i = self.i;
        loop {
            if !self.primes.iter().any(|p| i % p == 0) {
                self.primes.push(i);
                self.i = i + 1;
                return Some(i);
            }
            i += 1;
        }
    }
}

/// Factorizes the given number into its prime factors.
///
/// # Examples
///
/// ```
/// assert_eq!(akshually::math::factorize(36), vec![2, 2, 3, 3]);
/// assert_eq!(akshually::math::factorize(64), vec![2, 2, 2, 2, 2, 2]);
/// assert_eq!(akshually::math::factorize(13), vec![13]);
/// ```
pub fn factorize(n: u64) -> Vec<u64> {
    let mut primes = PrimeIterator::new();
    let mut factors = Vec::new();
    let mut n = n;
    let mut prime = match primes.next() {
        Some(p) => p,
        None => {
            return vec![n];
        }
    };
    while n > 1 {
        if n % prime == 0 {
            factors.push(prime);
            n /= prime;
        } else {
            prime = match primes.next() {
                Some(p) => p,
                None => {
                    factors.push(n);
                    return factors;
                }
            }
        }
    }
    factors
}

pub fn reduce_fraction(numerator: u64, denominator: u64) -> (u64, u64) {
    let num_factors = factorize(numerator);
    let den_factors = factorize(denominator);

    let mut common: Vec<u64> = Vec::new();
    let mut left = num_factors.iter();
    let mut right = num_factors.iter();
    let mut l = left.next();
    let mut r = right.next();
    loop {
        let x = match l {
            Some(n) => n,
            None => {
                break;
            }
        };
        let y = match r {
            Some(n) => n,
            None => {
                break;
            }
        };
        match x.cmp(y) {
            Ordering::Equal => {
                common.push(*x);
                l = left.next();
                r = left.next();
            }
            Ordering::Less => {
                l = left.next();
            }
            Ordering::Greater => {
                r = left.next();
            }
        }
    }

    let gcd = common.iter().fold(1, |x, acc| x * acc);
    (numerator / gcd, denominator / gcd)
}

#[cfg(test)]
mod tests {
    use crate::math::{factorize, round_to, PrimeIterator, reduce_fraction};

    #[test]
    fn round_to_nickel() {
        let result = round_to(10.0 / 3.0, 0.05);
        assert_eq!(result, 3.35);
    }

    #[test]
    fn prime_numbers_up_to_20() {
        let mut primes = PrimeIterator::new();
        assert_eq!(primes.next(), Some(2));
        assert_eq!(primes.next(), Some(3));
        assert_eq!(primes.next(), Some(5));
        assert_eq!(primes.next(), Some(7));
        assert_eq!(primes.next(), Some(11));
        assert_eq!(primes.next(), Some(13));
        assert_eq!(primes.next(), Some(17));
        assert_eq!(primes.next(), Some(19));
    }

    #[test]
    fn factorize_1000000000() {
        let factors = factorize(1000000000);
        assert_eq!(
            factors,
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5]
        );
    }

    #[test]
    fn factorize_1000000001() {
        let factors = factorize(1000000001);
        assert_eq!(factors, vec![7, 11, 13, 19, 52579]);
    }

    #[test]
    fn reduce_factors() {
        assert_eq!(reduce_fraction(18, 6), (3, 1));
        assert_eq!(reduce_fraction(136, 150), (68, 75));
    }
}
