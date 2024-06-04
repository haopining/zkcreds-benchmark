use std::collections::linked_list::Cursor;

use ark_ff::PrimeField;
use jf_merkle_tree::NodeValue;
use jf_relation::{BoolVar, Circuit, CircuitError, PlonkCircuit, Variable};
use jf_merkle_tree::gadgets::{DigestAlgorithmGadget, MerkleTreeGadget, MembershipProof, RescueDigestGadget, Merkle3AryMembershipProofVar};
use jf_merkle_tree::{MerkleTreeScheme, UniversalMerkleTreeScheme, prelude::RescueMerkleTree, ToTraversalPath};
use jf_rescue::{RescueParameter, gadgets::RescueGadget};


pub trait MerkleForestGadget<M>: MerkleTreeGadget<M>
where
    M: MerkleTreeScheme,
    M::NodeValue: PrimeField,
{
    type ForestMembershipProofVar;

    fn create_forest_membership_proof_variable(
        &mut self,
        forest_roots: Vec<M::NodeValue>,
        elem_root: M::NodeValue,
    ) -> Result<Self::ForestMembershipProofVar, CircuitError>;

    fn is_forest_member(
        &mut self,
        forest_proof_var: Self::ForestMembershipProofVar,
        elem_root_var: Variable,
    ) -> Result<BoolVar, CircuitError>;

    fn enforce_forest_membership_proof(
        &mut self,
        forest_proof_var: Self::ForestMembershipProofVar,
        elem_root_var: Variable,
    ) -> Result<(), CircuitError>;
    
}

pub struct ForestMembershipProofVar {
    roots_vars: Vec<Variable>,
    root_var:Variable,
}

impl<T> MerkleForestGadget<T> for PlonkCircuit<T::NodeValue>
where
    T: MerkleTreeScheme,
    T::MembershipProof: MembershipProof<T::NodeValue, T::Index, T::NodeValue>,
    T::NodeValue: PrimeField + RescueParameter,
    T::Index: ToTraversalPath<3>,

{
    type ForestMembershipProofVar = ForestMembershipProofVar;


    fn create_forest_membership_proof_variable(
        &mut self,
        forest_roots: Vec<T::NodeValue>,
        root_var: T::NodeValue,
    ) -> Result<Self::ForestMembershipProofVar, CircuitError> {
        let forest_roots: Vec<Variable> = forest_roots
            .into_iter()
            .map(|root| self.create_variable(root))
            .collect::<Result<_, _>>()?;
        let root_var = self.create_variable(root_var)?;
        Ok(ForestMembershipProofVar {
            roots_vars: forest_roots,
            root_var,
        })
    }
    fn is_forest_member(
        &mut self,
        forest_proof_var: Self::ForestMembershipProofVar,
        elem_root_var: Variable,
    ) -> Result<BoolVar, CircuitError> {
        let mut result = self.create_boolean_variable(false);
        // Check whether elem_root_var is in forest_proof_var.roots_vars
        for cur_root in forest_proof_var.roots_vars.iter() {
            result = self.is_equal(elem_root_var, *cur_root);
        };
        Ok(result.unwrap())
    }


    fn enforce_forest_membership_proof(
        &mut self,
        forest_proof_var: Self::ForestMembershipProofVar,
        elem_root_var: Variable,
    ) -> Result<(), CircuitError> {
        // create table vairables which are the roots of the forest
        let bool_val = MerkleForestGadget::<T>::is_forest_member(self, forest_proof_var, elem_root_var)?;
        self.enforce_true(bool_val.into())
    }

}