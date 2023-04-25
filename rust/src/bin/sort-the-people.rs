struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut result = names
            .into_iter()
            .zip(heights.into_iter())
            .collect::<Vec<(String, i32)>>();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result.into_iter().map(|(s, _)| s).collect::<Vec<String>>()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sort_people(
            vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
            vec![180, 165, 170],
        )
    );
    println!(
        "{:?}",
        Solution::sort_people(
            vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
            vec![155, 185, 150],
        )
    );
}
