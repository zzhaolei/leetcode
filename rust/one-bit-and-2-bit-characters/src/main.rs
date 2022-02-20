struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut bit = 0;
        let mut ans = true;
        for i in bits {
            if i == 1 || bit == 1 {
                bit = if bit + 1 == 2 { 0 } else { 1 };
                ans = false;
            } else {
                ans = true;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::is_one_bit_character(vec![1, 0, 0]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::is_one_bit_character(vec![1, 0, 0]), true);
        assert_eq!(Solution::is_one_bit_character(vec![1, 1, 1, 0]), false);
        assert_eq!(Solution::is_one_bit_character(vec![1, 1, 0, 0]), true);
        assert_eq!(Solution::is_one_bit_character(vec![1, 1, 0, 1]), false);
        assert_eq!(Solution::is_one_bit_character(vec![0, 1, 0, 0]), true);
        assert_eq!(Solution::is_one_bit_character(vec![0, 1, 0, 1]), false);
        assert_eq!(Solution::is_one_bit_character(vec![0]), true);
        assert_eq!(Solution::is_one_bit_character(vec![1]), false);
    }
}
