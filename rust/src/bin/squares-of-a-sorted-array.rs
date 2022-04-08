struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for i in nums.into_iter() {
            ans.push(i * i);
        }
        ans.sort_unstable();
        ans
    }
}

fn main() {
    println!("{:?}", Solution::sorted_squares(vec![-4, -1, 0, 3, 10]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            [0, 1, 9, 16, 100]
        );
    }
}
