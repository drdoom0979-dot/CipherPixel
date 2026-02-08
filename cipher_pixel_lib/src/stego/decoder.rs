use image::{DynamicImage, GenericImageView};

pub struct Decoder;

impl Decoder {
    /// Extrae los bits ocultos de la imagen y los reconstruye en bytes
    pub fn extract(img: &DynamicImage) -> Vec<u8> {
        let (width, height) = img.dimensions();
        let mut bits = Vec::new();
        let mut all_bytes = Vec::new();

        // Convertimos a buffer de 8 bits para mayor velocidad
        let img_buffer = img.to_rgba8();

        // Recorremos los píxeles (eliminamos 'outer: porque no lo usamos)
        for y in 0..height {
            for x in 0..width {
                let pixel = img_buffer.get_pixel(x, y);
                
                // Extraemos LSB de los canales R, G y B
                for channel in 0..3 { 
                    let val = pixel[channel];
                    bits.push(val & 1);

                    // Cada 8 bits, formamos un byte completo
                    if bits.len() == 8 {
                        let byte = bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
                        all_bytes.push(byte);
                        bits.clear();
                    }
                }
            }
        }
        all_bytes
    }
}