struct Solution;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let v = word
            .bytes()
            .map(|i| if (48..=57).contains(&i) { i } else { 32 })
            .collect::<Vec<u8>>();
        let binding = String::from_utf8_lossy(&v);
        let mut v = binding
            .split_ascii_whitespace()
            .map(|x| x.trim_start_matches('0'))
            .collect::<Vec<&str>>();
        v.sort();
        v.dedup();
        v.len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_different_integers("a123bc34d8ef34".to_string())
    );
    println!(
        "{}",
        Solution::num_different_integers("leet1234code234".to_string())
    );
    println!(
        "{}",
        Solution::num_different_integers("a1b01c001".to_string())
    );
    println!(
        "{}",
        Solution::num_different_integers("2393706880236110407059624696967828762752651982730115221690437821508229419410771541532394006597463715513741725852432559057224478815116557380260390432211227579663571046845842281704281749571110076974264971989893607137140456254346955633455446057823738757323149856858154529105301197388177242583658641529908583934918768953462557716z97438020429952944646288084173334701047574188936201324845149110176716130267041674438237608038734431519439828191344238609567530399189316846359766256507371240530620697102864238792350289978450509162697068948604722646739174590530336510475061521094503850598453536706982695212493902968251702853203929616930291257062173c79487281900662343830648295410".to_string())
    );
}
