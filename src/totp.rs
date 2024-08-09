use std::cmp::Ordering;
use data_encoding::{BASE32_NOPAD, HEXLOWER};
use hmac::{Hmac, Mac};
use sha1::Sha1;

pub struct Totp {
    secret: String,
}

impl Totp {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate(&self) -> String {
        let counter: u128 = Totp::get_counter();

        self.generate_hotp(counter)
    }

    pub fn verify(&self, input: &String) -> Result<(), ()> {
        let code = self.generate();

        if code.cmp(input) == Ordering::Equal {
            return Ok(());
        }
        Err(())
    }

    fn get_counter() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

        now / 1000 / 30
    }

    fn generate_hotp(&self, counter: u128) -> String {
        let message = format!("{:016x}", counter);
        let digest = self.compute_hmac_sha1(message);

        let bytes = digest;
        // Truncate
        let offset = bytes[19] & 0xF;
        let v = ((bytes[offset as usize] & 0x7F) as u32) << 24
            | ((bytes[offset as usize + 1] & 0xFF) as u32) << 16
            | ((bytes[offset as usize + 2] & 0xFF) as u32) << 8
            | (bytes[offset as usize + 3] & 0xFF) as u32;

        format!("{:06}", v % 1_000_000)
    }

    fn compute_hmac_sha1(&self, message: String) -> Vec<u8> {
        let key = BASE32_NOPAD.decode(&self.secret.clone().into_bytes()).expect("Failed to decode Base32 key");
        let mut mac = Hmac::<Sha1>::new_from_slice(&key).expect("HMAC can take key of any size");

        mac.update(&HEXLOWER.decode(&message.into_bytes()).expect("Failed to decode Hex message"));

        mac.finalize().into_bytes().to_vec()
    }
}