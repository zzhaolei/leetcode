struct Solution;

impl Solution {
    fn recursive(ans: &mut Vec<i32>, i: i32, n: i32) {
        let mut i = i;
        for _ in 0..=9 {
            if i > n {
                return;
            }
            ans.push(i);
            if i * 10 <= n {
                Solution::recursive(ans, i * 10, n);
            }
            i += 1;
        }
    }

    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        for i in 1..=n {
            if ans.contains(&i) {
                continue;
            }

            ans.push(i);
            if i * 10 <= n {
                Solution::recursive(&mut ans, i * 10, n);
            }
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::lexical_order(13));
    println!("{:?}", Solution::lexical_order(2));
    println!("{:?}", Solution::lexical_order(100));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::lexical_order(13),
            [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(Solution::lexical_order(2), [1, 2]);
        assert_eq!(
            Solution::lexical_order(100),
            [
                1, 10, 100, 11, 12, 13, 14, 15, 16, 17, 18, 19, 2, 20, 21, 22, 23, 24, 25, 26, 27,
                28, 29, 3, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 4, 40, 41, 42, 43, 44, 45, 46,
                47, 48, 49, 5, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 6, 60, 61, 62, 63, 64, 65,
                66, 67, 68, 69, 7, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 8, 80, 81, 82, 83, 84,
                85, 86, 87, 88, 89, 9, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99
            ]
        );
    }
}
