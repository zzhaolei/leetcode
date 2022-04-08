#![allow(unused)]

struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        candy_type
            .iter()
            .copied()
            .collect::<std::collections::HashSet<i32>>()
            .len()
            .min(candy_type.len() / 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn distribute_candies() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3, 4, 5]), 3);
        assert_eq!(Solution::distribute_candies(vec![1, 1]), 1);
        assert_eq!(Solution::distribute_candies(vec![1, 2]), 1);
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(
            Solution::distribute_candies(vec![1000, 1000, 2, 1, 2, 5, 3, 1]),
            4
        );
    }
}
fn main() {}
