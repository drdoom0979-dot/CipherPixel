use std::fs::File;
use std::io::{Read, Write, Result};
use std::path::Path;

pub struct FileHandler;

impl FileHandler {
    /// Lee cualquier archivo del disco y lo convierte en bytes puros.
    /// Esto funciona para .pdf, .rs, .cpp, .txt, etc.
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        
        // Leemos todo el contenido del archivo y lo volcamos al buffer
        file.read_to_end(&mut buffer)?;
        
        Ok(buffer)
    }

    /// Toma un vector de bytes (ya desencriptado) y lo vuelve a guardar como un archivo.
    pub fn save_file<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<()> {
        let mut file = File::create(path)?;
        
        // Escribimos los bytes directamente al disco
        file.write_all(data)?;
        
        Ok(())
    }
}