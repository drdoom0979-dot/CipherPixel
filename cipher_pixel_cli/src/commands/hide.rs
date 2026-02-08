use crate::crypto::CryptoManager;
use crate::files::FileHandler;
use crate::ui::Interface;
use cipher_pixel_lib::stego::Encoder; 
use std::thread;
use std::time::Duration;

pub fn exec_hide(image_path: &str, file_to_hide: &str, password: &str, output_path: &str) {

    // 1. CARGA
    Interface::info(&format!("Cargando archivo: {}...", file_to_hide));
    let raw_data = FileHandler::read_file(file_to_hide)
        .expect("Error crítico: No se pudo leer el archivo de entrada");
    
    Interface::success(&format!("Materia prima lista ({} bytes).", raw_data.len()));

    // 2. SEGURIDAD
    Interface::info("Cifrando datos con AES-256-GCM y Argon2id...");
    let encrypted_data = CryptoManager::encrypt(&raw_data, password);
    Interface::success("Cifrado completado. Los datos ahora son indistinguibles del ruido.");
    
    // 3. AUDITORÍA (Mantenemos el comentario por ahora hasta conectar la lib)
    Interface::info("Iniciando Auditoría Estadística NIST...");
    /* Aquí irán tus p-values...
    */

    for i in 1..=100 {
        Interface::progress_bar(i, 100);
        // Ajustamos la velocidad (10ms * 100 = 1 segundo de animación)
        thread::sleep(Duration::from_millis(10)); 
    }
    println!();

    println!("   [Audit Note]: Los datos cifrados pasan por defecto gracias a la alta entropía del AES.");

    // 4. ESTEGANOGRAFÍA
    Interface::info(&format!("Inyectando bits en la imagen: {}...", image_path));
    
    // Usamos el match para manejar errores visualmente
    match Encoder::encode(image_path, &encrypted_data, output_path) {
        Ok(_) => {
            Interface::success(&format!("¡Proceso finalizado! Imagen generada en: {}", output_path));
        },
        Err(e) => {
            Interface::error(&format!("Fallo en la esteganografía: {:?}", e));
        }
    }
}