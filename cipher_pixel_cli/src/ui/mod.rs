use std::io::{self, Write};

pub struct Interface;

impl Interface {
    /// Muestra un banner de inicio con estilo
    pub fn welcome_banner() {
        println!("--------------------------------------------------");
        println!("          🌑 MOON DYNAMICS: CIPHER PIXEL          ");
        println!("     Esteganografía & Criptografía de Grado Militar ");
        println!("--------------------------------------------------");
    }

    /// Muestra un mensaje de éxito en verde (usando códigos ANSI)
    pub fn success(msg: &str) {
        println!("\x1b[32m SUCCESS:\x1b[0m {}", msg);
    }

    /// Muestra un mensaje de error en rojo
    pub fn error(msg: &str) {
        eprintln!("\x1b[31m ERROR:\x1b[0m {}", msg);
    }

    /// Muestra un proceso de auditoría o seguridad en azul
    pub fn info(msg: &str) {
        println!("\x1b[34m INFO:\x1b[0m {}", msg);
    }

    /// Simula una pequeña barra de progreso para la terminal
    pub fn progress_bar(current: usize, total: usize) {
        let progress = (current as f64 / total as f64) * 20.0;
        print!("\r[");
        for i in 0..20 {
            if (i as f64) < progress { print!("="); }
            else { print!(" "); }
        }
        print!("] {:.2}%", (current as f64 / total as f64) * 100.0);
        io::stdout().flush().unwrap();
    }
}