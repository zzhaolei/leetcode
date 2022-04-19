struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut is = Vec::new();
        // 先找出所有目标字符的位置
        for (i, x) in s.chars().enumerate() {
            if x == c {
                is.push(i as i32);
            }
        }
        let mut ans: Vec<i32> = Vec::new();
        for (i, _) in s.chars().enumerate() {
            let i = i as i32;
            // 每一个字符都和目标字符位置比对，包含目标字符之前的字符和之后的字符，
            // 算出和目标字符最近的坐标位置
            let mut v = (is[0] - i).abs();
            for k in is.iter().skip(1) {
                v = (k - i).abs().min(v);
                if (k - i).abs() < v {
                    v = (k - i).abs();
                }
            }
            ans.push(v);
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::shortest_to_char("loveleetcode".to_string(), 'e')
    );
    println!("{:?}", Solution::shortest_to_char("aaab".to_string(), 'b'));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
            [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        assert_eq!(
            Solution::shortest_to_char("aaab".to_string(), 'b'),
            [3, 2, 1, 0]
        );
    }
}
