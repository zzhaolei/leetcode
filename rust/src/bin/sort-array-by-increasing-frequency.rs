struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in nums.iter() {
            let entry = map.entry(*i).or_insert(0);
            *entry += 1;
        }
        let mut vec = map
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<Vec<(i32, i32)>>();
        vec.sort_by(|a, b| {
            let order = a.1.cmp(&b.1);
            match order {
                std::cmp::Ordering::Equal => {
                    if a.0.cmp(&b.0).is_gt() {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                }
                _ => order,
            }
        });
        let mut ans = vec![];
        for (v, i) in vec {
            for _ in 0..i {
                ans.push(v);
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1])
    );
    println!("{:?}", Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1],
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1])
        );
        assert_eq!(
            vec![1, 3, 3, 2, 2],
            Solution::frequency_sort(vec![2, 3, 1, 3, 2])
        );
    }
}
