use jf_plonk::{PlonkCircuit, PlonkError, PlonkType};


fn gen_circuit_for_bench<F: PrimeField>(
    num_gates: usize,
    plonk_type: PlonkType,
) -> Result<PlonkCircuit<F>, PlonkError> {
    let range_bit_len = 8;
    let mut cs: PlonkCircuit<F> = match plonk_type {
        PlonkType::TurboPlonk => PlonkCircuit::new_turbo_plonk(),
        PlonkType::UltraPlonk => PlonkCircuit::new_ultra_plonk(range_bit_len),
    };
    let mut a = cs.zero();
    for _ in 0..num_gates - 10 {
        a = cs.add(a, cs.one())?;
    }
    // Finalize the circuit.
    cs.finalize_for_arithmetization()?;

    Ok(cs)
}

fn gen_circuit_for_tree_bench<F: 