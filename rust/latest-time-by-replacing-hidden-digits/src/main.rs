struct Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        if time == "??:??" {
            return "23:59".to_string();
        }
        let mut time = time.chars().collect::<Vec<char>>();
        if time[0] == '?' {
            if time[1] >= '4' && time[1] <= '9' {
                time[0] = '1';
            } else {
                time[0] = '2';
            }
        }
        if time[1] == '?' {
            if time[0] == '2' {
                time[1] = '3';
            } else {
                time[1] = '9';
            }
        }
        if time[3] == '?' {
            time[3] = '5';
        }
        if time[4] == '?' {
            time[4] = '9';
        }
        time.into_iter().collect::<String>()
    }
}

fn main() {
    println!("{}", Solution::maximum_time("??:??".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::maximum_time("??:??".to_string()), "23:59");
    }
}
