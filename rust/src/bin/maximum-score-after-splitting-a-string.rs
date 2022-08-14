struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        (0..s.len() - 1).fold(0, |ans, i| {
            ans.max(
                &s[..=i]
                    .iter()
                    .fold(0, |acc, x| if *x == '0' { acc + 1 } else { acc })
                    + &s[i + 1..]
                        .iter()
                        .fold(0, |acc, x| if *x == '1' { acc + 1 } else { acc }),
            )
        })
    }
}

fn main() {
    println!("{}", Solution::max_score("011101".to_owned()));
    println!("{}", Solution::max_score("00111".to_owned()));
    println!("{}", Solution::max_score("1111".to_owned()));
    println!("{}", Solution::max_score("00".to_owned()));
}
