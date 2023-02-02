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

		if self.primes.len() == 1 {
			self.primes.push(3);
			return Some(3);
		}

        let next_prime = (self.primes.last().unwrap() + 2..)
		.step_by(2)
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

pub fn solve(limit: u64) -> u64 {
    primes()
	.take_while(|number| number < &limit)
	.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_10() {
        let result = solve(10);
        assert_eq!(result, 17);
    }
}
