use crate::set1::challenge1::{string_to_hex, get_last_bits};

fn xor_combination(s1: String, s2: String) -> Vec<u8> {
    let mut ret = vec!();

    let r1 = string_to_hex(s1);
    let r2 = string_to_hex(s2);

    let mut i = 0;
    loop {
        match (r1.get(i), r2.get(i)) {
            (Some(x1), Some(x2)) => {
                let r11 = x1 >> 4;
                let r12 = get_last_bits(*x1 as u32, 4);

                let r21 = x2 >> 4;
                let r22 = get_last_bits(*x2 as u32, 4);

                let rr1 = r11 ^ r21;
                let rr2 = r12 ^ r22;

                ret.push((rr1 as u8) << 4 | rr2 as u8);
            }
            (Some(_), None) => panic!("Half pair in xor_combination."),
            _ => break,
        }
        i += 1;
    }
    ret
}

#[test]
fn xor_combination_test() {
    let s1 = "1c0111001f010100061a024b53535009181c".to_string();
    let s2 = "686974207468652062756c6c277320657965".to_string();
    let r = string_to_hex("746865206b696420646f6e277420706c6179".to_string());
    assert_eq!(xor_combination(s1, s2), r)
}
