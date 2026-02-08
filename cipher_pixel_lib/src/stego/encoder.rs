use image::{DynamicImage, GenericImageView, open};

pub struct Encoder;

impl Encoder {
    /// Función principal para abrir, ocultar y guardar la imagen
    pub fn encode(image_path: &str, data: &[u8], output_path: &str) -> Result<(), String> {
        // 1. Cargamos la imagen original (portadora)
        let mut img = open(image_path)
            .map_err(|e| format!("Error al abrir imagen: {}", e))?;

        // 2. Ejecutamos la ocultación LSB
        Self::hide(&mut img, data)?;

        // 3. Guardamos el resultado en formato PNG (para no perder datos por compresión)
        img.save(output_path)
            .map_err(|e| format!("Error al guardar imagen: {}", e))?;

        Ok(())
    }

    /// Lógica central de esteganografía LSB
    pub fn hide(img: &mut DynamicImage, encrypted_data: &[u8]) -> Result<(), String> {
        let (width, height) = img.dimensions();
        
        // --- PREPARACIÓN DE METADATOS ---
        // Obtenemos la longitud de los datos cifrados (4 bytes)
        let data_len = encrypted_data.len() as u32;
        let len_bytes = data_len.to_be_bytes(); // Big Endian: estándar de red/archivos

        // Creamos el payload final: [Longitud (4B)] + [Datos Cifrados (nB)]
        let mut payload = Vec::with_capacity(4 + encrypted_data.len());
        payload.extend_from_slice(&len_bytes);
        payload.extend_from_slice(encrypted_data);

        // Verificamos si la imagen tiene espacio suficiente (8 bits por byte)
        if payload.len() * 8 > (width * height * 3) as usize {
            return Err("La imagen es demasiado pequeña para el secreto".into());
        }

        let mut img_buffer = img.to_rgba8();
        let mut byte_idx = 0; // Byte actual que estamos procesando
        let mut bit_idx = 0;  // Bit actual (0 a 7) dentro de ese byte

        // --- PROCESO DE INYECCIÓN BIT A BIT ---
        'outer: for y in 0..height {
            for x in 0..width {
                let mut pixel = *img_buffer.get_pixel(x, y);
                
                // Recorremos canales R, G y B
                for i in 0..3 { 
                    if byte_idx < payload.len() {
                        let current_byte = payload[byte_idx];
                        
                        // Extraemos el bit específico usando desplazamiento (shifting)
                        // Movemos el bit deseado a la posición 0 y aplicamos máscara & 1
                        let bit = (current_byte >> (7 - bit_idx)) & 1;
                        
                        // LSB: Limpiamos el último bit del color (AND 11111110)
                        // e insertamos nuestro bit (OR 0 o 1)
                        pixel[i] = (pixel[i] & 0xFE) | bit;
                        
                        bit_idx += 1;
                        if bit_idx == 8 {
                            bit_idx = 0;
                            byte_idx += 1;
                        }
                    } else {
                        // Si terminamos de inyectar, guardamos y rompemos el bucle
                        img_buffer.put_pixel(x, y, pixel);
                        break 'outer;
                    }
                }
                img_buffer.put_pixel(x, y, pixel);
            }
        }

        // Actualizamos la imagen en memoria
        *img = image::DynamicImage::ImageRgba8(img_buffer);
        Ok(())
    }
}