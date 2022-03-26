struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}

fn main() {
    println!("{}", Solution::majority_element(vec![3, 2, 3]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
