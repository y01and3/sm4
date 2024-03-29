use super::key::Sm4KeyGenerator;
use crate::func::basis_func::{rot_l_32bit, Exchange};

pub struct Sm4 {
    key: Sm4KeyGenerator,
    x: [u32; 4],
}

impl Sm4 {
    pub fn new(key: u128, hex: u128) -> Self {
        let key = Sm4KeyGenerator::new(cut_from_128bit(key));
        let x = cut_from_128bit(hex);
        Self { key, x }
    }

    pub fn encode(&mut self) -> u128 {
        let k = self.key.get_key();
        for i in 0..32 {
            self.x = self.generate(self.x, k[i]);
        }
        self.x.reverse();
        merge_to_128bit(self.x)
    }
    pub fn decode(&mut self) -> u128 {
        let k = self.key.get_key();
        for i in (0..32).rev() {
            self.x = self.generate(self.x, k[i]);
        }
        self.x.reverse();
        merge_to_128bit(self.x)
    }
}

impl Exchange for Sm4 {
    fn l_transform(&self, input: u32) -> u32 {
        input
            ^ rot_l_32bit(input, 2)
            ^ rot_l_32bit(input, 10)
            ^ rot_l_32bit(input, 18)
            ^ rot_l_32bit(input, 24)
    }
}

fn cut_from_128bit(input: u128) -> [u32; 4] {
    let mut temp: [u32; 4] = [0; 4];
    for i in 0..4 {
        temp[i] = (input >> (32 * (3 - i))) as u32;
    }
    temp
}

fn merge_to_128bit(input: [u32; 4]) -> u128 {
    let mut temp: u128 = 0;
    for i in 0..4 {
        temp += (input[i] as u128) << (32 * (3 - i));
    }
    temp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sm4_encode() {
        let mut sm4 = Sm4::new(
            0x0123456789abcdeffedcba9876543210,
            0x0123456789abcdeffedcba9876543210,
        );
        assert_eq!(sm4.encode(), 0x681edf34d206965e86b3e94f536e4246);
    }

    #[test]
    fn test_sm4_decode() {
        let mut sm4 = Sm4::new(
            0x0123456789abcdeffedcba9876543210,
            0x681edf34d206965e86b3e94f536e4246,
        );
        assert_eq!(sm4.decode(), 0x0123456789abcdeffedcba9876543210);
    }
}
