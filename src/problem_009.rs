pub fn solve(sum_of_numbers: u64) -> u64 {
    for a in 1..sum_of_numbers {
        for b in a..sum_of_numbers {
            if a+b > sum_of_numbers {
                break;
            }
            for c in b..sum_of_numbers {
                if a+b+c != sum_of_numbers {
                    continue;
                }
                if a.pow(2) + b.pow(2) != c.pow(2) {
                    continue;
                }
                return a * b * c;
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_25() {
        let result = solve(3+4+5);
        assert_eq!(result, 3*4*5);
    }
}
