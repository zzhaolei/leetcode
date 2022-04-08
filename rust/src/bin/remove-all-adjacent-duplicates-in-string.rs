#![allow(unused)]

/// https://leetcode-cn.com/problems/remove-all-adjacent-duplicates-in-string/

struct Solution;

impl Solution {
    pub fn impl_1(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let mut s = s;

        let result = loop {
            let mut result = String::new();
            let mut dup = false;
            let mut i_c: Option<char> = None;

            for c in s.chars() {
                if let Some(cc) = i_c {
                    if cc == c {
                        i_c = None;
                        dup = true;
                    } else {
                        result.push(i_c.unwrap());
                        i_c = Some(c);
                    }
                } else {
                    i_c = Some(c);
                }
            }
            if let Some(c) = i_c {
                result.push(c);
            }
            if !dup {
                break result;
            }
            s = result;
        };

        result
    }

    pub fn impl_2(s: String) -> String {
        let mut r: Vec<char> = Vec::new();
        for c in s.chars() {
            match r.last() {
                Some(l) => {
                    if c == *l {
                        r.pop();
                    } else {
                        r.push(c);
                    }
                }

                None => {
                    r.push(c);
                }
            }
        }
        r.iter().collect::<String>()
    }

    pub fn impl_3(s: String) -> String {
        let mut r = String::new();
        for c in s.chars() {
            let last = r.pop();
            match last {
                Some(l) => {
                    if c != l {
                        r.push(l);
                        r.push(c);
                    }
                }

                None => {
                    r.push(c);
                }
            }
        }
        r
    }

    pub fn remove_duplicates(s: String) -> String {
        // Solution::impl_1(s)
        // Solution::impl_2(s)
        Solution::impl_3(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_1() {
        assert_eq!(Solution::impl_1("abbaca".to_string()), "ca");
    }

    #[test]
    fn impl_2() {
        assert_eq!(Solution::impl_2("abbaca".to_string()), "ca");
    }

    #[test]
    fn impl_3() {
        assert_eq!(Solution::impl_3("abbaca".to_string()), "ca");
    }
}
fn main() {}
