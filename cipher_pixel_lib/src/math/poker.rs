use crate::math::StatisticalTest;
use statrs::distribution::{ChiSquared, ContinuousCDF};

pub struct PokerTest {
    pub m: usize, // Tamaño del bloque (ej. 3)
}



impl StatisticalTest for PokerTest {
    fn verify(&self, bits: &[u8]) -> f64 {
        let n = bits.len();
        
        // Evitamos tamaños de bloque absurdos (m > 20 es demasiado para RAM)
        if self.m == 0 || self.m > 20 || n < self.m {
            return f64::NAN;
        }

        let k = n / self.m;
        let num_patterns = 2_usize.pow(self.m as u32);
        
        // 1. Histogram
        let mut frequencies = vec![0.0; num_patterns];

        // 2. Conteo
        for chunk in bits.chunks_exact(self.m) {
            let index = chunk.iter().fold(0, |acc, &bit| (acc << 1) | bit as usize);
            frequencies[index] += 1.0;
        }

        // 3. Cálculo matemático
        let sum_sq_frequencies: f64 = frequencies.iter().map(|&f| f * f).sum();
        let v_obs = (num_patterns as f64 / k as f64) * sum_sq_frequencies - k as f64;

        // 4. Distribución con validación
        let df = (num_patterns - 1) as f64;
        
        // En lugar de un try/except, usamos match o if let para crear la distribución
        match ChiSquared::new(df) {
            Ok(chi) => 1.0 - chi.cdf(v_obs),
            Err(_) => f64::NAN, // Si falla la creación (ej. df <= 0), devolvemos 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poker_random_pass() {
        let poker = PokerTest { m: 2 };
        // Secuencia equilibrada: 00, 01, 10, 11 (2 veces cada uno)
        let bits = vec![0,0, 0,1, 1,0, 1,1, 0,0, 0,1, 1,0, 1,1];
        let p_value = poker.verify(&bits);
        
        println!("P-Value (Random): {}", p_value);
        assert!(p_value > 0.01, "Debería pasar con una secuencia uniforme");
    }

    #[test]
    fn test_poker_non_random_fail() {
        let poker = PokerTest { m: 3 };
        // Secuencia altamente repetitiva (siempre el patrón '111' o 7 decimal)
        let bits = vec![1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1];
        let p_value = poker.verify(&bits);
        
        println!("P-Value (Repetitive): {}", p_value);
        assert!(p_value < 0.01, "Debería fallar con patrones idénticos");
    }

    #[test]
    fn test_poker_empty_data() {
        let poker = PokerTest { m: 3 };
        let bits = vec![];
        let p_value = poker.verify(&bits);
        assert_eq!(p_value, 0.0, "Debe retornar 0.0 si no hay suficientes datos");
    }
}