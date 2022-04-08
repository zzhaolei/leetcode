struct Solution;

impl Solution {
    fn impl1(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        ans.extend(nums);
        ans
    }

    fn impl2(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for _ in 0..2 {
            for i in nums.iter() {
                ans.push(*i);
            }
        }
        ans
    }

    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        Solution::impl1(nums)
    }
}

fn main() {
    println!("{:?}", Solution::get_concatenation(vec![1, 2, 1]));
    println!("{:?}", Solution::impl2(vec![1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::impl1(vec![1, 2, 1]), [1, 2, 1, 1, 2, 1]);
        assert_eq!(Solution::impl1(vec![1, 3, 2, 1]), [1, 3, 2, 1, 1, 3, 2, 1]);

        assert_eq!(Solution::impl2(vec![1, 2, 1]), [1, 2, 1, 1, 2, 1]);
        assert_eq!(Solution::impl2(vec![1, 3, 2, 1]), [1, 3, 2, 1, 1, 3, 2, 1]);
    }
}
