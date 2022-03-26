struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.push(new_interval);
        intervals.sort_unstable_by(|i, j| i[0].cmp(&j[0]));

        let mut ans: Vec<Vec<i32>> = vec![];
        for i in intervals.into_iter() {
            if ans.is_empty() || i[0] > ans.last().unwrap()[1] {
                ans.push(i);
            } else if i[1] > ans.last().unwrap()[1] {
                let last = ans.last_mut().unwrap();
                *last = vec![last[0], i[1]];
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            [[1, 5], [6, 9]]
        )
    }
}
