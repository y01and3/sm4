pub mod func;
fn main() {
    let plain_text =0x0123456789abcdeffedcba9876543210;
    let sk = 0x0123456789abcdeffedcba9876543210;
    let enc_text = func::sm4::enc(plain_text, sk);
    let dec_text = func::sm4::dec(enc_text, sk);
    println!("plain_text: {:x}", plain_text);
    println!("enc_text: {:x}", enc_text);
    println!("dec_text: {:x}", dec_text);
}
