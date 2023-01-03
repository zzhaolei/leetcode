struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        s.split_whitespace()
            .filter_map(|x| x.parse::<u8>().ok())
            .collect::<Vec<u8>>()
            .windows(2)
            .all(|x| x[0] < x[1])
    }
}

fn main() {
    println!(
        "{}",
        Solution::are_numbers_ascending("4 5 11 26".to_owned())
    );
    println!(
        "{}",
        Solution::are_numbers_ascending("hello world 5 x 5".to_owned())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert!(Solution::are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_owned()
        ));
        assert!(!Solution::are_numbers_ascending(
            "hello world 5 x 5".to_owned()
        ));
        assert!(Solution::are_numbers_ascending("hello world 0".to_owned()));
        assert!(Solution::are_numbers_ascending("4 5 11 26".to_owned()));
        assert!(!Solution::are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_owned()
        ));
    }
}
