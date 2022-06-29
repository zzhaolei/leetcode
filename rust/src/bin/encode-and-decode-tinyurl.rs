#![allow(non_snake_case)]
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, longURL: String) -> String {
        longURL
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        shortURL
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

fn main() {
    let code = Codec::new();
    println!("{}", code.decode(code.encode("1".to_string())));
}
