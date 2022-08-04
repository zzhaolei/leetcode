struct Solution;

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 || nums.len() == 2 && nums[0] == nums[1] {
            return nums;
        }
        let mut nums = nums;
        nums.sort();
        nums.reverse();

        let total = nums.iter().sum::<i32>();
        for i in 0..nums.len() {
            let left = nums[..i].iter().sum::<i32>();
            if left > total - left {
                return nums[..i].to_vec();
            }
        }
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution::min_subsequence(vec![4, 3, 10, 9, 8]));
    println!("{:?}", Solution::min_subsequence(vec![4, 4, 7, 6, 7]));
    println!("{:?}", Solution::min_subsequence(vec![8, 8]));
    println!("{:?}", Solution::min_subsequence(vec![6]));
}
