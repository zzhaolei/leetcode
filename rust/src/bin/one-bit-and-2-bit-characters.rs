struct Solution;

impl Solution {
    fn impl1(bits: Vec<i32>) -> bool {
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

    #[allow(unused)]
    fn impl2(bits: Vec<i32>) -> bool {
        let mut s = 0;
        while s < bits.len() - 1 {
            s = if bits[s] == 0 { s + 1 } else { s + 2 };
        }
        s == bits.len() - 1
    }

    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        Solution::impl1(bits)
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
        assert!(Solution::impl1(vec![1, 0, 0]));
        assert!(!Solution::impl1(vec![1, 1, 1, 0]));
        assert!(Solution::impl1(vec![1, 1, 0, 0]));
        assert!(Solution::impl1(vec![0, 1, 0, 0]));
        assert!(Solution::impl1(vec![0]));
    }

    #[test]
    fn test_impl2() {
        assert!(Solution::impl2(vec![1, 0, 0]));
        assert!(!Solution::impl2(vec![1, 1, 1, 0]));
        assert!(Solution::impl2(vec![1, 1, 0, 0]));
        assert!(Solution::impl2(vec![0, 1, 0, 0]));
        assert!(Solution::impl2(vec![0]));
    }
}
