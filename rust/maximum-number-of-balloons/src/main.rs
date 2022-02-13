struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut b = 0;
        let mut a = 0;
        let mut ls = 0;
        let mut os = 0;
        let mut n = 0;

        for i in text.chars() {
            match i {
                'b' => b += 1,
                'a' => a += 1,
                'l' => ls += 1,
                'o' => os += 1,
                'n' => n += 1,
                _ => continue,
            }
        }
        b.min(a).min(ls / 2).min(os / 2).min(n)
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_number_of_balloons("nlaebolko".to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn testt_impl1() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
        assert_eq!(
            Solution::max_number_of_balloons("loonbalxballpoon".to_string()),
            2
        );
        assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
        assert_eq!(Solution::max_number_of_balloons("hpitp".to_string()), 0);
        assert_eq!(Solution::max_number_of_balloons("krhizmmgmcrecekgyljqkldocicziihtgpqwbticmvuyznragqoyrukzopfmjhjjxemsxmrsxuqmnkrzhgvtgdgtykhcglurvppvcwhrhrjoislonvvglhdciilduvuiebmffaagxerjeewmtcwmhmtwlxtvlbocczlrppmpjbpnifqtlninyzjtmazxdbzwxthpvrfulvrspycqcghuopjirzoeuqhetnbrcdakilzmklxwudxxhwilasbjjhhfgghogqoofsufysmcqeilaivtmfziumjloewbkjvaahsaaggteppqyuoylgpbdwqubaalfwcqrjeycjbbpifjbpigjdnnswocusuprydgrtxuaojeriigwumlovafxnpibjopjfqzrwemoinmptxddgcszmfprdrichjeqcvikynzigleaajcysusqasqadjemgnyvmzmbcfrttrzonwafrnedglhpudovigwvpimttiketopkvqw".to_string()), 10);
    }
}
