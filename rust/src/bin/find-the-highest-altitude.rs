struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (mut ans, mut prev) = (0, 0);
        gain.iter().for_each(|v| {
            prev += v;
            ans = ans.max(prev);
        });
        ans
    }
}

fn main() {
    println!("{}", Solution::largest_altitude(vec![-5, 1, 5, 0, -7]));
    println!(
        "{}",
        Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2])
    );
}
