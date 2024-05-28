use ark_bls12_381::Fq;
use jf_merkle_tree::gadgets::{MerkleTreeGadget, UniversalMerkleTreeGadget};
use jf_relation::{Circuit, PlonkCircuit};
use jf_merkle_tree::{MerkleTreeScheme, MerkleCommitment, UniversalMerkleTreeScheme, prelude::RescueSparseMerkleTree};
use jf_rescue::RescueParameter;

use hashbrown::HashMap;
use num_bigint::BigUint;

#[test]
fn test_universal_mt_gadget() {
    tree_membership_proof_circuit_test::<Fq>();

}

type SparseMerkleTree<F> = RescueSparseMerkleTree<BigUint, F>;
const TREE_HIGH: usize = 4;

pub fn tree_membership_proof_circuit_test<F: RescueParameter>() -> Result<(), Box<dyn std::error::Error>> {
    let mut circuit = PlonkCircuit::<F>::new_turbo_plonk();

    // An element we care about is inserted into the tree
    let uid = BigUint::from(3u64);

    // Create a sparse Merkle tree with height ˋTREE_HIGHˋ
    let mut hashmap = HashMap::new();
    hashmap.insert(BigUint::from(1u64), F::from(2u64));
    hashmap.insert(BigUint::from(2u64), F::from(2u64));

    let mt = SparseMerkleTree::<F>::from_kv_set(TREE_HIGH, &hashmap).unwrap();
    let expected_root = mt.commitment().digest();

    // Create non-membership proof
    let proof = mt.universal_lookup(&uid).expect_not_found().unwrap();

    // Circuit computation with SparseMerkleTree non-membership proof
    let non_elem_idx_var = circuit.create_variable(uid.into()).unwrap();
    let proof_var = UniversalMerkleTreeGadget::<SparseMerkleTree<F>>::create_non_membership_proof_variable(
        &mut circuit,
        &proof,
    ).unwrap();
    
    let root_var = MerkleTreeGadget::<SparseMerkleTree<F>>::create_root_variable(
        &mut circuit,
        expected_root,
    ).unwrap();

    // Enforce non-membership proof
    UniversalMerkleTreeGadget::<SparseMerkleTree<F>>::enforce_non_membership_proof(
        &mut circuit,
        non_elem_idx_var,
        proof_var,
        root_var,
    )?;

    assert!(circuit.check_circuit_satisfiability(&[]).is_ok());

    // Create membership proof
    let mut circuit1 = PlonkCircuit::<F>::new_turbo_plonk();
    let member_uid = BigUint::from(1u64);
    let (_retreieved_elem, membership_proof) = mt.lookup(&member_uid).expect_ok().unwrap();

    //Circuit computation with SparseMerkleTree membership proof
    let elem_idx_var = circuit1.create_variable(member_uid.into()).unwrap();
    let membership_proof_var = MerkleTreeGadget::<SparseMerkleTree<F>>::create_membership_proof_variable(
        &mut circuit1,
        &membership_proof,
    ).unwrap();
    
    MerkleTreeGadget::<SparseMerkleTree<F>>::enforce_membership_proof(
        &mut circuit1,
        elem_idx_var,
        membership_proof_var,
        root_var,
    ).unwrap();

    assert!(circuit1.check_circuit_satisfiability(&[]).is_ok());

    Ok(())
}
