use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand::RngCore;
use rand::thread_rng;

pub struct CryptoManager;

impl CryptoManager {
    /// Genera una clave de 32 bytes a partir de una contraseña usando Argon2
    fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
        let mut config = argon2::Config::default();
        config.variant = argon2::Variant::Argon2id; // El más seguro contra ataques de GPU
        
        let hash = argon2::hash_raw(password.as_bytes(), salt, &config)
            .expect("Fallo al derivar la clave");
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash[..32]);
        key
    }

    /// Encripta cualquier archivo (PDF, Script, Texto)
    pub fn encrypt(data: &[u8], password: &str) -> Vec<u8> {
        let mut salt = [0u8; 16];
        thread_rng().fill_bytes(&mut salt); // Sal aleatoria para que la clave sea única
        
        let key_bytes = Self::derive_key(password, &salt);
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes); // Número aleatorio único por cada cifrado
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .expect("Error durante la encriptación");

        // Unimos [Salt + Nonce + Ciphertext] para que el Decoder tenga todo lo necesario
        let mut final_payload = Vec::new();
        final_payload.extend_from_slice(&salt);
        final_payload.extend_from_slice(&nonce_bytes);
        final_payload.extend_from_slice(&ciphertext);
        
        final_payload
    }
}