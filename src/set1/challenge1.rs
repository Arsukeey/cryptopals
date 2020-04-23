pub fn string_to_hex(input: String) -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    let mut i = input.chars();
    loop {
        let i1 = i.next();
        let i2 = i.next();
        match (i1, i2) {
            (Some(c1), Some(c2)) => {
                let n1 = char_to_hex(c1);
                let n2 = char_to_hex(c2) | (n1 << 4);
                v.push(n2);
            }
            (Some(_), _) => panic!("Expected a pair of numbers in string_to_hex."),
            _ => break,
        }
    }
    v
}

fn char_to_hex(c: char) -> u8 {
    match c {
        '0'..='9' => (c as u8 - '0' as u8),
        'a'..='f' => (c as u8 - 'a' as u8) + 10,
        'A'..='F' => (c as u8 - 'A' as u8) + 10,
        _ => panic!(format!("char_to_hex only converts 0-F chars, got {}.", c)),
    }
}

const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn get_last_bits(input: u32, n: u8) -> usize {
    if n < 32 {
        return (input & ((1 << n) - 1)) as usize;
    }
    0
}

pub fn hex_to_base64(input: &[u8]) -> String {
    let mut ret = String::new();

    let mut i = 0;
    loop {
        match (input.get(i), input.get(i + 1), input.get(i + 2)) {
            (Some(&n1), Some(&n2), Some(&n3)) => {
                let n = (n1 as u32) << 16 | (n2 as u32) << 8 | n3 as u32;
                ret.push(BASE64[(n >> 18) as usize]);
                ret.push(BASE64[get_last_bits(n >> 12, 6)]);
                ret.push(BASE64[get_last_bits(n >> 6, 6)]);
                ret.push(BASE64[get_last_bits(n, 6)]);
            }

            (Some(&n1), Some(&n2), None) => {
                let n = (n1 as u32) << 10 | (n2 as u32) << 2;
                ret.push(BASE64[(n >> 12) as usize]);
                ret.push(BASE64[get_last_bits(n >> 6, 6)]);
                ret.push(BASE64[get_last_bits(n, 6)]);
                ret.push('=');
            }

            (Some(&n1), None, None) => {
                let n = (n1 as usize) << 4;
                ret.push(BASE64[n >> 6]);
                ret.push(BASE64[n]);
                ret.push('=');
                ret.push('=');
            }

            (None, None, None) => break,

            _ => unreachable!(),
        }
        
        i += 3;
    }
    ret
}

#[test]
fn hex_to_base64_test() {
    let hex = string_to_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());
    let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(hex_to_base64(&hex), expected_output);
}
