use crate::math::StatisticalTest;
use statrs::distribution::{ChiSquared, ContinuousCDF};

pub struct PokerTest {
    pub m: usize, // Tamaño del bloque (ej. 3)
}

impl StatisticalTest for PokerTest {
    fn verify(&self, bits: &[u8]) -> f64 {
        let n = bits.len();
        let k = n / self.m; // Número de bloques completos
        
        if k == 0 { return 0.0; }

        // 1. Crear un vector de frecuencias para los 2^m patrones posibles
        let num_patterns = 2_usize.pow(self.m as u32);
        let mut frequencies = vec![0.0; num_patterns];

        // 2. Procesar la secuencia en bloques de tamaño m
        for chunk in bits.chunks_exact(self.m) {
            // Convertir el bloque de bits (ej. [1, 0, 1]) en un índice decimal (ej. 5)
            let index = chunk.iter().fold(0, |acc, &bit| (acc << 1) | bit as usize);
            frequencies[index] += 1.0;
        }

        // 3. Aplicar la fórmula del estadístico Chi-cuadrado (X^2)
        let sum_sq_frequencies: f64 = frequencies.iter().map(|&f| f * f).sum();
        
        let v_obs = (num_patterns as f64 / k as f64) * sum_sq_frequencies - k as f64;

        // 4. Calcular el P-value usando la distribución Chi-cuadrado
        // Grados de libertad = 2^m - 1
        let degrees_of_freedom = (num_patterns - 1) as f64;
        let chi = ChiSquared::new(degrees_of_freedom).unwrap();
        
        // El P-value es 1 - CDF (probabilidad acumulada)
        1.0 - chi.cdf(v_obs)
    }
}