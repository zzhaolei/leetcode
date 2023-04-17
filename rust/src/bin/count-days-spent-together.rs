struct Solution;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let split_str = |x: String| {
            x.split('-')
                .map(|x| unsafe { x.parse::<i32>().unwrap_unchecked() })
                .collect::<Vec<i32>>()
        };

        let calculate = |date: Vec<i32>| {
            let month = date[0];
            let day = date[1];
            months[..month as usize - 1].iter().sum::<i32>() + day
        };
        let arrive_alice = calculate(split_str(arrive_alice));
        let leave_alice = calculate(split_str(leave_alice));
        let arrive_bob = calculate(split_str(arrive_bob));
        let leave_bob = calculate(split_str(leave_bob));
        0.max(leave_alice.min(leave_bob) - arrive_alice.max(arrive_bob) + 1)
    }
}

fn main() {
    println!(
        "{}",
        Solution::count_days_together(
            "08-15".to_string(),
            "08-18".to_string(),
            "08-16".to_string(),
            "08-19".to_string()
        )
    );
    println!(
        "{}",
        Solution::count_days_together(
            "10-01".to_string(),
            "10-31".to_string(),
            "11-01".to_string(),
            "12-31".to_string()
        )
    );
}
