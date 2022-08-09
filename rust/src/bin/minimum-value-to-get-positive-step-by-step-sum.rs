struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        loop {
            let mut sum = ans;
            let mut f = true;
            for i in nums.iter() {
                sum += i;
                if sum < 1 {
                    ans += 1;
                    f = false;
                    break;
                }
            }
            if f {
                break;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::min_start_value(vec![-3, 2, -3, 4, 2]));
    println!("{}", Solution::min_start_value(vec![1, 2]));
    println!("{}", Solution::min_start_value(vec![1, -2, -3]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    }
}
