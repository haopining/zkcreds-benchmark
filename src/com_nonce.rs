use core::fmt;
// use ark_ff::UniformRand;
use rand::Rng;
use ark_std::UniformRand;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

/// A commitment nonce is always just a 256 bit value
#[derive(Clone, Default, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct ComNonce(pub [u8; 32]);

impl fmt::Debug for ComNonce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("[omitted]")
    }
}

impl UniformRand for ComNonce {
    fn rand<R: Rng + ?Sized>(rng: &mut R) -> Self {
        ComNonce(UniformRand::rand(rng))
    }
}
