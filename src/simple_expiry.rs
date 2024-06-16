use std::clone;
use criterion::Criterion;
use jf_commitment::CommitmentScheme;
use jf_merkle_tree::{
    gadgets::MerkleTreeGadget, 
    prelude::{MerkleCommitment, MerkleTreeScheme, RescueMerkleTree}
};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use jf_relation::{Arithmetization, Circuit, PlonkCircuit, Variable, CircuitError};
use jf_rescue::{
        RescueParameter, commitment::FixedLengthRescueCommitment,
        gadgets::commitment::CommitmentGadget
    };
use jf_plonk::{
    errors::PlonkError,
    proof_system::{PlonkKzgSnark, UniversalSNARK},
    transcript::StandardTranscript,
};
use jf_utils::{test_rng, to_bytes};
use ark_bls12_381::{Bls12_381, Fr};
use ark_std::{rand::Rng, borrow::Borrow};
use ark_ff::UniformRand;
use rand::{rngs::StdRng, SeedableRng, thread_rng};
use crate::{
com_nonce::ComNonce, forest_gadget::{ForestMembershipProofVar, MerkleForestGadget}
};

const EXPIRY_INPUT_LENGTH: usize = 1;
const EXPIRY_INPUT_LENGTH_PLUS_ONE: usize = EXPIRY_INPUT_LENGTH + 1;
type ComSchemeGadget = CommitmentGadget;
type ComScheme = FixedLengthRescueCommitment<Fr, EXPIRY_INPUT_LENGTH, EXPIRY_INPUT_LENGTH_PLUS_ONE>;

#[derive(Clone)]
struct ExpiryAttrs {
    nonce: StdRng,
    expiry: Fr,
}

#[derive(Clone)]
struct ExpiryAttrsInput {
    nonce: [u8; 32],
    expiry_input: [Fr; 1],
    com: Fr,
}



#[derive(Clone, Default)]
pub(crate) struct ExpiryChecker {
    pub (crate) threshold_expiry: Fr,
}

impl ExpiryChecker {
    fn pred(
        &self,
        expiry_attrs: ExpiryAttrsInput,
    ) -> Result<PlonkCircuit<Fr>, CircuitError> {
        let mut circuit = PlonkCircuit::<Fr>::new_turbo_plonk();
        let nonce = expiry_attrs.nonce;
        let mut nonce_seed = StdRng::from_seed(nonce);
        let rng = Fr::rand(&mut nonce_seed);
        let expiry_input = expiry_attrs.expiry_input;
        let com = expiry_attrs.com;
        FixedLengthRescueCommitment::<Fr, 1, 2>::verify(expiry_input, Some(&rng), &com).unwrap();

        let expiry_input_var = expiry_input.iter().map(|x| circuit.create_variable(*x)).collect::<Result<Vec<_>, _>>()?;
        let com_var = circuit.create_variable(com).unwrap();
        let expiry_com_var = ComSchemeGadget::commit(&mut circuit, &expiry_input_var, com_var).unwrap();
        let threshold_expiry_var = circuit.create_variable(self.threshold_expiry).unwrap();
        circuit.enforce_geq(expiry_com_var, threshold_expiry_var).unwrap();
        assert!(circuit.check_circuit_satisfiability(&[]).is_ok());

        Ok(circuit)
    }
    
}



// #[test]
// pub fn tree() {

