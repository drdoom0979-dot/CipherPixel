mod ui;
mod crypto;
mod files;
mod commands;

use clap::{Parser, Subcommand};
use ui::Interface;

/// CipherPixel CLI - Moon Dynamics
#[derive(Parser)]
#[command(name = "cipher_pixel")]
#[command(about = "Oculta y recupera archivos en imágenes con cifrado AES-256-GCM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Oculta un archivo dentro de una imagen (Cifrado + Esteganografía)
    Hide {
        /// Ruta de la imagen original (portadora)
        #[arg(short, long)]
        image: String,

        /// Ruta del archivo que quieres ocultar (PDF, Script, etc)
        #[arg(short, long)]
        file: String,

        /// Contraseña para cifrar los datos
        #[arg(short, long)]
        pass: String,

        /// Ruta de salida para la nueva imagen
        #[arg(short, long, default_value = "output.png")]
        output: String,
    },
    
    /// Extrae y descifra un archivo de una imagen portadora
    Extract {
        /// Ruta de la imagen que contiene el archivo oculto
        #[arg(short, long)]
        image: String,

        /// Contraseña para descifrar los datos
        #[arg(short, long)]
        pass: String,

        /// Ruta y nombre del archivo a recuperar (ej: secreto_recuperado.pdf)
        #[arg(short, long)]
        output: String,
    },
}

fn main() {
    // El banner siempre se muestra al inicio para dar identidad a la herramienta
    Interface::welcome_banner();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Hide { image, file, pass, output } => {
            // Lógica para ocultar
            commands::hide::exec_hide(image, file, pass, output);
        }
        Commands::Extract { image, pass, output } => {
            // Lógica para extraer (La nueva ruta que conecta con save_file)
            commands::extract::exec_extract(image, pass, output);
        }
    }
}