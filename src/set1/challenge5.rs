fn repeating_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    let key_len = key.len() - 1;
    let mut key_i = 0;

    for h in input.iter() {
        if key_i > key_len {
            key_i = 0;
        }
        ret.push(h ^ key[key_i]);
        key_i += 1;
    }

    ret
}

fn hex_to_char(short: u8) -> char {
    match short {
        0x0..=0x9 => (short + '0' as u8) as char,
        0xa..=0xf => (short - 0xa + 'a' as u8) as char,
        _ => panic!("hex_to_char only converts short values between 0x0 and 0xf"),
    }
}

fn hex_to_string(hex: &[u8]) -> String {
    let mut ret = String::new();
    for h in hex.iter() {
        let hi = (h & 0xF0) >> 4;
        let lo = h & 0x0F;
        ret.push_str(&format!("{}{}", hex_to_char(hi), hex_to_char(lo)).to_string());
    }
    ret
}

#[test]
fn test_repeating_xor() {
    let key = "ICE";
    let x = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

    assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_string(),
               hex_to_string(&repeating_xor(
                   x.as_bytes(), key.as_bytes())));
}
