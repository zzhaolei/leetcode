use std::cmp::Ordering;

struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn impl1(arr: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        let mut arr = arr;
        arr.sort_unstable();

        let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        let mut min: Option<i32> = None;
        for i in arr.windows(2) {
            let r = i[1] - i[0];
            match min {
                Some(a) => min = Some(a.min(r)),
                None => min = Some(r),
            }
            match map.get_mut(&r) {
                Some(v) => v.push(i.to_vec()),
                None => {
                    map.insert(r, vec![i.to_vec()]);
                }
            }
        }
        map.get(&min.unwrap()).unwrap().to_vec()
    }

    pub fn impl2(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();

        let mut ans: Vec<Vec<i32>> = vec![];
        let mut min: i32 = 1000000; // 10 ^ 6
        for i in arr.windows(2) {
            let i = i.to_vec();
            let v = i[1] - i[0];
            match v.cmp(&min) {
                Ordering::Less => {
                    ans = vec![i];
                    min = v;
                }
                Ordering::Equal => {
                    ans.push(i);
                }
                _ => {}
            }
        }
        ans
    }

    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        // Solution::impl1(arr)
        Solution::impl2(arr)
    }
}

fn main() {
    println!("{:?}", Solution::minimum_abs_difference(vec![4, 2, 1, 3]));
    println!(
        "{:?}",
        Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15])
    );
    println!(
        "{:?}",
        Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
            [[1, 2], [2, 3], [3, 4]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
            [[1, 3]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            [[-14, -10], [19, 23], [23, 27]]
        );
    }
}
