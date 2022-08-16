struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace('.', "[.]")
    }
}

fn main() {
    println!("{}", Solution::defang_i_paddr("1.1.1.1".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1"
        );
    }
}
