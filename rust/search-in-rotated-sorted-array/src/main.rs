struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        for (i, v) in nums.iter().enumerate() {
            if *v == target {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    println!("{}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
}
