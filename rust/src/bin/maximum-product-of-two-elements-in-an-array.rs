struct Solution;

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        (nums[nums.len() - 1] - 1) * (nums[nums.len() - 2] - 1)
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![3, 4, 5, 2]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }
}
