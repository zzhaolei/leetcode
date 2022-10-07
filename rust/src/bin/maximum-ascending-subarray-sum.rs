struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums.iter().sum::<i32>();
        }
        let (mut ans, mut temp, mut prev) = (nums[0], nums[0], nums[0]);
        for i in nums.into_iter().skip(1) {
            if prev >= i {
                ans = ans.max(temp);
                temp = i;
            } else {
                temp += i;
            }
            prev = i;
        }
        ans.max(temp)
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_ascending_sum(vec![3, 6, 10, 1, 8, 9, 9, 8, 9])
    );
    println!("{}", Solution::max_ascending_sum(vec![1]));
    println!(
        "{}",
        Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50])
    );
    println!("{}", Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]));
    println!(
        "{}",
        Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12])
    );
    println!("{}", Solution::max_ascending_sum(vec![100, 10, 1]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            19,
            Solution::max_ascending_sum(vec![3, 6, 10, 1, 8, 9, 9, 8, 9])
        );
        assert_eq!(1, Solution::max_ascending_sum(vec![1]));
        assert_eq!(65, Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]));
        assert_eq!(150, Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]));
        assert_eq!(
            33,
            Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12])
        );
        assert_eq!(100, Solution::max_ascending_sum(vec![100, 10, 1]));
    }
}
