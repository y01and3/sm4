use crate::func::basis_func::{rot_l_32bit, tau_change};

use super::basis_func::add_into_list;

fn l_transform(input: u32) -> u32 {
    let mut temp = input;
    let n = [13, 23];
    for i in n {
        temp = temp ^ rot_l_32bit(temp, i);
    }
    temp
}

fn t_exchange(input: u32) -> u32 {
    l_transform(tau_change(input))
}

fn generate_new_key(key: [u32; 4], ck: u32) -> [u32; 4] {
    let k = key[0] ^ t_exchange(key[1] ^ key[2] ^ key[3] ^ ck);
    add_into_list(key, k)
}

pub fn key_generator(sk: [u32; 4]) -> [u32; 32] {
    let fk = [0xA3B1BAC6, 0x56AA3350, 0x677D9197, 0xB27022DC];
    let ck = [
        0x00070E15, 0x1C232A31, 0x383F464D, 0x545B6269, 0x70777E85, 0x8C939AA1, 0xA8AFB6BD,
        0xC4CBD2D9, 0xE0E7EEF5, 0xFC030A11, 0x181F262D, 0x343B4249, 0x50575E65, 0x6C737A81,
        0x888F969D, 0xA4ABB2B9, 0xC0C7CED5, 0xDCE3EAF1, 0xF8FF060D, 0x141B2229, 0x30373E45,
        0x4C535A61, 0x686F767D, 0x848B9299, 0xA0A7AEB5, 0xBCC3CAD1, 0xD8DFE6ED, 0xF4FB0209,
        0x10171E25, 0x2C333A41, 0x484F565D, 0x646B7279,
    ];
    let mut key = [sk[0] ^ fk[0], sk[1] ^ fk[1], sk[2] ^ fk[2], sk[3] ^ fk[3]];
    let mut rk = [0; 32];
    for i in 0..32 {
        key = generate_new_key(key, ck[i]);
        rk[i] = key[3];
    }
    rk
}
