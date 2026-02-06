use crate::math::StatisticalTest;
use statrs::distribution::{ChiSquared, ContinuousCDF};

pub struct SerialTest;

impl StatisticalTest for SerialTest {
    fn verify(&self, bits: &[u8]) -> f64 {
        let n = bits.len();
        if n < 2 { return 0.0; }

        // 1. Inicializar contadores para los 4 pares posibles (00, 01, 10, 11)
        let mut counts = [0.0; 4];

        // 2. Contar pares de bits consecutivos (traslapados)
        // Usamos .windows(2) para obtener pares [b1, b2], [b2, b3], etc.
        for pair in bits.windows(2) {
            // Convertir el par [b1, b2] a un índice decimal 0-3
            let index = ((pair[0] << 1) | pair[1]) as usize;
            counts[index] += 1.0;
        }

        // 3. Calcular el estadístico Chi-cuadrado
        // La frecuencia esperada para cada par en una secuencia aleatoria es (n-1)/4
        let expected = (n - 1) as f64 / 4.0;
        let mut v_obs = 0.0;
        
        for &observed in &counts {
            v_obs += (observed - expected).powi(2) / expected;
        }

        // 4. Calcular el P-value
        // Grados de libertad para pares de bits = 3 (4 categorías - 1)
        let degrees_of_freedom = 3.0;
        let chi = ChiSquared::new(degrees_of_freedom).unwrap();
        
        1.0 - chi.cdf(v_obs)
    }
}