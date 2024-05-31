#[cfg(test)]
mod tests {
use core::hash;
use ark_bls12_381::Fr;
use criterion::Criterion;
use jf_merkle_tree::{
    gadgets::{Merkle3AryMembershipProofVar, MerkleTreeGadget}, 
    prelude::{MerkleCommitment, MerkleTreeScheme, RescueSparseMerkleTree,MerkleProof, UniversalMerkleTreeScheme}};
// use jf_merkle_tree::gadgets::{MerkleTreeGadget, UniversalMerkleTreeGadget};
// use jf_plonk::{
//     errors::PlonkError,
//     proof_system::{PlonkKzgSnark, UniversalSNARK},
//     transcript::StandardTranscript,
// };
use jf_relation::{Arithmetization, Circuit, PlonkCircuit, Variable};
use jf_rescue::RescueParameter;
use hashbrown::HashMap;
use num_bigint::BigUint;
use crate::forest_gadget::{MerkleForest, MerkleForestRoots};

const LOG2_NUM_LEAVES: u32 = 31;
const LOG2_NUM_TREES: u32 = 8;
const TREE_HEIGHT: u32 = LOG2_NUM_LEAVES + 1 - LOG2_NUM_TREES; // 24
const NUM_TREES: usize = 2usize.pow(LOG2_NUM_TREES); // 2^8 = 256
const TREE_HEIGHT_USIZE: usize = 24;

// use crate universal_merkletree_extension::UniversalMerkleTreeMembershipVerify;


pub fn test<F: RescueParameter>() {

    let elem = F::from(310u64);
    
    // // Create a sparse Merkle tree with height ˋTREE_HEIGHT_USIZEˋ
    let mut hashmap = HashMap::new();
        hashmap.insert(BigUint::from(1u64), F::from(2u64));
        hashmap.insert(BigUint::from(2u64), F::from(2u64));
        hashmap.insert(BigUint::from(1u64), F::from(3u64));
    
    let mt = RescueSparseMerkleTree::<BigUint, F>::from_kv_set(TREE_HEIGHT_USIZE, &hashmap).unwrap();
    assert_eq!(mt.num_leaves(), hashmap.len() as u64);

    let mut proof = mt.lookup(BigUint::from(1u64)).expect_ok().unwrap();

    // let mut proof = mt
    //     .universal_lookup(BigUint::from(1u64))
    //     .expect_ok()
    //     .unwrap()
    //     .1;
    
    let root = mt.commitment().digest();
    let pos_u64: u64 = 1;
    
    // TRACE 不到！！！！！！！！！！
    let verify_result = mt.verify(&root, &pos_u64, &proof);

    // //Auth Path
    // let auth_path = mt.lookup(&uid).expect_ok().unwrap().1;

    let mut merkle_tree_circuit = PlonkCircuit::<F>::new_turbo_plonk();
    let elem_idx_var: Variable = merkle_tree_circuit.create_variable(F::from(2u64).into()).unwrap();
    
    let proof_var = 
    MerkleTreeGadget::<SparseMerkleTree<F>>::create_membership_proof_variable(
        &mut merkle_tree_circuit,
        &proof,
    ).unwrap();

    let root_var = MerkleTreeGadget::<SparseMerkleTree<F>>::create_root_variable(
        &mut merkle_tree_circuit,
        expected_root,
    ).unwrap();

    // Bench proving tree   
    // TODO: Add benching for proving tree

    // create tree circuit
    MerkleTreeGadget::<SparseMerkleTree<F>>::enforce_membership_proof(
        &mut merkle_tree_circuit,
        elem_idx_var,
        proof_var,
        root_var,
    ).unwrap(); 
 

    assert!(merkle_tree_circuit.check_circuit_satisfiability(&[]).is_ok());
    
    // match merkle_tree_circuit.check_circuit_satisfiability(&[]) {
    //     Ok(_) => println!("Circuit is satisfiable."),
    //     Err(e) => println!("Circuit error: {:?}", e),
    // }
    
    // create forest circuit
    // Create a forest of 256 trees
    // let mut forest = MerkleForest::<F> {
    //     trees: vec![mt; NUM_TREES],
    //     _marker: core::marker::PhantomData::<F>,
    // };

    // create expiry circuit

    // aggregate circuit

    // Generate proof and Bench circuit
    // setup
    // preprocess
    // prove
    // verify


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use num_bigint::BigUint;
//     use criterion::Criterion;
//     use std::collections::HashMap;
//     use ark_bls12_381::Fr;
//     #[test]
//     fn test_bench_expiry() {
//         let mut criterion = Criterion::default();
//         bench_expiry::<Fr>(&mut criterion);
//     }

//}
}

#[test] fn main() {
    test::<Fr>();
}
}