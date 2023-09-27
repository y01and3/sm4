use super::basis_func::{rot_l_32bit, Exchange};

pub struct Sm4KeyGenerator {
    sk: [u32; 4],
    key: Option<[u32; 32]>,
}

impl Sm4KeyGenerator {
    pub fn new(sk: [u32; 4]) -> Self {
        Self { sk, key: None }
    }

    fn key_generator(&mut self) -> () {
        let fk = [0xA3B1BAC6, 0x56AA3350, 0x677D9197, 0xB27022DC];
        let ck = [
            0x00070E15, 0x1C232A31, 0x383F464D, 0x545B6269, 0x70777E85, 0x8C939AA1, 0xA8AFB6BD,
            0xC4CBD2D9, 0xE0E7EEF5, 0xFC030A11, 0x181F262D, 0x343B4249, 0x50575E65, 0x6C737A81,
            0x888F969D, 0xA4ABB2B9, 0xC0C7CED5, 0xDCE3EAF1, 0xF8FF060D, 0x141B2229, 0x30373E45,
            0x4C535A61, 0x686F767D, 0x848B9299, 0xA0A7AEB5, 0xBCC3CAD1, 0xD8DFE6ED, 0xF4FB0209,
            0x10171E25, 0x2C333A41, 0x484F565D, 0x646B7279,
        ];
        self.sk = [
            self.sk[0] ^ fk[0],
            self.sk[1] ^ fk[1],
            self.sk[2] ^ fk[2],
            self.sk[3] ^ fk[3],
        ];
        let mut rk = [0; 32];
        for i in 0..32 {
            self.sk = self.generate(self.sk, ck[i]);
            rk[i] = self.sk[3];
        }
        self.key = Some(rk);
    }

    pub fn get_key(&mut self) -> [u32; 32] {
        match self.key {
            Some(key) => key,
            None => {
                self.key_generator();
                match self.key {
                    Some(key) => key,
                    None => panic!("Error!"),
                }
            }
        }
    }

    pub fn _get_next_key(&mut self) -> [u32; 32] {
        match self.key {
            Some(_) => {
                self.key_generator();
            }
            None => {
                self.get_key();
            }
        }
        match self.key {
            Some(key) => key,
            None => panic!("Error!"),
        }
    }
}

impl Exchange for Sm4KeyGenerator {
    fn l_transform(&self, input: u32) -> u32 {
        input ^ rot_l_32bit(input, 13) ^ rot_l_32bit(input, 23)
    }
}
