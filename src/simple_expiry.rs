use core::num;

use criterion::{Criterion, criterion_group, criterion_main};
use jf_merkle_tree::{
    gadgets::MerkleTreeGadget, 
    prelude::{MerkleCommitment, MerkleTreeScheme, RescueMerkleTree}
};
use jf_relation::{Arithmetization, Circuit, PlonkCircuit, Variable};
use jf_rescue::RescueParameter;
use jf_plonk::{
    errors::PlonkError,
    proof_system::{PlonkKzgSnark, UniversalSNARK},
    transcript::StandardTranscript,
};
use jf_utils::test_rng;
// use ark_bls12_377::{Bls12_377, Fr};
use ark_bls12_381::{Bls12_381, Fr};

// #[test]
pub fn tree(c: &mut Criterion) {
    let mut circuit = PlonkCircuit::<Fr>::new_turbo_plonk();
    // Create a 3-ary MT, instantiated with a Rescue-based hash, of height 1.
    let elements = vec![Fr::from(1_u64), Fr::from(2_u64), Fr::from(100_u64)];
    let mt = RescueMerkleTree::<Fr>::from_elems(Some(24), elements).unwrap();
    let expected_root = mt.commitment().digest();
    // Get a proof for the element in position 2
    let (_, proof) = mt.lookup(2).expect_ok().unwrap();

    // Circuit computation with a MT
    let elem_idx = circuit.create_variable(2_u64.into()).unwrap();
    let proof_var =
        MerkleTreeGadget::<RescueMerkleTree<Fr>>::create_membership_proof_variable(
            &mut circuit,
            &proof
        )
        .unwrap();
    let root_var =
        MerkleTreeGadget::<RescueMerkleTree<Fr>>::create_root_variable(
            &mut circuit,
            expected_root
        )
        .unwrap();
    MerkleTreeGadget::<RescueMerkleTree<Fr>>::enforce_membership_proof(
        &mut circuit,
        elem_idx,
        proof_var,
        root_var
    )
    .unwrap();
    assert!(circuit.check_circuit_satisfiability(&[]).is_ok());
    circuit.finalize_for_arithmetization().unwrap();

    // // gen_test_mt_gadget_circuit::<Bls12_381, Fq, _>(); // dont know why this is failing
    let rng = &mut jf_utils::test_rng();

    let max_degree = circuit.srs_size().unwrap() + 2;

    let srs = PlonkKzgSnark::<Bls12_381>::universal_setup_for_testing(max_degree, rng).unwrap();
    // why the circuit above is `PlonkCircuir<ark_ff::Fp<MontBackend<ark_bn254::FqConfi, 4>, 4>>`
    // don't know why it found the "FrConfig"
    let (pk ,vk) = PlonkKzgSnark::<Bls12_381>::preprocess(&srs, &circuit).unwrap();

    c.bench_function("Expiry show: proving tree", |b| {
        b.iter(|| {
            PlonkKzgSnark::<Bls12_381>::prove::<_, _, StandardTranscript>(
                rng,
                &circuit,
                &pk,
                None,
            ).unwrap();
        })
    });
    let proof = PlonkKzgSnark::<Bls12_381>::prove::<_, _, StandardTranscript>(
        rng,
        &circuit,
        &pk,
        None,
    ).unwrap();
    let public_inputs = circuit.public_input().unwrap();

    c.bench_function("Expiry show: Verifying tree", |b| {
        b.iter(|| {
            PlonkKzgSnark::<Bls12_381>::verify::<StandardTranscript>(
                &vk,
                &public_inputs,
                &proof,
                None,
            )
        })
    });
    assert!(PlonkKzgSnark::<Bls12_381>::verify::<StandardTranscript>(
        &vk,
        &public_inputs,
        &proof,
        None,
    ).is_ok());

    //Forest
    //Generate 256 trees
    let num_trees = 256;
    let mut forest_roots = Vec::with_capacity(num_trees);

    for _ in 0..num_trees {
        let elements = vec![Fr::from(1_u64), Fr::from(2_u64), Fr::from(100_u64)];
        let mt = RescueMerkleTree::<Fr>::from_elems(Some(24), elements).unwrap();
        let expected_root = mt.commitment().digest();
        forest_roots.push(expected_root);
    }
    
    


}