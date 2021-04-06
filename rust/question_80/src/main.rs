#![allow(unused)]
/// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array-ii/

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        let mut element = None;
        *nums = nums
            .iter_mut()
            .filter_map(|ele| {
                if let Some(prev) = element {
                    if prev == *ele {
                        count += 1;
                        if count >= 2 {
                            return None;
                        }
                        return Some(*ele);
                    }
                }
                count = 0;
                element = Some(*ele);
                element
            })
            .collect();
        nums.len() as i32
    }
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    println!("{}, {:?}", Solution::remove_duplicates(&mut nums), nums);
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    println!("{}, {:?}", Solution::remove_duplicates(&mut nums), nums);
}
