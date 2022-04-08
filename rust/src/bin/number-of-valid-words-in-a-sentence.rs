struct Solution;

impl Solution {
    pub fn valid_word(s: &str) -> bool {
        let end = s.len() - 1;
        let mut a = 0;
        let mut flag = true;
        for (i, c) in s.chars().enumerate() {
            if ('a'..='z').any(|x| x == c) {
                flag = true;
                continue;
            }
            if !flag {
                return false;
            }
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    return false;
                }
                '-' => {
                    if a + 1 >= 2 || i == 0 || i == end {
                        return false;
                    }
                    a += 1;
                    flag = false;
                }
                '!' | '.' | ',' => {
                    if i != end {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }

    pub fn impl1(sentence: String) -> i32 {
        let mut ans = 0;
        for i in sentence.split_whitespace() {
            if Solution::valid_word(i) {
                ans += 1;
            }
        }
        ans
    }

    pub fn count_valid_words(sentence: String) -> i32 {
        Solution::impl1(sentence)
    }
}

fn main() {
    println!(
        "{}",
        Solution::count_valid_words(" 62   nvtk0wr4f  8 qt3r! w1ph 1l ,e0d 0n 2v 7c.  n06huu2n9 s9   ui4 nsr!d7olr  q-, vqdo!btpmtmui.bb83lf g .!v9-lg 2fyoykex uy5a 8v whvu8 .y sc5 -0n4 zo pfgju 5u 4 3x,3!wl  fv4   s  aig cf j1 a i  8m5o1  !u n!.1tz87d3 .9    n a3  .xb1p9f  b1i a j8s2 cugf l494cx1! hisceovf3 8d93 sg 4r.f1z9w   4- cb r97jo hln3s h2 o .  8dx08as7l!mcmc isa49afk i1 fk,s e !1 ln rt2vhu 4ks4zq c w  o- 6  5!.n8ten0 6mk 2k2y3e335,yj  h p3 5 -0  5g1c  tr49, ,qp9 -v p  7p4v110926wwr h x wklq u zo 16. !8  u63n0c l3 yckifu 1cgz t.i   lh w xa l,jt   hpi ng-gvtk8 9 j u9qfcd!2  kyu42v dmv.cst6i5fo rxhw4wvp2 1 okc8!  z aribcam0  cp-zp,!e x  agj-gb3 !om3934 k vnuo056h g7 t-6j! 8w8fncebuj-lq    inzqhw v39,  f e 9. 50 , ru3r  mbuab  6  wz dw79.av2xp . gbmy gc s6pi pra4fo9fwq k   j-ppy -3vpf   o k4hy3 -!..5s ,2 k5 j p38dtd   !i   b!fgj,nx qgif ".to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_impl1() {
        assert_eq!(Solution::count_valid_words("cat and  dog".to_string()), 3);
        assert_eq!(
            Solution::count_valid_words("!this  1-s b8d!".to_string()),
            0
        );
        assert_eq!(
            Solution::count_valid_words("alice and  bob are playing stone-game10".to_string()),
            5
        );
        assert_eq!(
            Solution::count_valid_words(
                "he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.".to_string()
            ),
            6
        );
        assert_eq!(Solution::count_valid_words(".".to_string()), 1);
        assert_eq!(Solution::count_valid_words(",".to_string()), 1);
        assert_eq!(Solution::count_valid_words("!".to_string()), 1);
        assert_eq!(Solution::count_valid_words("I1".to_string()), 0);
        assert_eq!(Solution::count_valid_words("q-,".to_string()), 0);
    }
}
