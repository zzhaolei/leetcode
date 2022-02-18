struct Solution;

impl Solution {
    fn impl1(edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let len = edges.len() as i32;
        for v in edges {
            for i in v {
                if let Some(m) = map.get_mut(&i) {
                    *m += 1;
                } else {
                    map.insert(i, 1);
                }
            }
        }
        for (k, v) in map {
            if v == len {
                return k;
            }
        }
        0
    }

    #[allow(unused)]
    fn impl2(edges: Vec<Vec<i32>>) -> i32 {
        let first = &edges[0];
        let second = &edges[1];
        for i in first {
            if second.contains(i) {
                return *i;
            }
        }
        0
    }

    #[allow(unused)]
    fn impl3(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
            edges[0][0]
        } else if edges[0][1] == edges[1][0] || edges[0][1] == edges[1][1] {
            edges[0][1]
        } else {
            0
        }
    }

    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        Solution::impl1(edges)
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::impl1(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
        assert_eq!(
            Solution::impl1(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }

    #[test]
    fn test_impl2() {
        assert_eq!(Solution::impl2(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
        assert_eq!(
            Solution::impl2(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }

    #[test]
    fn test_impl3() {
        assert_eq!(Solution::impl3(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
        assert_eq!(
            Solution::impl3(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
