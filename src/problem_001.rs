fn is_divisible(a: &u32, b: &u32) -> bool {
	a % b == 0
}

pub fn solve(limit: u32) -> u32 {
	let checks = vec![3, 5];
	(1..limit)
	.filter(|x|
		checks
        .iter()
        .any(|check|
			is_divisible(x, check)
		)
	)
	.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_10() {
        let result = solve(10);
        assert_eq!(result, 23);
    }

    #[test]
    fn gives_correct_result_for_5() {
        let result = solve(6);
        assert_eq!(result, 8);
    }
}
