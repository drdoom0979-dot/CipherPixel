use crate::crypto::CryptoManager;
use crate::files::FileHandler;
use crate::ui::Interface;
use cipher_pixel_lib::stego::Decoder;
use image;

pub fn exec_extract(image_path: &str, password: &str, output_path: &str) {
    // 1. CARGA DE IMAGEN
    Interface::info(&format!("Abriendo imagen portadora: {}...", image_path));
    
    // Abrimos la imagen de forma segura
    let img = match image::open(image_path) {
        Ok(image) => image,
        Err(e) => {
            Interface::error(&format!("No se pudo abrir la imagen: {}", e));
            return;
        }
    };

    // 2. EXTRACCIÓN LSB (Librería)
    // Ahora Decoder::extract es inteligente y sabe detenerse gracias a la cabecera de 4 bytes
    Interface::info("Extrayendo bits ocultos de los píxeles...");
    let encrypted_data = Decoder::extract(&img);

    // 3. DESCRIPCIÓN (Criptografía)
    Interface::info("Descifrando datos con AES-256-GCM y derivación Argon2id...");
    
    match CryptoManager::decrypt(&encrypted_data, password) {
        Ok(raw_data) => {
            Interface::success("¡Descifrado exitoso! El Authentication Tag de AES es válido.");

            // 4. GUARDADO
            Interface::info(&format!("Restaurando archivo en: {}...", output_path));
            
            if let Err(e) = FileHandler::save_file(output_path, &raw_data) {
                Interface::error(&format!("Error al escribir el archivo: {}", e));
            } else {
                Interface::success("Proceso completado. El archivo ha sido recuperado íntegramente.");
            }
        },
        Err(e) => {
            // Este error saltará si la contraseña es incorrecta o si los datos están corruptos
            Interface::error(&format!("Error de seguridad: {}.", e));
        }
    }
}