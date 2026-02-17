use inquire::Select;
use crate::ui::Interface;


pub fn run_interactive_mode() {
    let opciones = vec!["Hide (Ocultar)", "Extract (Extraer)", "Salir"];
    let seleccion = Select::new("--- 🔒 CipherPixel Interactive ---", opciones).prompt();

    match seleccion {
        Ok("Hide (Ocultar)") => {
            let img = Interface::ask_text("> Ruta de la imagen portadora:", "original.png");
            let file = Interface::ask_text("> Archivo a ocultar:", "secreto.pdf");
            let pass = Interface::ask_password("> Password de cifrado:");
            let out = Interface::ask_text("> Nombre del archivo resultante:", "output.png");

            Interface::info("Iniciando proceso de esteganografía...");
            // Usamos 'crate::commands' o 'super' para llegar a hide
            crate::commands::hide::exec_hide(&img, &file, &pass, &out);
        }
        Ok("Extract (Extraer)") => {
            let img = Interface::ask_text("> Ruta de la imagen con secreto:", "output.png");
            let pass = Interface::ask_password("> Password de descifrado:");
            let out = Interface::ask_text("> Nombre del archivo recuperado:", "recuperado.pdf");

            Interface::info("Extrayendo bits de la imagen...");
            crate::commands::extract::exec_extract(&img, &pass, &out);
        }
        _ => println!("Operación cancelada."),
    }
}