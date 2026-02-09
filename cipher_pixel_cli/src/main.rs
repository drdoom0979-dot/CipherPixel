mod ui;
mod crypto;
mod files;
mod commands;

use clap::{Parser, Subcommand};
use ui::Interface;

#[derive(Parser)]
#[command(
    name = "CipherPixel",
    author = "Moon Dynamics",
    version = "1.0",
    about = "🔒 Esteganografía LSB + Cifrado AES-256-GCM",
    help_template = "{before-help}
{name} {version}
{author-with-newline}
{about-section}

MODO DE USO:
    {usage}

{all-args}

EJEMPLOS DE HELP:
    1. Ocultar un reporte en una foto:
       cipherpixel hide -h

    2. Recuperar el archivo:
       cipherpixel extract -h

Para más detalles de un comando usa:
    cipherpixel <COMMAND> --help
{after-help}"
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Oculta un archivo dentro de una imagen (Cifrado + Esteganografía)
    Hide {
        #[arg(short, long, value_name = "RUTA", help = "Imagen original donde se esconderá el secreto")]
        image: String,

        #[arg(short, long, value_name = "ARCHIVO", help = "Archivo que deseas ocultar (PDF, TXT, etc.)")]
        file: String,

        #[arg(short, long, value_name = "PASSWORD", help = "Clave de cifrado AES-256")]
        pass: String,

        #[arg(short, long, value_name = "Result",default_value = "output.png", help = "Nombre de la imagen resultante")]
        output: String,
    },
    
    /// Extrae y descifra un archivo de una imagen portadora
    Extract {
        #[arg(
            short, 
            long, 
            value_name = "IMAGEN", 
            help = "Ruta de la imagen que contiene el archivo oculto"
        )]
        image: String,

        #[arg(
            short, 
            long, 
            value_name = "PASSWORD", 
            help = "Contraseña para descifrar los datos (debe coincidir con la de ocultación)"
        )]
        pass: String,

        #[arg(
            short, 
            long, 
            value_name = "DESTINO", 
            help = "Nombre y extensión del archivo a recuperar (ej: secreto_recuperado.pdf)"
        )]
        output: String,
    },
}

fn main() {
    // El banner siempre se muestra al inicio para dar identidad a la herramienta

    let cli = Cli::parse();

    Interface::welcome_banner();

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