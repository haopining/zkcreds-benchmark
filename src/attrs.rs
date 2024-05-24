use jf_commitment::CommitmentScheme;
// use ark_ff::{PrimeField, ToConstraintField};
// use core::marker::PhantomData;
// use std::ptr::NonNull;
// use rand::SeedableRng;
// use rand_chacha::ChaCha12Rng;
// use crate::poseidon_utils::ComNonce;

pub trait Attrs<C: CommitmentScheme>: Default {
    /// Convert the attributes to bytes.
    fn to_bytes(&self) -> Vec<u8>;

    /// Get the commitment nonce (randomness used in the commitment).
    fn get_comm_nonce(&self) -> Option<&C::Randomness>;

    /// Generate a commitment using the attributes and optional randomness.
    fn commit(&self) -> Result<C::Output, C::Error>;
}
