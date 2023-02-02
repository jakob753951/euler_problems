use std::collections::HashSet;

fn is_palindrome(n: u64) -> bool {
    let a = n.to_string().chars().collect::<Vec<char>>();
    let b = n.to_string().chars().rev().collect::<Vec<char>>();
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

pub fn solve(digits: u32) -> u64 {
    let mut palindromic_numbers: HashSet<u64> = HashSet::new();
    (10_u64.pow(digits-1)..10_u64.pow(digits))
    .for_each(|a|{
        (10_u64.pow(digits-1)..10_u64.pow(digits))
        .for_each(|b| {
            if is_palindrome(a * b) {
                palindromic_numbers.insert(a * b);
            }
        })
    });

    palindromic_numbers
    .into_iter()
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_2() {
        let result = solve(2);
        assert_eq!(result, 9009);
    }
}
