struct MagicDictionary {
    vec: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        Self { vec: vec![] }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.vec.extend(dictionary);
    }

    fn search(&self, search_word: String) -> bool {
        let len = search_word.len();
        let words = search_word.bytes();
        for i in self.vec.iter().filter(|x| x.len() == len) {
            let mut num = 0;
            for (a, b) in i.bytes().zip(words.clone()) {
                if a != b {
                    num += 1;
                }
            }
            if num == 1 {
                return true;
            }
        }
        false
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */

fn main() {
    let mut obj = MagicDictionary::new();
    obj.build_dict(vec![
        "hello".to_string(),
        "leetcode".to_string(),
        "judge".to_string(),
    ]);
    println!("{}", obj.search("hello".to_string()));
    println!("{}", obj.search("hhllo".to_string()));
    println!("{}", obj.search("hell".to_string()));
    println!("{}", obj.search("leetcoded".to_string()));
    println!("{}", obj.search("judge".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::MagicDictionary;

    #[test]
    fn test() {
        let mut obj = MagicDictionary::new();
        obj.build_dict(vec![
            "hello".to_string(),
            "leetcode".to_string(),
            "judge".to_string(),
        ]);
        assert!(!obj.search("hello".to_string()));
        assert!(obj.search("hhllo".to_string()));
        assert!(!obj.search("hell".to_string()));
        assert!(!obj.search("leetcoded".to_string()));
        assert!(!obj.search("judge".to_string()));
    }
}
