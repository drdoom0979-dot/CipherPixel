use image::GenericImageView;

pub struct Decoder;

impl Decoder {
    /// Extrae los datos ocultos basándose en la longitud guardada en los primeros 4 bytes
    pub fn extract(img: &image::DynamicImage) -> Vec<u8> {
        let (width, height) = img.dimensions();
        let mut bits = Vec::new();
        let mut all_bytes = Vec::new();
        // Usamos el buffer RGBA8 para una extracción precisa de bits
        let img_buffer = img.to_rgba8();
        let mut expected_len: Option<usize> = None;

        for y in 0..height {
            for x in 0..width {
                let pixel = img_buffer.get_pixel(x, y);
                
                for channel in 0..3 { // Extraemos solo de R, G y B
                    let val = pixel[channel];
                    
                    // Capturamos solo el último bit (LSB)
                    bits.push(val & 1);

                    // Cada vez que juntamos 8 bits, formamos un byte completo
                    if bits.len() == 8 {
                        // Reconstruimos el byte desplazando bits a la izquierda
                        let byte = bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
                        all_bytes.push(byte);
                        bits.clear();

                        // --- LÓGICA DE DETENCIÓN CRÍTICA ---
                        
                        // 1. Al obtener los primeros 4 bytes, calculamos la longitud real
                        if expected_len.is_none() && all_bytes.len() == 4 {
                            let len_bytes = [all_bytes[0], all_bytes[1], all_bytes[2], all_bytes[3]];
                            // Convertimos bytes a número u32
                            expected_len = Some(u32::from_be_bytes(len_bytes) as usize);
                        }

                        // 2. Si ya sabemos la longitud y tenemos los bytes necesarios, cortamos
                        if let Some(len) = expected_len {
                            // len + 4 porque los primeros 4 bytes son los metadatos de longitud
                            if all_bytes.len() == len + 4 {
                                // Devolvemos solo los datos cifrados (omitimos la cabecera)
                                return all_bytes[4..].to_vec();
                            }
                        }
                    }
                }
            }
        }
        all_bytes // Fallback por si la imagen está corrupta o vacía
    }
}