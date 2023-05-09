use crate::func::{
    basis_func::{cut_from_128bit, merge_to_128bit},
    f::round,
    key::key_generator,
};

pub fn enc(plain_text: u128, sk: u128) -> u128 {
    let key = key_generator(cut_from_128bit(sk));
    let mut x = cut_from_128bit(plain_text);
    for i in 0..32 {
        x = round(x, key[i]);
    }
    x.reverse();
    merge_to_128bit(x)
}

pub fn dec(enc_text: u128, sk: u128) -> u128 {
    let key = key_generator(cut_from_128bit(sk));
    let mut x = cut_from_128bit(enc_text);
    for i in (0..32).rev() {
        x = round(x, key[i]);
    }
    x.reverse();
    merge_to_128bit(x)
}