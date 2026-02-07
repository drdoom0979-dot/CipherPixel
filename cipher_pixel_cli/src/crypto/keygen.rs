use argon2::{
    password_hash::{SaltString},
    Argon2, Params, Version,
};
use rand::RngCore;
use rand::thread_rng;

pub struct KeyGenerator;

impl KeyGenerator {
    /// Transforma una contraseña y un salt en una clave de 32 bytes (AES-256)
    pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
        // Configuramos Argon2id con parámetros de seguridad recomendados
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            Version::V0x13,
            Params::default(), // Usa valores estándar para memoria y tiempo
        );

        let mut output_key = [0u8; 32];
        
        // Ejecutamos el hashing para obtener la clave final
        argon2.hash_password_into(
            password.as_bytes(),
            salt,
            &mut output_key
        ).expect("Error crítico al derivar la clave criptográfica");

        output_key
    }

    /// Genera un Salt aleatorio de 16 bytes para asegurar que la clave sea única
    pub fn generate_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];
        thread_rng().fill_bytes(&mut salt);
        salt
    }
}