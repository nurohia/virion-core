use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use rand_core::RngCore;

pub struct Cipher {
    key: Key,
}

impl Cipher {
    pub fn new_from_password(password: &str) -> Self {
        let mut key_bytes = [0u8; 32];
        let hash = blake3::hash(password.as_bytes());
        key_bytes.copy_from_slice(hash.as_bytes());
        Self {
            key: Key::from_slice(&key_bytes).clone(),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Option<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(&self.key);
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        cipher.encrypt(nonce, plaintext)
            .map(|mut ciphertext| {
                let mut out = Vec::from(nonce_bytes);
                out.append(&mut ciphertext);
                out
            }).ok()
    }

    pub fn decrypt(&self, data: &[u8]) -> Option<Vec<u8>> {
        if data.len() < 12 { return None; }
        let (nonce, ciphertext) = data.split_at(12);
        let cipher = ChaCha20Poly1305::new(&self.key);
        cipher.decrypt(Nonce::from_slice(nonce), ciphertext).ok()
    }
}
