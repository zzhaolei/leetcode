struct Solution;

impl Solution {
    fn dfs(s: &str, left: i32, right: i32, n: i32, ans: &mut Vec<String>) {
        if left > n || right > left {
            return;
        }
        if s.len() as i32 == n * 2 {
            ans.push(s.to_owned());
            return;
        }

        Solution::dfs(&(s.to_owned() + "("), left + 1, right, n, ans);
        Solution::dfs(&(s.to_owned() + ")"), left, right + 1, n, ans);
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n <= 0 {
            return vec![];
        }
        let mut ans: Vec<String> = Vec::new();
        Solution::dfs("", 0, 0, n, &mut ans);
        ans
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(1));
    println!("{:?}", Solution::generate_parenthesis(3));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::generate_parenthesis(1), ["()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
