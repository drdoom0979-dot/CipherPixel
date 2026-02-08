use crate::crypto::CryptoManager;
use crate::files::FileHandler;
use crate::ui::Interface;
use cipher_pixel_lib::stego::Decoder; // Necesitas tener el Decoder en tu lib
use image;

pub fn exec_extract(image_path: &str, password: &str, output_path: &str) {
    // 1. CARGA DE IMAGEN
    Interface::info(&format!("Abriendo imagen portadora: {}...", image_path));
    let img = image::open(image_path).expect("No se pudo abrir la imagen");

    // 2. EXTRACCIÓN LSB (Librería)
    Interface::info("Extrayendo bits ocultos de los píxeles...");
    let encrypted_data = Decoder::extract(&img);

    // 3. DESCRIPCIÓN (Criptografía)
    Interface::info("Descifrando datos con AES-256-GCM...");
    match CryptoManager::decrypt(&encrypted_data, password) {
        Ok(raw_data) => {
            Interface::success("¡Descifrado exitoso! Integridad de datos verificada.");

            // 4. GUARDADO (Aquí es donde se usa save_file)
            Interface::info(&format!("Guardando archivo recuperado en: {}...", output_path));
            FileHandler::save_file(output_path, &raw_data)
                .expect("Error al escribir el archivo en disco");

            Interface::success("Proceso completado. El archivo secreto ha sido restaurado.");
        },
        Err(e) => {
            Interface::error(&format!("Error de seguridad: {}. ¿Es correcta la contraseña?", e));
        }
    }
}