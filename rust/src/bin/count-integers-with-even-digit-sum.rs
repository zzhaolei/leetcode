struct Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        (1..=num)
            .filter(|i| {
                i.to_string()
                    .chars()
                    .map(|x| x.to_digit(10).unwrap())
                    .sum::<u32>()
                    % 2
                    == 0
            })
            .collect::<Vec<i32>>()
            .len() as i32
    }
}

fn main() {
    println!("{}", Solution::count_even(1));
    println!("{}", Solution::count_even(4));
    println!("{}", Solution::count_even(30));
    println!("{}", Solution::count_even(1000));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(0, Solution::count_even(1));
        assert_eq!(2, Solution::count_even(4));
        assert_eq!(14, Solution::count_even(30));
        assert_eq!(499, Solution::count_even(1000));
    }
}
