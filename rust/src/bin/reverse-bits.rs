struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        let mut ans = 0;
        for _ in 0..32 {
            ans = (ans << 1) | (x & 1);
            x >>= 1;
        }
        ans
    }

    #[allow(unused)]
    fn impl2(x: u32) -> u32 {
        x.reverse_bits()
    }
}

fn main() {
    println!("{}", Solution::reverse_bits(43261596));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
        assert_eq!(Solution::impl2(43261596), 964176192);
    }
}
