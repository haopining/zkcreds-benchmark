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
where
    T: NodeValue,
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

// implement Clone for MerkleForestRoots<T: NodeValue>
impl<T: NodeValue> Clone for MerkleForestRoots<T> {
    fn clone(&self) -> Self {
        Self {
            trees_roots: self.trees_roots.clone(),
        }
    }
}

/////// Here to start
type SparseMerkleTree<F> = RescueSparseMerkleTree<BigUint, F>;


impl<F> MerkleForest<F>
where
    F: RescueParameter
{
    pub fn roots(&self) -> MerkleForestRoots<F> {
        let mut trees_roots = Vec::new();
        for tree in &self.trees {
            trees_roots.push(tree.commitment().digest());
        }
        MerkleForestRoots {
            trees_roots,
        }
    }
}

// pub trait MerkleForestGadget