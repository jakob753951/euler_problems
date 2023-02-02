struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

pub fn solve(limit: u64) -> u64 {
	fibonacci()
	.take_while(|x| x <= &limit)
	.filter(|x| x % 2 == 0)
	.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_result_for_100() {
        let result = solve(100);
        assert_eq!(result, 44);
    }
}
