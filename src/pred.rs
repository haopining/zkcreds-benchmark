use jf_commitment::CommitmentScheme;
use jf_relation::{
    PlonkCircuit, Circuit, Variable, CircuitError, 
    gadgets::ecc::SWToTEConParam
};
use jf_plonk::proof_system::structs::Proof;
use jf_rescue::{gadgets::commitment::CommitmentGadget, RescueParameter};
use crate::attrs::{Attrs, AttrsVar};
use ark_ff::ToConstraintField;
use ark_std::borrow::Borrow;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use jf_merkle_tree::MerkleTreeScheme;
use core::marker::PhantomData;
use ark_ec::{
    pairing::Pairing,
    short_weierstrass::{Affine, SWCurveConfig},
};

// pub trait PredicateChecker   
// {
//     /// Enforces constraints on the given attributes
//     fn pred(self) -> Result<(), CircuitError>;
    
//     /// This outputs the field elements corresponding to the public inputs of this predicate. This
//     /// DOES NOT include `attrs`.
//     fn public_input(&self) -> Vec<Variable>;
// }

// // Represents a predicate proof
// pub struct PredProof<E, F, P, A, AV, C>
// where
//     E: Pairing<BaseField = F, G1Affine = Affine<P>>,
//     F: RescueParameter + SWToTEConParam,
//     P: SWCurveConfig<BaseField = F>,   
//     A: Attrs<F, C>,
//     AV: AttrsVar<A, C, F>,
//     C: CommitmentScheme,

  
// {
//     pub(crate) proof: Proof<E>,
//     pub(crate) _marker: PhantomData<(E, F, P, A, AV, C)>,
// }

// impl<E, F, P, A, AV, C> Clone for PredProof<E, F, P, A, AV, C>
// where
//     E: Pairing<BaseField = F, G1Affine = Affine<P>>,
//     F: RescueParameter + SWToTEConParam,
//     P: SWCurveConfig<BaseField = F>,   
//     A: Attrs<F, C>,
//     AV: AttrsVar<A, C, F>,
//     C: CommitmentScheme,
// {
//     fn clone(&self) -> Self {
//         Self {
//             proof: self.proof.clone(),
//             _marker: PhantomData,
//         }
//     }

// }

