struct Solution;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut map: HashMap<String, u32> = HashMap::new();

        for i in cpdomains.into_iter() {
            let v = i.split(" ").into_iter().collect::<Vec<&str>>();
            let count = unsafe { v[0].parse::<u32>().unwrap_unchecked() };
            let domains = v[1].split(".").into_iter().collect::<Vec<&str>>();

            for j in 0..domains.len() {
                let domain = domains[j..].join(".");
                match map.get_mut(&domain) {
                    Some(v) => *v += count,
                    None => {
                        let _ = map.insert(domain, count);
                    }
                }
            }
        }
        let mut ans = Vec::with_capacity(map.len());
        for (k, v) in map.into_iter() {
            ans.push(format!("{v} {k}"));
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".into()])
    );
    println!(
        "{:?}",
        Solution::subdomain_visits(vec![
            "900 google.mail.com".into(),
            "50 yahoo.com".into(),
            "1 intel.mail.com".into(),
            "5 wiki.org".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let result = Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".into()]);
        for i in ["9001 com", "9001 discuss.leetcode.com", "9001 leetcode.com"] {
            assert!(result.contains(&i.to_owned()));
        }
    }
}
