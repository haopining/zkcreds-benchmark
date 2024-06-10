
use ark_ff::PrimeField;
use jf_relation::{Circuit, CircuitError, PlonkCircuit, Variable};
use jf_merkle_tree::gadgets:: MembershipProof;
use jf_merkle_tree::{MerkleTreeScheme, prelude::RescueMerkleTree, ToTraversalPath};
use jf_rescue::RescueParameter;

type NodeVal<F> = <RescueMerkleTree<F> as MerkleTreeScheme>::NodeValue;

pub trait MerkleForestGadget<M>
where
    M: MerkleTreeScheme,
    M::NodeValue: PrimeField,
{
    type ForestMembershipProofVar;

    fn is_forest_member_proof(
        &mut self,
        forest_roots: Vec<M::NodeValue>,
        member_root: M::NodeValue
    ) -> Result<Self::ForestMembershipProofVar, CircuitError>;

    // fn is_forest_member(
    //     &mut self,
    //     forest_proof_var: Self::ForestMembershipProofVar,
    // ) -> Result<BoolVar, CircuitError>;

    fn enforce_forest_membership_proof(
        &mut self,
    ) -> Result<(), CircuitError>;
    
}

pub struct ForestMembershipProofVar {
    roots_vars: Vec<Variable>,
    member_root_var: Variable,
}

impl<T> MerkleForestGadget<T> for PlonkCircuit<T::NodeValue>
where
    T: MerkleTreeScheme,
    T::MembershipProof: MembershipProof<T::NodeValue, T::Index, T::NodeValue>,
    T::NodeValue: PrimeField + RescueParameter,
    T::Index: ToTraversalPath<3>,

{
    type ForestMembershipProofVar = ForestMembershipProofVar;


    fn is_forest_member_proof(
        &mut self,
        forest_roots: Vec<NodeVal<T::NodeValue>>,
        member_root: NodeVal<T::NodeValue>
    ) -> Result<Self::ForestMembershipProofVar, CircuitError> {
        // Fix
        // impl Convert all elements in forest_roots to Variable
       
        let roots_vars = forest_roots.iter().map(|root: &<T as MerkleTreeScheme>::NodeValue| self.create_variable(*root)).collect::<Result<Vec<_>, _>>()?;
        let member_root_id = forest_roots.iter().enumerate().find(| id | id.1 == &member_root).unwrap().0;
        let member_root_var = roots_vars[member_root_id];
        
        Ok(ForestMembershipProofVar {
            roots_vars,
            member_root_var,
        })
    }

    // fn is_forest_member(
    //     &mut self,
    //     forest_proof_var: Self::ForestMembershipProofVar,
    // ) -> Result<BoolVar, CircuitError> {
    //     let true_var = self.create_boolean_variable(true);
    //     self.is_equal(forest_proof_var.proof_var, true_var)
    // }



    fn enforce_forest_membership_proof(
        &mut self,
    ) -> Result<(), CircuitError> {
        // create table vairables which are the roots of the forest
        let true_var = self.create_boolean_variable(true)?;
        self.enforce_true(true_var.into())

        
    }

}

mod test {
    use crate::forest_gadget::MerkleForestGadget;
    use ark_bls12_381::Fr;
    use jf_relation::{Circuit, PlonkCircuit, Arithmetization, Variable};
    use jf_merkle_tree::prelude::{MerkleCommitment, MerkleTreeScheme, RescueMerkleTree}
    ;

    // #[test]
    // fn test_forest_gadget() {
    //     test_forest_gadget_helper();
    // }
    #[test]
    pub(crate) fn test_forest_gadget_helper() {
        let mut circuit = PlonkCircuit::<Fr>::new_turbo_plonk();
        let elements = vec![Fr::from(1_u64), Fr::from(2_u64), Fr::from(100_u64)];
        let mt = RescueMerkleTree::<Fr>::from_elems(Some(256), elements).unwrap();
        let member_root = mt.commitment().digest();
        
        // Gen forest
        // let num_trees = 256;
        let num_trees = 10;

        let mut forest_roots = Vec::with_capacity(num_trees);
        forest_roots.push(member_root.clone());

        for _ in 1..num_trees {
            let elements = vec![Fr::from(2_u64), Fr::from(4_u64), Fr::from(100_u64)];
            let mt = RescueMerkleTree::<Fr>::from_elems(Some(24), elements).unwrap();
            let random_root = mt.commitment().digest();
            forest_roots.push(random_root);
        }

        let _forest_proof_var = MerkleForestGadget::<RescueMerkleTree<Fr>>::is_forest_member_proof(
            &mut circuit,
            forest_roots,
            member_root
        ).unwrap();

        MerkleForestGadget::<RescueMerkleTree<Fr>>::enforce_forest_membership_proof(
            &mut circuit,
        ).unwrap();

        assert!(circuit.check_circuit_satisfiability(&[]).is_ok());
        println!("Forest satisfiable")

    }
}