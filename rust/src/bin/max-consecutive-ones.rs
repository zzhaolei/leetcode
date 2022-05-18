struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count = 0;
        for i in nums.iter() {
            match i {
                1 => count += 1,
                0 => {
                    ans = ans.max(count);
                    count = 0;
                }
                _ => {}
            }
        }
        ans.max(count)
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
    );
    println!(
        "{}",
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
