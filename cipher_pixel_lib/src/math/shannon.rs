use crate::math::StatisticalTest;

pub struct ShannonEntropy;

impl StatisticalTest for ShannonEntropy {
    fn verify(&self, bits: &[u8]) -> f64 {
        if bits.len() < 8 { return 0.0; }

        let mut histogram = [0u32; 256];
        // Ahora el total de muestras son BYTES, no bits
        let total_bytes = (bits.len() / 8) as f64;

        // 1. Agrupamos de 8 en 8 para formar un byte real (0-255)
        for chunk in bits.chunks_exact(8) {
            let byte_val = chunk.iter().fold(0u8, |acc, &bit| (acc << 1) | bit);
            histogram[byte_val as usize] += 1;
        }

        // 2. Calculamos la entropía sobre el histograma de 256
        let mut entropy = 0.0;
        for &count in histogram.iter() {
            if count > 0 {
                let p = count as f64 / total_bytes;
                entropy -= p * p.log2();
            }
        }
        
        entropy // Ahora sí: 0.0 (predecible) a 8.0 (puro ruido)
    }
}