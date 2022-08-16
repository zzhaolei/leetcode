struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let n = n as usize;
        if n % 2 != 0 {
            "a".repeat(n)
        } else {
            let mut ans = String::with_capacity(n);
            ans.push_str(&"a".repeat(n - 1));
            ans.push('b');
            ans
        }
    }
}

fn main() {
    println!("{}", Solution::generate_the_string(78));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::generate_the_string(78),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"
        );
    }
}
