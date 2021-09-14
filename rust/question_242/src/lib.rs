#![allow(unused)]
/// https://leetcode-cn.com/problems/valid-anagram/

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut arr = [0_i32; 26];
        let mut index;
        let mut ascii;
        for c in s.chars().into_iter() {
            ascii = c as usize;
            index = ascii - 97;
            arr[index] += 1;
        }
        for c in t.chars().into_iter() {
            ascii = c as usize;
            index = ascii - 97;
            if let Some(value) = arr.get(index) {
                if value - 1 < 0 {
                    return false;
                }
                arr[index] = value - 1;
            } else {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn impl1() {
        let r = Solution::is_anagram("anagram".to_string(), "nagaram".to_string());
        assert_eq!(r, true);

        let r = Solution::is_anagram("rat".to_string(), "car".to_string());
        assert_eq!(r, false);
    }
}
