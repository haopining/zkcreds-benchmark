use std::marker::PhantomData;
use ark_std::collections::BTreeMap;
use ark_ff::PrimeField;
use jf_merkle_tree::prelude::{MerkleTreeScheme, MerkleNode, NodeValue, RescueMerkleTree, MerkleCommitment, UniversalMerkleTreeScheme, RescueSparseMerkleTree};
use jf_merkle_tree::gadgets::{DigestAlgorithmGadget, MerkleTreeGadget, UniversalMerkleTreeGadget};
use jf_relation::{Circuit, CircuitError, PlonkCircuit};
use jf_rescue::RescueParameter;
// use jf_plonk::proof_system::PlonkKzgSnark;
use num_bigint::{BigUint, ToBigUint};

/// Represents the roots of the Merkle trees in the forest
pub struct MerkleForestRoots<T>
{
    // Roots of the Merkle trees in the forest
    pub trees_roots: Vec<T>,
}

pub struct MerkleForest<F> 
where
    F: RescueParameter,
{
    pub trees: Vec<RescueSparseMerkleTree<BigUint, F>>,
    _marker: PhantomData<F>,
}

// impl<F> MerkleForest<F>
// where
//     F: RescueParameter
// {
//     pub fn roots<T>(&self) -> MerkleForestRoots<T> {
//         let mut trees_roots = Vec::new();
//         for tree in &self.trees {
//             trees_roots.push(tree.commitment().digest());
//         }
//         MerkleForestRoots {
//             trees_roots,
//         }
//     }
// }

// implement Clone for MerkleForestRoots<T: NodeValue>
impl<T: NodeValue> Clone for MerkleForestRoots<T> {
    fn clone(&self) -> Self {
        Self {
            trees_roots: self.trees_roots.clone(),
        }
    }
}



impl<T> MerkleForestRoots<T> {
    /// Create a new instance of MerkleForestRoots
    pub fn new(num_trees: usize) -> Self {
        Self {
            trees_roots: Vec::with_capacity(num_trees),
        }
    }

}




// implement MerkleForestGadget
// impl <T> MerkleForestGadget<T> for PlonkCircuit<T::NodeValue> {

// }

#[cfg(test)]
mod merkle_forest_tests {

}

