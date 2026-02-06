pub struct BitTools;

impl BitTools {
    /// Convierte un String de texto en un vector de bits (0s y 1s)
    pub fn string_to_bits(message: &str) -> Vec<u8> {
        let mut bits = Vec::new();
        // Convertimos el string a bytes
        let bytes = message.as_bytes();

        for &byte in bytes {
            // Extraemos cada bit del byte, del más significativo al menos significativo
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }

        // Añadimos un "Null Terminator" (8 ceros) para que el Decoder sepa dónde parar
        for _ in 0..8 {
            bits.push(0);
        }

        bits
    }

    /// Convierte un vector de bits (0s y 1s) de vuelta a un String
    pub fn bits_to_string(bits: &[u8]) -> String {
        let mut bytes = Vec::new();
        
        // Procesamos los bits en grupos de 8
        for chunk in bits.chunks(8) {
            if chunk.len() < 8 { break; }
            
            let byte = chunk.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
            
            // Si llegamos al terminador nulo, dejamos de convertir
            if byte == 0 { break; }
            
            bytes.push(byte);
        }

        String::from_utf8_lossy(&bytes).into_owned()
    }
}