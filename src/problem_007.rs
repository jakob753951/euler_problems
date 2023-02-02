fn is_divisible(a: &u64, b: &u64) -> bool {
	a % b == 0
}

struct Primes {
    primes: Vec<u64>,
}

impl Iterator for Primes {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {

		if self.primes.is_empty() {
			self.primes.push(2);
			return Some(2);
		}

        let next_prime = (self.primes.last().unwrap() + 1..)
        .find(|n|{
            self.primes
            .iter()
            .all(|prime| !is_divisible(n, prime))
        });

        self.primes.push(next_prime.unwrap());

        next_prime
    }
}

fn primes() -> Primes {
    Primes { primes: vec![] }
}

pub fn solve(n: u64) -> u64 {
    primes()
    .skip(n as usize - 1)
    .next()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_6() {
        let result = solve(6);
        assert_eq!(result, 13);
    }
}
