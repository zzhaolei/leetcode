struct Solution;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        use std::collections::{HashMap, HashSet};

        let mut map: HashMap<char, u8> = HashMap::new();
        for i in word.chars() {
            match map.get_mut(&i) {
                Some(value) => *value += 1,
                None => {
                    map.insert(i, 1);
                }
            }
        }

        let mut v = map.into_values().collect::<Vec<u8>>();
        v.sort();
        v.reverse();

        let len = v.len();
        if len == 1
            || (len == 2 && v[len - 1] == 1)
            || (v[0] == v[len - 1] && v[0] == 1)
            || (v[len - 1] == 1 && v[..len - 1].iter().collect::<HashSet<&u8>>().len() == 1)
        {
            return true;
        }
        (v[1..].iter().collect::<HashSet<&u8>>().len() == 1) && v[0] - 1 == v[v.len() - 1]
    }
}

// 1,1,2

fn main() {
    println!("1 false == {}", Solution::equal_frequency("ddaccb".into()));
    println!("2 false == {}", Solution::equal_frequency("babbdd".into()));
    println!("3 false == {}", Solution::equal_frequency("aazz".into()));
    println!("4 true == {}", Solution::equal_frequency("abcc".into()));
    println!("5 true == {}", Solution::equal_frequency("abc".into()));
    println!("6 true == {}", Solution::equal_frequency("abbcc".into()));
    println!("7 true == {}", Solution::equal_frequency("cccaa".into()));
    println!("8 false == {}", Solution::equal_frequency("ccaa".into()));
    println!("9 true == {}", Solution::equal_frequency("ca".into()));
    println!("10 true == {}", Solution::equal_frequency("zz".into()));
    println!("11 true == {}", Solution::equal_frequency("cccd".into()));
    println!(
        "12 false == {}",
        Solution::equal_frequency("aaaabbbbccc".into())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(!Solution::equal_frequency("ddaccb".into()));
        assert!(!Solution::equal_frequency("babbdd".into()));
        assert!(!Solution::equal_frequency("aazz".into()));
        assert!(Solution::equal_frequency("abcc".into()));
        assert!(Solution::equal_frequency("abc".into()));
        assert!(Solution::equal_frequency("abbcc".into()));
        assert!(Solution::equal_frequency("cccaa".into()));
        assert!(!Solution::equal_frequency("ccaa".into()));
        assert!(Solution::equal_frequency("ca".into()));
        assert!(Solution::equal_frequency("zz".into()));
        assert!(Solution::equal_frequency("cccd".into()));
        assert!(!Solution::equal_frequency("aaaabbbbccc".into()));
    }
}
