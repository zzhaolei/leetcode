struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut arr_sort = arr.clone();
        arr_sort.sort();

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut i = 0;
        for v in arr_sort.into_iter() {
            match map.get(&v) {
                Some(_) => (),
                None => {
                    i += 1;
                    map.insert(v, i);
                }
            }
        }
        let mut ans = vec![];
        for i in arr.into_iter() {
            if let Some(&v) = map.get(&i) {
                ans.push(v);
            }
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::array_rank_transform(vec![1, 1, 1]));
    println!("{:?}", Solution::array_rank_transform(vec![4, 1, 2, 3]));
    println!(
        "{:?}",
        Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::array_rank_transform(vec![1, 1, 1]), [1, 1, 1]);
        assert_eq!(
            Solution::array_rank_transform(vec![4, 1, 2, 3]),
            [4, 1, 2, 3]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            [5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
