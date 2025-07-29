use aes::cipher::KeyIvInit;
use cfb_mode::Decryptor;
use aes::Aes128;

pub const AES_LENGTH: usize = 16;
type Aes128CfbDec = Decryptor<Aes128>;

pub fn replenish_byte(input: &str) -> [u8; AES_LENGTH] {
    let mut bytes = [0u8; AES_LENGTH];
    let input_bytes = input.as_bytes();
    let length = input_bytes.len().min(AES_LENGTH);
    bytes[..length].copy_from_slice(&input_bytes[..length]);
    bytes
}

pub fn create_decryptor(key: &str, iv: &str) -> Aes128CfbDec {
    let key = replenish_byte(key);
    let iv = replenish_byte(iv);
    Aes128CfbDec::new(&key.into(), &iv.into())
} 