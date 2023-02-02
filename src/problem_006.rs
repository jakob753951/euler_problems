pub fn solve(limit: u64) -> u64 {
    let sum_of_squares: u64 = (1..=limit)
    .map(|num| num * num)
    .sum();

    let sum: u64 = (1..=limit).sum();
    let square_of_sum = sum * sum;

    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_10() {
        let result = solve(10);
        assert_eq!(result, 2640);
    }
}
