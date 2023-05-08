use crate::func::basis_func::{rot_l_32bit, tau_change};

 fn add_into_list(input: [u32; 4], x_4: u32) -> [u32; 4] {
    [input[1], input[2], input[3], x_4]
}

fn l_transform(input: u32) -> u32 {
    let mut temp = input;
    let n = [2, 10, 18, 24];
    for i in n {
        temp = temp ^ rot_l_32bit(temp, i);
    }
    temp
}

fn t_exchange(input: u32) -> u32 {
    l_transform(tau_change(input))
}

pub fn round(x: [u32; 4], round_key: u32) -> [u32; 4] {
    let x_4: u32 = t_exchange(x[1] ^ x[2] ^ x[3] ^ round_key);
    add_into_list(x, x_4)
}
