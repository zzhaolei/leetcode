struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        s.split('|')
            .enumerate()
            .filter(|(i, _)| (i + 1) % 2 != 0)
            .map(|(_, x)| x.chars().filter(|c| *c == '*').count())
            .sum::<usize>() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".into())
    );
    println!("{}", Solution::count_asterisks("l|*e*et|c**o|*de|".into()));
    println!("{}", Solution::count_asterisks("iamprogrammer".into()));
}
