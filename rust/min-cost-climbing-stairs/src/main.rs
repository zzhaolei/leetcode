/// 现在一个台阶的费用为上次两个台阶的费用，因为一直在往前走
/// 例如:
///     索引: 0,  1,    2,  3,  4,  5,    6,  7,  8,    9
///     元素: 1,  100,  1,  1,  1,  100,  1,  1,  100,  1
///     步骤: 0,  0,    1,  2,  2,  3,    3,  4,  4,    5,  6
///
/// i = 2:  0 + 1    <  0 + 100,  result: ans_1=0, ans_2=1
/// i = 3:  0 + 100  >  1 + 1,    result: ans_1=1, ans_2=2
/// i = 4:  1 + 1    >  2 + 1,    result: ans_1=2, ans_2=2
/// i = 5:  2 + 1    >  2 + 1,    result: ans_1=2, ans_2=3
/// i = 6:  2 + 1    >  3 + 100,  result: ans_1=3, ans_2=3
/// i = 7:  3 + 100  >  3 + 1,    result: ans_1=3, ans_2=4
/// i = 8:  3 + 1    >  4 + 1,    result: ans_1=4, ans_2=4
/// i = 9:  4 + 1    >  4 + 100,  result: ans_1=4, ans_2=5
/// i = 10: 4 + 100  >  5 + 1,    result: ans_1=5, ans_2=6

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut ans_1 = 0; // 1个台阶
        let mut ans_2 = 0; // 2个台阶
        for i in 2..=cost.len() {
            // 将ans_2赋值给ans_1，重新计算ans_2的值。
            let t = ans_2;
            ans_2 = (ans_1 + cost[i - 2]).min(ans_2 + cost[i - 1]);
            ans_1 = t;

            // (ans_1, ans_2) = (ans_2, (ans_1 + cost[i - 2]).min(ans_2 + cost[i - 1]));
        }
        // 最后一个就是爬完整个楼梯需要的花费
        ans_2
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 1, 1, 1]), 1);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 2, 2, 1]), 2);
    }
}
