use crate::math::StatisticalTest;

pub struct ShannonEntropy;

impl StatisticalTest for ShannonEntropy {
    fn verify(&self, bits: &[u8]) -> f64 {
        // 1. Usamos chunks_exact para procesar bytes completos
        let chunks = bits.chunks_exact(8);
        let total_chunks = chunks.len(); // Número real de bytes procesados

        if total_chunks == 0 { return 0.0; }

        let mut histogram = [0usize; 256];

        for chunk in chunks {
            // Reconstrucción del byte (0-255)
            let byte_val = chunk.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
            histogram[byte_val as usize] += 1;
        }

        // 2. Cálculo de Entropía
        let mut entropy = 0.0;
        let n_f64 = total_chunks as f64;

        for &count in histogram.iter() {
            if count > 0 {
                let p = count as f64 / n_f64;
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::BitTools;

    #[test]
    fn test_shannon_logic() {
        let shannon = ShannonEntropy;

        println!("\n=======================================================");
        println!("       TEST DE VALIDACIÓN: ENTROPÍA DE SHANNON         ");
        println!("=======================================================");

        // --- CASO 1: ORDEN TOTAL ---
        println!(" [🧪] Prueba 1: Entropía Mínima (Datos constantes)");
        // 4 bytes de ceros = 32 bits
        let zero_bits = vec![0u8; 32]; 
        let entropy_low = shannon.verify(&zero_bits);
        
        println!("      └─ Descripción: 4 bytes idénticos (00000000)");
        println!("      └─ Resultado: {:.4} bits/byte", entropy_low);
        assert_eq!(entropy_low, 0.0);
        println!(" [✅] Pasa: El sistema detecta orden absoluto.\n");

        // --- CASO 2: CAOS TOTAL (DISTRIBUCIÓN UNIFORME) ---
        println!(" [🧪] Prueba 2: Entropía Máxima (0 al 255)");
        let mut max_entropy_bits = Vec::new();
        for i in 0..=255 {
            for shift in (0..8).rev() {
                max_entropy_bits.push((i >> shift) & 1);
            }
        }
        
        let entropy_high = shannon.verify(&max_entropy_bits);
        println!("      └─ Descripción: Cada byte (0-255) aparece una vez.");
        println!("      └─ Resultado: {:.4} bits/byte", entropy_high);
        
        assert!((entropy_high - 8.0).abs() < 1e-10);
        println!(" [✅] Pasa: El sistema detecta aleatoriedad perfecta.");
        
        // --- CASO 3: INSPECCIÓN DE BYTES ---
        println!("\n [🧪] Prueba 3: Debug de Reconstrucción de Bytes");
        // Vamos a probar con el caracter 'A' (binario: 01000001)
        let a_bits = vec![0, 1, 0, 0, 0, 0, 0, 1];
        // En tu implementación de Shannon, esto debería contarse como 1 byte
        let entropy_a = shannon.verify(&a_bits);
        println!("      └─ Analizando bits del caracter 'A': {:?}", a_bits);
        println!("      └─ Entropía de un solo símbolo: {:.4}", entropy_a);

        // --- CASO 4: TEXTO REPETITIVO (Baja Entropía) ---
        // "AAAA" tiene entropía 0. Pero "ABAB" tiene entropía 1.0 (50% de probabilidad cada uno)
        let abab_bits = BitTools::string_to_bits_raw("ABABABAB"); 
        let entropy_abab = shannon.verify(&abab_bits);
        println!(" [🧪] Prueba 4: Texto Repetitivo ('ABABABAB')");
        println!("      └─ Resultado: {:.4} bits/byte", entropy_abab);
        assert!((entropy_abab - 1.0).abs() < 1e-10);
        println!("      └─ Nota: Detecta que solo hay 2 estados posibles.\n");

        // --- CASO 5: TEXTO EN INGLÉS/ESPAÑOL (Entropía Media) ---
        // El lenguaje humano es redundante. Suele rondar los 3.5 - 4.5 bits/byte.
        let msg = "Moon Dynamics: Grado Militar";
        let msg_bits = BitTools::string_to_bits_raw(msg);
        let entropy_msg = shannon.verify(&msg_bits);
        println!(" [🧪] Prueba 5: Mensaje de texto real");
        println!("      └─ Mensaje: '{}'", msg);
        println!("      └─ Resultado: {:.4} bits/byte", entropy_msg);
        assert!(entropy_msg > 3.0 && entropy_msg < 5.0);
        println!("      └─ Nota: Típico de archivos no cifrados.\n");

        // --- CASO 6: DATOS PSEUDO-ALEATORIOS (Simulación de Cifrado) ---
        // Simulamos una secuencia que parece ruido (alta entropía)
        let mut pseudo_rand = Vec::new();
        for i in 0..1000 {
            // Usamos una operación matemática simple para generar "caos"
            let byte = ((i * 167) % 256) as u8;
            for shift in (0..8).rev() {
                pseudo_rand.push((byte >> shift) & 1);
            }
        }
        let entropy_rand = shannon.verify(&pseudo_rand);
        println!(" [🧪] Prueba 6: Simulación de datos cifrados");
        println!("      └─ Resultado: {:.4} bits/byte", entropy_rand);
        assert!(entropy_rand > 7.5);
        println!("      └─ Nota: Este es el objetivo para un 'PASS' de seguridad.");
        
        println!("=======================================================\n");
    }
}