pub mod monobit;
pub mod poker;
pub mod serial;

pub trait StatisticalTest {
    fn verify(&self, bits: &[u8]) -> f64;
}
