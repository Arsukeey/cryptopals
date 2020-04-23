fn repeating_xor(input: &[u8], key: String) -> String {
    let mut ret = String::new();
    let key_len = key.len() - 1;
    let key: Vec<u8> = key.chars().map(|x| x as u8).collect();

    let mut i = 0;
    let mut key_i = 0;
    loop {
        if let Some(s) = input.get(i) {
            ret.push(hex_to_char(s ^ key[key_i]));

            key_i += 1;
            i += 1;

            if key_i > key_len {
                key_i = 0;
            }
        } else {
            break;
        }
    }
    ret
}

fn hex_to_char(input: u8) -> char {
    (input + '0' as u8) as char
}

#[test]
fn test_repeating_xor() {
    let inp: Vec<u8> = "Burning 'em, if you ain't quick and nimble".chars().map(|x| x as u8).collect();

    assert_eq!("".to_string(),
               repeating_xor(
                   &inp, "ICE".to_string()));
}
