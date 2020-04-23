use crate::set1::challenge3::xor_bruteforce;

pub fn xor_decrypt_find_highest_score(input: Vec<String>) -> String {
    let mut ret = String::new();
    let mut rec = 0;

    for string in input.iter() {
        let (s, score) = xor_bruteforce(string.to_string());
        if score > rec {
            rec = score;
            ret = s;
        }
    }

    ret
}

#[test]
fn test_xor_decrypt_find_highest_score() {
    use std::fs::File;
    use std::io::{self, BufRead};

    let file = File::open("4.txt").unwrap();
    let strings: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    assert_eq!("Now that the party is jumping\n".to_string(), xor_decrypt_find_highest_score(strings));
}
