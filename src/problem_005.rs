fn is_divisible(a: &u64, b: &u64) -> bool {
	a % b == 0
}

pub fn solve(limit: u64) -> u64 {
    (1..).find(|n| {
        (1..limit)
        .all(|divisor| is_divisible(n, &divisor))
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_10() {
        let result = solve(10);
        assert_eq!(result, 2520);
    }
}
