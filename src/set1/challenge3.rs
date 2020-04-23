use crate::set1::challenge1::string_to_hex;
use std::collections::HashMap;

pub fn xor_bruteforce(input: String) -> (String, u32) {
    let hex = string_to_hex(input);
    let mut ret = String::new();
    let mut f = String::new();
    let mut rec = 0;

    let cf = CharFreq::new();

    for xor_key in 0x00..=0xFF {
        for n in &hex {
            f.push((n ^ xor_key) as char);
        }
        let count = cf.count(f.clone());
        if count > rec {
            rec = count;
            ret = f;
        }
        f = String::new();
    }

    (ret, rec as u32)
}

struct CharFreq {
    pub h: HashMap<char, u32>,
}

impl CharFreq {
    pub fn new() -> Self {
        let mut h = HashMap::new();
        h.insert(' ', 12802);
        h.insert('e', 12702);
        h.insert('t', 9056);
        h.insert('a', 8167);
        h.insert('o', 7507);
        h.insert('i', 6966);
        h.insert('n', 6749);
        h.insert('s', 6327);
        h.insert('h', 6094);
        h.insert('r', 5987);
        h.insert('d', 4253);
        h.insert('l', 4025);
        h.insert('c', 2782);
        h.insert('u', 2758);
        h.insert('m', 2406);
        h.insert('w', 2361);
        h.insert('f', 2228);
        h.insert('g', 2015);
        h.insert('y', 1974);
        h.insert('p', 1929);
        h.insert('b', 1492);
        h.insert('v', 978);
        h.insert('k', 772);
        h.insert('j', 153);
        h.insert('x', 150);
        h.insert('q', 95);
        h.insert('z', 74);

        Self {
            h
        }
    }

    pub fn count(&self, s: String) -> u32 {
        let mut ret = 0;
        for c in s.chars() {
            if let Some(x) = self.h.get(&c) {
                ret += x;
            }
        }
        ret
    }
}

#[test]
fn xor_bruteforce_test() {
    assert_eq!(("Cooking MC's like a pound of bacon".to_string(), 206915), xor_bruteforce("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string()));
}
