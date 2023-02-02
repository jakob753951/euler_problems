pub fn solve(sum_of_numbers: u64) -> u64 {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_25() {
        let result = solve(3*3+4*4+5*5);
        assert_eq!(result, 5832);
    }
}
