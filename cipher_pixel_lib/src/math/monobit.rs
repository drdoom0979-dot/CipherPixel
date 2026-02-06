use crate::math::StatisticalTest; // Importamos el Rasgo (Trait)
use statrs::function::erf::erfc;   // Importamos la función de error complementaria

pub struct Monobit;

impl StatisticalTest for Monobit {
    fn verify(&self, bits: &[u8]) -> f64 {
        // 1. Obtener n (longitud de la secuencia)
        let n = bits.len() as f64;
        if n == 0.0 { return 0.0; }

        // 2. Sumatoria Sn: Transformar 0 -> -1 y 1 -> 1
        let s_n: f64 = bits.iter() // .iter() recorre cada bit
            .map(|&bit| if bit == 1 { 1.0 } else { -1.0 }) // .map() transforma el valor
            .sum(); // .sum() acumula el resultado

        // 3. Calcular Sobs (Estadístico observado)
        // .abs() asegura que el valor sea positivo (valor absoluto)
        // .sqrt() calcula la raíz cuadrada de n
        let s_obs = s_n.abs() / n.sqrt();

        // 4. Calcular P-value usando erfc(Sobs / sqrt(2))
        let p_value = erfc(s_obs / 2.0f64.sqrt());

        p_value
    }
}