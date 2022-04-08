#![allow(unused)]

/// https://leetcode-cn.com/problems/merge-intervals/

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|x, y| x[0].cmp(&y[0]));

        let mut v: Vec<Vec<i32>> = Vec::new();
        for i in intervals.into_iter() {
            if v.is_empty() || v.last().unwrap()[1] < i[0] {
                // 如果当前v为空，则直接插入进去
                // 如果v中最后一个元素的end值，小于i的start元素，说明i此时还在v的最后一个元素的范围中。
                v.push(i);
            } else {
                // 获取结束范围值最大的数，作为end的值
                let end = v.last_mut().unwrap();
                end[1] = i32::max(end[1], i[1]);
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn merge() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![0, 4]]),
            vec![vec![0, 4]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![0, 1]]),
            vec![vec![0, 4]]
        );
    }
}
fn main() {}
