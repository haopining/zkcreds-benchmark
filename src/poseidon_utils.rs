use core::fmt;
use ark_ff::UniformRand;
use rand::Rng;


/// A commitment nonce is always just a 256 bit value
#[derive(Clone, Default, PartialEq, Eq)]
pub struct ComNonce(pub [u8; 32]);

impl fmt::Debug for ComNonce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("[omitted]")
    }
}

impl ComNonce {
    pub fn to_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl UniformRand for ComNonce {
    #[inline]
    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self {
        ComNonce(UniformRand::rand(rng))
    }
}