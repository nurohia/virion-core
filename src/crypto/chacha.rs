
use chacha20::cipher::{KeyIvInit, StreamCipher};
use rand::Rng;

pub struct Cipher {
    key: [u8; 32],
    iv: [u8; 12],
}

impl Cipher {
    pub fn new_from_password(password: &str) -> Self {
        let mut key = [0u8; 32];
        let mut iv = [0u8; 12];
        let mut rng = rand::thread_rng();
        rng.fill(&mut key);
        rng.fill(&mut iv);
        Self { key, iv }
    }

    pub fn encrypt(&self, data: &[u8]) -> Option<Vec<u8>> {
        let mut buffer = data.to_vec();
        let mut cipher = chacha20::ChaCha20::new(&self.key.into(), &self.iv.into());
        cipher.apply_keystream(&mut buffer);
        Some(buffer)
    }

    pub fn decrypt(&self, data: &[u8]) -> Option<Vec<u8>> {
        self.encrypt(data) // ChaCha20 is symmetric
    }
}
