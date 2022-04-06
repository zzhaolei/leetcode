struct Solution;

impl Solution {
    fn impl1(s: String, k: i32) -> String {
        let mut s = s.replace('-', "").to_uppercase();
        let (len, k) = (s.len(), k as usize);
        if len % k != 0 {
            s = format!("{}{}", " ".repeat(k - len % k), s);
        }
        let mut ans: Vec<String> = Vec::new();
        let mut i = 0;
        while i < len {
            ans.push(s.drain(..k).collect::<String>());
            i += k;
        }
        ans.join("-").trim().to_string()
    }

    fn impl2(s: String, k: i32) -> String {
        let mut ans = String::new();
        let mut count = 0;
        s.to_ascii_uppercase().chars().rev().for_each(|c| {
            if c != '-' {
                ans.push(c);
                count += 1;
                if count == k {
                    count = 0;
                    ans.push('-');
                }
            }
        });
        ans.trim_end_matches('-').chars().rev().collect::<String>()
    }

    pub fn license_key_formatting(s: String, k: i32) -> String {
        Solution::impl1(s, k)
    }
}

fn main() {
    println!(
        "{}",
        Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4)
    );
    println!("{}", Solution::impl1("2-5g-3-J".to_string(), 2));
    println!("{}", Solution::impl2("2-5g-3-J".to_string(), 2));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::impl1("5F3Z-2e-9-w".to_string(), 4), "5F3Z-2E9W");
        assert_eq!(Solution::impl2("5F3Z-2e-9-w".to_string(), 4), "5F3Z-2E9W");
        assert_eq!(
            Solution::license_key_formatting("2-5g-3-J".to_string(), 2),
            "2-5G-3J"
        );
    }
}
