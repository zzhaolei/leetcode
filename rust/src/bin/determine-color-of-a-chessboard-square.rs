struct Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        coordinates.bytes().sum::<u8>() % 2 != 0
    }
}

fn main() {
    println!("{}", Solution::square_is_white("a1".to_string()));
    println!("{}", Solution::square_is_white("h3".to_string()));
    println!("{}", Solution::square_is_white("c7".to_string()));
    println!("{}", Solution::square_is_white("h8".to_string()));
}
