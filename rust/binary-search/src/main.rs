struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let (mut left, mut right) = (0, nums.len() - 1);
        let mut mid;
        while left <= right {
            mid = left + (right - left) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => left += 1,
                Ordering::Greater => {
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                }
            }
        }
        -1
    }
}

fn main() {
    println!("{}", Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
    println!("{}", Solution::search(vec![-1, 0, 3, 5, 9, 12], 10));
    println!("{}", Solution::search(vec![-1, 0, 3, 5, 9, 12], 12));
    println!("{}", Solution::search(vec![1, 0, 3, 5, 9, 12], 2));
    println!("{}", Solution::search(vec![5], -5));
}
