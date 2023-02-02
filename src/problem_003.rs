use std::collections::HashSet;

fn is_divisible(a: &u64, b: &u64) -> bool {
	a % b == 0
}

fn get_lowest_prime_factor(n: u64) -> u64 {
    (2..=n)
    .find(|i| is_divisible(&n, i))
    .unwrap()
}

pub fn solve(mut n: u64) -> u64 {
    // get prime factors of n
    let mut prime_factors: HashSet<u64> = HashSet::new();
    while n > 1 {
        let lowest_prime_factor = get_lowest_prime_factor(n);
        prime_factors.insert(lowest_prime_factor);
        n = n / lowest_prime_factor;
    }

    prime_factors
    .into_iter()
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_13195() {
        let result = solve(13195);
        assert_eq!(result, 29);
    }
}
