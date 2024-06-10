use jf_commitment::CommitmentScheme;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use ark_bls12_381::{Bls12_381, Fr};
use jf_relation::{Circuit, CircuitError, PlonkCircuit, Variable};
use jf_rescue::{
    commitment::FixedLengthRescueCommitment,
    gadgets::commitment::CommitmentGadget, RescueParameter
};
use ark_std::{borrow::Borrow, UniformRand};
use ark_ff::{PrimeField, ToConstraintField, to_bytes};
use rand_chacha::ChaChaRng;
use rand::{distributions::Slice, SeedableRng};

// use core::marker::PhantomData;
// use std::ptr::NonNull;
// use rand::SeedableRng;
use crate::com_nonce::ComNonce;


pub trait Attrs<C>: Default 
where
    C: CommitmentScheme,
    // C::Output: ToConstraintField<F>,
{
    // In my understanding, this data, excluding the nonce, is what is being committed to.
    fn to_commit_input(&self) -> Vec<Fr>;

    /// Get the commitment nonce (randomness used in the commitment).
    fn get_comm_nonce(&self) -> &ComNonce;

    /// Generate a commitment using the attributes and optional randomness.
    fn commit(&self) -> C::Output {
        let nonce = {
            let nonce_seed = self.get_comm_nonce();
            let mut rng = ChaChaRng::from_seed(nonce_seed.0);
            C::Randomness::rand(&mut rng)
        };
        let input: Vec<ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>> = self.to_commit_input();
       
        // Commit to the serialized attributes
        C::commit(input, Some(&nonce)).unwrap()
    }    
}



pub trait AttrsVar<A, C, F> : Sized
where
    A: Attrs<C>,
    C: CommitmentScheme,
    F: RescueParameter

{      
    fn cs(&self) -> PlonkCircuit<F>;

    fn get_comm_nonce(&self) -> &ComNonce;

    fn commit(&mut self, attrs: &A) -> Result<Variable, CircuitError> {
        let mut circuit = self.cs();
        let nonce_seed = self.get_comm_nonce();
        let mut rng = ChaChaRng::from_seed(nonce_seed.0);
        let nonce = F::rand(&mut rng);
        let nonce_var = circuit.create_variable(nonce).unwrap();

        let attrs_vars: Vec<Variable> = [attrs.to_commit_input()]
                .iter()
                .map(|&x| circuit.create_variable(x).unwrap())
                .collect();
        
        circuit.commit(&attrs_com_input, nonce_var)
    }
}


