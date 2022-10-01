struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let number = number.replace(" ", "").replace("-", "");
        if number.len() <= 3 {
            return number;
        }

        let number = number
            .chars()
            .into_iter()
            .map(|x| x as u8)
            .collect::<Vec<u8>>();

        let mut ans: Vec<u8> = Vec::with_capacity(number.len() + 1);
        let mut i = 0;
        loop {
            match (number.len() - i).cmp(&4) {
                std::cmp::Ordering::Greater => {
                    ans.extend(&number[i..i + 3]);
                    ans.push(45);
                    i += 3;
                }
                std::cmp::Ordering::Less => {
                    ans.extend(&number[i..]);
                    break;
                }
                std::cmp::Ordering::Equal => {
                    ans.extend(&number[i..i + 2]);
                    ans.push(45);
                    ans.extend(&number[i + 2..]);
                    break;
                }
            }
        }
        unsafe { String::from_utf8(ans).unwrap_unchecked() }
    }
}

fn main() {
    println!("{}", Solution::reformat_number("12".to_owned()));
    println!("{}", Solution::reformat_number("123-".to_owned()));
    println!("{}", Solution::reformat_number("123 4-567".to_owned()));
    println!("{}", Solution::reformat_number("123 4-5678".to_owned()));
    println!(
        "{}",
        Solution::reformat_number("--17-5 229 35-39475 ".to_owned())
    );
}
