use image::{DynamicImage, GenericImageView};

pub struct Encoder;

impl Encoder {
    /// Oculta una secuencia de bits en una imagen usando LSB
    pub fn hide(img: &mut DynamicImage, message_bits: &[u8]) -> Result<(), String> {
        let (width, height) = img.dimensions();
        let total_pixels = width * height;
        
        // Verificamos si el mensaje cabe en la imagen (3 canales por píxel: RGB)
        if message_bits.len() > (total_pixels * 3) as usize {
            return Err("El mensaje es demasiado grande para esta imagen".into());
        }

        let mut bit_iter = message_bits.iter();
        let mut img_buffer = img.to_rgba8(); // Trabajamos sobre el buffer de píxeles

        'outer: for y in 0..height {
            for x in 0..width {
                let mut pixel = img_buffer.get_pixel(x, y).clone();
                
                // Modificamos canales R, G y B (0, 1, 2)
                for i in 0..3 {
                    if let Some(&bit) = bit_iter.next() {
                        // Lógica LSB: 
                        // Limpiamos el último bit con '& 0xFE' (11111110) 
                        // y luego aplicamos un 'OR' con el bit del mensaje
                        pixel[i] = (pixel[i] & 0xFE) | bit;
                    } else {
                        // Si no hay más bits, guardamos el último píxel y salimos
                        img_buffer.put_pixel(x, y, pixel);
                        break 'outer;
                    }
                }
                img_buffer.put_pixel(x, y, pixel);
            }
        }

        // Actualizamos la imagen original con el buffer modificado
        *img = DynamicImage::ImageRgba8(img_buffer);
        Ok(())
    }
}