use image::{DynamicImage, GenericImageView};

pub struct Decoder;

impl Decoder {
    /// Extrae un mensaje oculto de una imagen
    pub fn extract(img: &DynamicImage) -> Vec<u8> {
        let mut bits = Vec::new();
        let mut message_bytes = Vec::new();

        // Recorremos los píxeles de la imagen
        for (_x, _y, pixel) in img.pixels() {
            // El pixel tiene canales R, G, B, A (0, 1, 2, 3)
            for channel in 0..3 { // Solo usamos R, G y B
                let val = pixel[channel];
                // Extraemos el bit menos significativo usando una máscara (val & 1)
                bits.push(val & 1);

                // Cuando acumulamos 8 bits, formamos un byte
                if bits.len() == 8 {
                    let byte = bits.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
                    
                    // Si encontramos el carácter nulo, terminamos
                    if byte == 0 {
                        return message_bytes;
                    }
                    
                    message_bytes.push(byte);
                    bits.clear();
                }
            }
        }
        message_bytes
    }
}