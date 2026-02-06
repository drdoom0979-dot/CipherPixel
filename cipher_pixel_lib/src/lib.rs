// cipher_pixel_lib/src/lib.rs

// 1. Declaración de los módulos internos
pub mod math;
pub mod stego;
pub mod utils;

// 2. Re-exportaciones para un uso más cómodo (Opcional pero recomendado)
// Esto permite usar 'cipher_pixel_lib::Monobit' en lugar de rutas largas.
pub use crate::math::monobit::Monobit;
pub use crate::math::poker::PokerTest;
pub use crate::math::serial::SerialTest;
pub use crate::math::StatisticalTest;

pub use crate::stego::encoder::Encoder;
pub use crate::stego::decoder::Decoder;

pub use crate::utils::bit_tools::BitTools;

/// Función de utilidad para correr todos los tests de una vez
pub fn run_security_audit(bits: &[u8]) {
    let tests: Vec<(String, Box<dyn StatisticalTest>)> = vec![
        ("Monobit Test".to_string(), Box::new(Monobit)),
        ("Serial Test".to_string(), Box::new(SerialTest)),
        ("Poker Test (m=3)".to_string(), Box::new(PokerTest { m: 3 })),
    ];

    println!("\n--- Security Audit (NIST Standards) ---");
    for (name, test) in tests {
        let p_value = test.verify(bits);
        let status = if p_value >= 0.01 { "PASS" } else { "FAIL" };
        println!("{}: p-value = {:.4} [{}]", name, p_value, status);
    }
    println!("---------------------------------------\n");
}