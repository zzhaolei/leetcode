struct Solution;

impl Solution {
    fn impl1(s: &mut Vec<char>) {
        s.reverse();
    }

    #[allow(unused)]
    fn impl2(s: &mut Vec<char>) {
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            (s[left], s[right]) = (s[right], s[left]);
            left += 1;
            right -= 1;
        }
    }

    pub fn reverse_string(s: &mut Vec<char>) {
        Solution::impl1(s);
    }
}

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut s);
    println!("{:?}", s);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, ['o', 'l', 'l', 'e', 'h']);

        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::impl2(&mut s);
        assert_eq!(s, ['o', 'l', 'l', 'e', 'h']);
    }
}
