struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut ans = 0;
        let (mut i, mut j) = (0, 1);
        while j < nums.len() {
            let mut x = nums[j] - nums[i];
            if x <= 0 {
                x = x.abs() + 1;
                ans += x;
                nums[j] += x;
            }
            i += 1;
            j += 1;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::min_operations(vec![1, 1, 1]));
    println!("{}", Solution::min_operations(vec![1]));
    println!("{}", Solution::min_operations(vec![1, 5, 2, 4, 1]));
}
