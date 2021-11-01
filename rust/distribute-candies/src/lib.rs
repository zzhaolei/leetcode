struct Solution;

impl Solution {
    pub fn distribute_candies(mut candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let candy_len = candy_type.len();
        let half = (candy_len / 2) as i32;

        let c: HashSet<i32> = candy_type.drain(..).collect();

        let len = c.len() as i32;
        if len == 1 || len == half || len < half {
            return len;
        }

        return half;
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
