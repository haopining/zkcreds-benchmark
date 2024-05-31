use ark_std::{borrow::Borrow, fmt::Debug, marker::PhantomData, string::ToString, vec, vec::Vec};

use jf_merkle_tree::{
    DigestAlgorithm, Element, Index, NodeValue, 
    internal::{MerkleNode, MerklePath, MerkleProof, MerkleTreeCommitment},
    };
use jf_merkle_tree::errors::MerkleTreeError;
use jf_merkle_tree::universal_merkle_tree::UniversalMerkleTree;

// define a new trait for UniversalMerkleTree with a membership_verify method
pub trait UniversalMerkleTreeMembershipVerify<E, H, I, const ARITY: usize, T>
where
    E: Element,
    H: DigestAlgorithm<E, I, T>,
    I: Index,
    T: NodeValue,
{
    fn membership_verify(
        &self,
        pos: impl Borrow<Self::Index>,
        proof: impl Borrow<Self::NonMembershipProof>,
    ) -> Result<bool, MerkleTreeError>;
}

impl<E, H, I, const ARITY: usize, T> UniversalMerkleTreeMembershipVerify<E, H, I, ARITY, T> for UniversalMerkleTree<E, H, I, ARITY, T>
where
    E: Element,
    H: DigestAlgorithm<E, I, T>,
    I: Index,
    T: NodeValue,
{
    fn membership_verify(
        &self,
        pos: impl Borrow<Self::Index>,
        proof: impl Borrow<Self::NonMembershipProof>,
    ) -> Result<bool, MerkleTreeError> {
        let pos = pos.borrow();
        let proof = proof.borrow();
        if self.height != proof.tree_height() - 1 {
            return Err(MerkleTreeError::InconsistentStructureError(
                "Incompatible membership proof for this merkle tree".to_string(),
            ));
        }
        if *pos != proof.pos {
            return Err(MerkleTreeError::InconsistentStructureError(
                "Inconsistent proof index".to_string(),
            ));
        }
        proof.verify_membership_proof::<H>(self.root.value())
        
    }
}
