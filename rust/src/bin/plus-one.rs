struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            digits[i] += 1;
            if digits[i] <= 9 {
                break;
            } else {
                digits[i] = 0;
                if i == 0 {
                    digits.insert(0, 1);
                }
            }
        }
        digits
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![1, 2, 8]), vec![1, 2, 9]);
        assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
