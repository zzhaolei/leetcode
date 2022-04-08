struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut v: Vec<Vec<char>> = Vec::new();
        for _ in 0..num_rows {
            v.push(Vec::new());
        }

        let num_rows = num_rows as usize - 1;
        let mut index = 0;
        let mut column = 0;
        let mut ele = num_rows - 1;
        for c in s.chars() {
            if column == 0 || column == num_rows {
                v[index].push(c);
            } else {
                v[ele].push(c);
                ele -= 1;
                if ele == 0 {
                    ele = num_rows - 1;
                    index = 0;
                }
                column += 1;
                if column >= num_rows {
                    column = 0;
                }
                continue;
            }

            index += 1;
            if index > num_rows {
                index = 0;
                column += 1;
                if column >= num_rows {
                    column = 0;
                }
            }
        }

        let mut ans = String::new();
        for i in v {
            for c in i {
                ans.push(c);
            }
        }
        ans
    }
}

fn main() {
    Solution::convert("PAYPALISHIRING".to_string(), 3);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }
}