pub fn bench_expiry(c: &mut Criterion) {
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



    //Forest
    //Generate 255 trees and add the expected root to the list with the random roots
    let mut forest_circuit = PlonkCircuit::<Fr>::new_turbo_plonk();
    let num_trees = 256;
    let mut forest_roots = Vec::with_capacity(num_trees);
    forest_roots.push(expected_root.clone());

    for _ in 1..num_trees {
        let elements = vec![Fr::from(2_u64), Fr::from(4_u64), Fr::from(100_u64)];
        let mt = RescueMerkleTree::<Fr>::from_elems(Some(24), elements).unwrap();
        let random_root = mt.commitment().digest();
        forest_roots.push(random_root);
    }

    // verify that the expected root is a member of the forest
    let _forest_proof_var = MerkleForestGadget::<RescueMerkleTree<Fr>>::is_forest_member_proof(
        &mut forest_circuit,
        forest_roots,
        expected_root
    ).unwrap();

    //TODO: Here the proof is not constraint to the above result
    //Need to fix it !!!
    MerkleForestGadget::<RescueMerkleTree<Fr>>::enforce_forest_membership_proof(
        &mut forest_circuit,
    ).unwrap();
    assert!(forest_circuit.check_circuit_satisfiability(&[]).is_ok());
    forest_circuit.finalize_for_arithmetization().unwrap();

    
    // gen_test_mt_gadget_circuit::<Bls12_381, Fq, _>(); // dont know why this is failing
    let rng = &mut jf_utils::test_rng();

    //srs size
    let tree_degree = circuit.srs_size().unwrap();
    let forest_degree = forest_circuit.srs_size().unwrap();
    let max_degree = tree_degree + forest_degree;


    let srs = PlonkKzgSnark::<Bls12_381>::universal_setup_for_testing(max_degree, rng).unwrap();
    // why the circuit above is `PlonkCircuir<ark_ff::Fp<MontBackend<ark_bn254::FqConfi, 4>, 4>>`
    // don't know why it found the "FrConfig"
    let (tree_pk ,tree_vk) = PlonkKzgSnark::<Bls12_381>::preprocess(&srs, &circuit).unwrap();

    c.bench_function("Expiry show: proving tree", |b| {
        b.iter(|| {
            PlonkKzgSnark::<Bls12_381>::prove::<_, _, StandardTranscript>(
                rng,
                &circuit,
                &tree_pk,
                None,
            ).unwrap();
        })
    });
    let proof = PlonkKzgSnark::<Bls12_381>::prove::<_, _, StandardTranscript>(
        rng,
        &circuit,
        &tree_pk,
        None,
    ).unwrap();
    crate::util::record_size("Tree Proof", &proof);
    let public_inputs = circuit.public_input().unwrap();

    c.bench_function("Expiry show: Verifying tree", |b| {
        b.iter(|| {
            PlonkKzgSnark::<Bls12_381>::verify::<StandardTranscript>(
                &tree_vk,
                &public_inputs,
                &proof,
                None,
            )
        })
    });
    assert!(PlonkKzgSnark::<Bls12_381>::verify::<StandardTranscript>(
        &tree_vk,
        &public_inputs,
        &proof,
        None,
    ).is_ok());

    //Forest proof generation
    let (forest_pk ,forest_vk) = PlonkKzgSnark::<Bls12_381>::preprocess(&srs, &forest_circuit).unwrap();

    c.bench_function("Expiry show: proving forest", |b| {
        b.iter(|| {
            PlonkKzgSnark::<Bls12_381>::prove::<_, _, StandardTranscript>(
                rng,
                &forest_circuit,
                &forest_pk,
                None,
            ).unwrap();
        })
    });
    let forest_proof = PlonkKzgSnark::<Bls12_381>::prove::<_, _, StandardTranscript>(
        rng,
        &forest_circuit,
        &forest_pk,
        None,
    ).unwrap();
    crate::util::record_size("Forest Proof", &forest_proof);

    let public_inputs = forest_circuit.public_input().unwrap();

    c.bench_function("Expiry show: Verifying forest", |b| {
        b.iter(|| {
            PlonkKzgSnark::<Bls12_381>::verify::<StandardTranscript>(
                &forest_vk,
                &public_inputs,
                &forest_proof,
                None,
            )
        })
    });
    assert!(PlonkKzgSnark::<Bls12_381>::verify::<StandardTranscript>(
        &forest_vk,
        &public_inputs,
        &forest_proof,
        None,
    ).is_ok());

    // Commitment for expiry
    let nonce = thread_rng().gen::<[u8; 32]>();
    let mut nonce_seed = StdRng::from_seed(nonce);
    let expiry_rng = Fr::rand(&mut nonce_seed);

    let expiry = Fr::from(10);
    let expiry_input = [expiry];

    let com = FixedLengthRescueCommitment::<Fr, 1, 2>::commit(&expiry_input, Some(&expiry_rng)).unwrap();

    // Expiry
    let expiry_attrs = ExpiryAttrsInput{nonce, expiry_input, com};
    FixedLengthRescueCommitment::<Fr, 1, 2>::verify(expiry_input, Some(&expiry_rng), &com).unwrap();

    //
    let expiry_checker = ExpiryChecker{threshold_expiry: Fr::from(100)};
    let mut expiry_circuit = expiry_checker.pred(expiry_attrs).unwrap();
    expiry_circuit.finalize_for_arithmetization().unwrap();

    let (expiry_pk ,expiry_vk) = PlonkKzgSnark::<Bls12_381>::preprocess(&srs, &expiry_circuit).unwrap();
    c.bench_function("Expiry show: proving expiry", |b| {
        b.iter(|| {
            PlonkKzgSnark::<Bls12_381>::prove::<_, _, StandardTranscript>(
                rng,
                &expiry_circuit,
                &expiry_pk,
                None,
            ).unwrap();
        })
    });
    let expiry_proof = PlonkKzgSnark::<Bls12_381>::prove::<_, _, StandardTranscript>(
        rng,
        &expiry_circuit,
        &expiry_pk,
        None,
    ).unwrap();
    crate::util::record_size("Expiry Proof", &expiry_proof);
    let expiry_public_inputs = expiry_circuit.public_input().unwrap();

    c.bench_function("Expiry show: Verifying expiry", |b| {
        b.iter(|| {
            PlonkKzgSnark::<Bls12_381>::verify::<StandardTranscript>(
                &expiry_vk,
                &expiry_public_inputs,
                &expiry_proof,
                None,
            )
        })
    });
    assert!(PlonkKzgSnark::<Bls12_381>::verify::<StandardTranscript>(
        &expiry_vk,
        &expiry_public_inputs,
        &expiry_proof,
        None,
    ).is_ok());

}