#[cfg(test)]
mod test {use ark_bls12_381::Fq;
    use jf_merkle_tree::gadgets::{MerkleTreeGadget, UniversalMerkleTreeGadget};
    use jf_relation::{Circuit, PlonkCircuit};
    use jf_merkle_tree::{MerkleTreeScheme, MerkleCommitment, UniversalMerkleTreeScheme, prelude::RescueSparseMerkleTree};
    use hashbrown::HashMap;
    use num_bigint::BigUint;
    
    type SparseMerkleTree<F> = RescueSparseMerkleTree<BigUint, F>;

    fn membership_proof_circuit_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut circuit = PlonkCircuit::<Fq>::new_turbo_plonk();
    
        // 创建一个 3-ary 稀疏 Merkle 树，高度为 2
        let mut hashmap = HashMap::new();
        hashmap.insert(BigUint::from(1u64), Fq::from(2u64));
        hashmap.insert(BigUint::from(2u64), Fq::from(2u64));
        hashmap.insert(BigUint::from(1u64), Fq::from(3u64));
        let mt = SparseMerkleTree::<Fq>::from_kv_set(24, &hashmap).unwrap();
        let expected_root = mt.commitment().digest();
    
        // 生成非成员证明
        let uid = BigUint::from(3u64);
        let proof = mt.universal_lookup(&uid).expect_not_found().unwrap();
    
        // 创建电路变量
        let non_elem_idx_var = circuit.create_variable(uid.into()).unwrap();
        let proof_var = UniversalMerkleTreeGadget::<SparseMerkleTree<Fq>>::create_non_membership_proof_variable(
            &mut circuit,
            &proof,
        )?;
        let root_var = MerkleTreeGadget::<SparseMerkleTree<Fq>>::create_root_variable(
            &mut circuit,
            expected_root,
        )?;
    
        // 在电路中强制非成员证明
        UniversalMerkleTreeGadget::<SparseMerkleTree<Fq>>::enforce_non_membership_proof(
            &mut circuit,
            non_elem_idx_var,
            proof_var,
            root_var,
        )?;
    
        assert!(circuit.check_circuit_satisfiability(&[]).is_ok());
    
        Ok(())
    }
}
