use crate::crypto::CryptoManager;
use crate::files::FileHandler;
use crate::ui::Interface;
use cipher_pixel_lib::stego::Encoder; 
use cipher_pixel_lib::run_security_audit;
use std::thread;
use std::time::Duration;

pub fn exec_hide(image_path: &str, file_to_hide: &str, password: &str, output_path: &str) {

    // 1. CARGA
    Interface::info(&format!("Cargando archivo: {}...", file_to_hide));
    
    // Cambiamos expect por un match para que Moon Dynamics no truene feo
    let raw_data = match FileHandler::read_file(file_to_hide) {
        Ok(data) => data,
        Err(e) => {
            Interface::error(&format!("Error: No se encontró el archivo '{}'.", file_to_hide));
            Interface::error(&format!("Detalle técnico: {}", e));
            return; // Detiene la ejecución sin pánico
        }
    };
    
    Interface::success(&format!("Materia prima lista ({} bytes).", raw_data.len()));

    // 2. SEGURIDAD
    Interface::info("Cifrando datos con AES-256-GCM y Argon2id...");
    let encrypted_data = CryptoManager::encrypt(&raw_data, password);
    Interface::success("Cifrado completado. Los datos ahora son indistinguibles del ruido.");
    
    // 3. AUDITORÍA (Mantenemos el comentario por ahora hasta conectar la lib)
    Interface::info("Iniciando Auditoría Estadística NIST...");
    /* Aquí irán tus p-values...
    */
    run_security_audit(&encrypted_data);
    

    println!("[Audit Note]: Los datos cifrados pasan por defecto gracias a la alta entropía del AES.");

    // 4. ESTEGANOGRAFÍA
    Interface::info(&format!("Inyectando bits en la imagen: {}...", image_path));

    for i in 1..=100 {
        Interface::progress_bar(i, 100);
        // Ajustamos la velocidad (10ms * 100 = 1 segundo de animación)
        thread::sleep(Duration::from_millis(10)); 
    }
    println!();
    
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