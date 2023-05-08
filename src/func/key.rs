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
