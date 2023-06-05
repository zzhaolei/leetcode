struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::HashSet;
        // 记录字符是否已经比较过
        let mut m: HashSet<char> = HashSet::new();
        'a: for (i, v) in s.chars().enumerate() {
            // 如果字符无法插入，则说明重复，已经比较过，直接continue
            if !m.insert(v) {
                continue;
            }
            for vv in s.chars().skip(i + 1) {
                if v == vv {
                    // 如果重复，则直接跳转到最外层for循环，继续下一次寻找
                    continue 'a;
                }
            }
            // 如果上面的for循环正常结束，则说明i对应的字符不重复
            return i as i32;
        }
        -1
    }
}

fn main() {
    println!("{}", Solution::first_uniq_char("loveleetcode".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let test = vec![
            ("leetcode", 0),
            ("loveleetcode", 2),
            ("aabb", -1),
            ("z", 0),
            ("dddccdbba", 8),
        ];
        for (s, r) in test {
            assert_eq!(Solution::first_uniq_char(s.to_string()), r);
        }
    }
}
