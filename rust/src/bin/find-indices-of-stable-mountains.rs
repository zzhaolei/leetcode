struct Solution;

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        height
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(i, _)| {
                if height[i - 1] > threshold {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>()
    }
}

fn main() {
    let r = Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2);
    println!("{:?}", r);
}
