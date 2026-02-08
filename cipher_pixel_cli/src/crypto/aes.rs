use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    Argon2, Algorithm, Version, Params
};
use rand::{RngCore, thread_rng};

pub struct CryptoManager;

impl CryptoManager {
    /// Genera una clave de 32 bytes usando Argon2id (API moderna)
    fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
        let mut key = [0u8; 32];
        
        // Configuramos Argon2id con parámetros recomendados (FIME / NIST)
        // t_cost: 3 pasadas, m_cost: 64MB, p_cost: 4 hilos
        let params = Params::new(65536, 3, 4, Some(32))
            .expect("Parámetros de Argon2 inválidos");
            
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        
        argon2.hash_password_into(password.as_bytes(), salt, &mut key)
            .expect("Fallo al derivar la clave");
            
        key
    }

    pub fn encrypt(data: &[u8], password: &str) -> Vec<u8> {
        // 1. Salt aleatorio (16 bytes)
        let mut salt = [0u8; 16];
        thread_rng().fill_bytes(&mut salt);
        
        // 2. Derivación de clave
        let key_bytes = Self::derive_key(password, &salt);
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        // 3. Nonce aleatorio (12 bytes)
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // 4. Cifrado
        let ciphertext = cipher
            .encrypt(nonce, data)
            .expect("Error durante la encriptación");

        // 5. Empaquetado final: [Salt(16) + Nonce(12) + Ciphertext(n)]
        let mut final_payload = Vec::with_capacity(salt.len() + nonce_bytes.len() + ciphertext.len());
        final_payload.extend_from_slice(&salt);
        final_payload.extend_from_slice(&nonce_bytes);
        final_payload.extend_from_slice(&ciphertext);
        
        final_payload
    }

    // Dentro de aes.rs, añade esto:
    pub fn decrypt(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, String> {
        if encrypted_data.len() < 28 { // 16 (Salt) + 12 (Nonce)
            return Err("Datos insuficientes para descifrar".to_string());
        }

        // 1. Extraer las piezas del paquete
        let salt = &encrypted_data[..16];
        let nonce_bytes = &encrypted_data[16..28];
        let ciphertext = &encrypted_data[28..];

        // 2. Re-derivar la clave usando el Salt que venía en la imagen
        let key_bytes = Self::derive_key(password, salt);
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce_bytes);

        // 3. Descifrar
        cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Fallo al descifrar: ¿Es correcta la contraseña? {:?}", e))
    }
}