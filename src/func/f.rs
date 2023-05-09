use crate::func::basis_func::{add_into_list, rot_l_32bit, tau_change};

fn l_transform(input: u32) -> u32 {
    input ^ rot_l_32bit(input, 2) ^ rot_l_32bit(input, 10) ^ rot_l_32bit(input, 18) ^ rot_l_32bit(input, 24)
}

fn t_exchange(input: u32) -> u32 {
    l_transform(tau_change(input))
}

pub fn round(x: [u32; 4], round_key: u32) -> [u32; 4] {

    let x_4: u32 = x[0] ^ t_exchange(x[1] ^ x[2] ^ x[3] ^ round_key);
    println!("{:x} ", x_4);
    add_into_list(x, x_4)
}
