use ark_ff::PrimeField;
use jf_merkle_tree::NodeValue;
use jf_relation::{BoolVar, Circuit, CircuitError, PlonkCircuit, Variable};
use jf_merkle_tree::gadgets::{DigestAlgorithmGadget, MerkleTreeGadget, MembershipProof, RescueDigestGadget, Merkle3AryMembershipProofVar};
use jf_merkle_tree::{MerkleTreeScheme, UniversalMerkleTreeScheme, prelude::RescueMerkleTree, ToTraversalPath};
use jf_rescue::{RescueParameter, gadgets::RescueGadget};


pub struct MerkleForest<MT: MerkleTreeScheme> {
    roots: Vec<MT::NodeValue>,
    num_trees: usize,
}

impl<MT: MerkleTreeScheme> MerkleForest<MT> {
    pub fn new(num_trees: usize) -> Self {
            Self {
            roots: vec![MT::NodeValue::default(); num_trees],
            num_trees,
        }
    }

    pub fn update(&mut self, tree_idx: usize, root: MT::NodeValue) {
        self.roots[tree_idx] = root;

    }

    pub fn is_member(&mut self, input_root: MT::NodeValue) -> bool {
        self.roots.contains(&input_root)
    }
}

// Create a Merkle Forest Gadget
// Implement the MerkleForestGadget trait for the PlonkCircuit

pub struct MerkleForestProofVar {
    roots: Vec<Variable>,
    member_root_var: Variable,
}


pub trait MerkleForestGadget<M>
where
    M: MerkleTreeScheme,
    M::NodeValue: PrimeField,
{
    type MerkleForestProofVar;

    type DigestGadget: DigestAlgorithmGadget<M::NodeValue>;

    fn create_forest_root_variable(&mut self, roots: Vec<M::NodeValue>) -> Result<Variable, CircuitError>;

    fn enforce_forest_membership_proof(
        &mut self,
        roots: Vec<M::NodeValue>,
        member_root_var: M::NodeValue,
    ) -> Result<(), CircuitError>;
}

impl<T> MerkleForestGadget<T> for PlonkCircuit<T::NodeValue>
where
    T: MerkleTreeScheme,
    T::MembershipProof: MembershipProof<T::NodeValue, T::Index, T::NodeValue>,
    T::NodeValue: PrimeField + RescueParameter,
    T::Index: ToTraversalPath<3>,

{
    type MerkleForestProofVar = MerkleForestProofVar;

    type DigestGadget = RescueDigestGadget;

    fn create_forest_root_variable(
        &mut self,
        roots: Vec<T::NodeValue>,
    ) -> Result<Variable, CircuitError> {
        let root_vars: Vec<Variable> = roots
            .into_iter()
            .map(|root| self.create_variable(root))
            .collect::<Result<Vec<Variable>, CircuitError>>()?;
        Self::DigestGadget::digest(self, &root_vars)
    }

    fn enforce_forest_membership_proof(
        &mut self,
        roots: Vec<T::NodeValue>,
        member_root_var: T::NodeValue,
    ) -> Result<(), CircuitError> {
        let bool_val = self.create_boolean_variable(roots.contains(&member_root_var));
        match bool_val {
            Ok(val) => self.enforce_true(val.into()),
            Err(e) => Err(e),
        }
    }    
}

// Tests for the Merkle Forest Gadget

#[cfg(test)]
mod test {
    use ark_bls12_381::Fr;
    use ark_std::test_rng;
    use jf_merkle_tree::{prelude::RescueMerkleTree, MerkleCommitment, MerkleTreeScheme, ToTraversalPath};
    use jf_relation::Circuit;
    use jf_relation::PlonkCircuit;
    use forest_gadget::{MerkleForest, MerkleForestGadget};
    use ark_ff::UniformRand;

    #[test]
    fn test_merkle_forest_gadget() {
        // Set up a random number generator
        let mut rng = test_rng();

        // Create a Plonk circuit
        let mut circuit = PlonkCircuit::<Fr>::new_turbo_plonk();

        // Number of trees in the forest
        let num_trees = 3;

        // Create the Merkle trees and add their roots to the forest
        let mut forest = MerkleForest::<RescueMerkleTree<Fr>>::new(num_trees);
        for i in 0..num_trees {
            // Generate random elements for the Merkle tree
            let elements: Vec<Fr> = (0..4).map(|_| Fr::rand(&mut rng)).collect();
            
            // Create a Merkle tree from the elements
            let mt = RescueMerkleTree::<Fr>::from_elems(Some(2), elements.clone()).unwrap();

            // Get the root of the Merkle tree
            let root = mt.commitment().digest(); // Call the commitment method on the MerkleTree struct

            // Update the forest with the Merkle tree root
            forest.update(i, root);
        }

    

        // Choose a root to verify membership
        let member_root = forest.roots[1]; // Choose the second tree's root
        
        circuit
            .enforce_forest_membership_proof(forest.roots, member_root)
            .unwrap();

        // Check circuit satisfiability
        assert!(circuit.check_circuit_satisfiability(&[]).is_ok());

        // Test a non-member root (this should fail)
        let non_member_root = Fr::rand(&mut rng);
        let result = circuit.enforce_forest_membership_proof(forest.roots.clone(), non_member_root);
        assert!(result.is_err());
    }
}

