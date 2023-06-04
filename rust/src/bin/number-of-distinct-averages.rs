struct Solution;

#[derive(Hash, PartialEq, Eq)]
struct Float {
    a: i32,
    b: i32,
}

impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        use std::collections::HashSet;
        let mut set: HashSet<Float> = HashSet::new();

        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            set.insert(Float {
                a: (nums[i] + nums[j]) / 2,
                b: (nums[i] + nums[j]) % 2,
            });
            i += 1;
            j -= 1;
        }
        return set.len() as i32;
    }
}

fn main() {
    println!(
        "{}",
        Solution::distinct_averages(vec![9, 5, 7, 8, 7, 9, 8, 2, 0, 7])
    );
    println!("{}", Solution::distinct_averages(vec![4, 1, 4, 0, 3, 5]));
    println!("{}", Solution::distinct_averages(vec![1, 100]));
}
