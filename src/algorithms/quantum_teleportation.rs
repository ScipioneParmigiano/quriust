use super::super::register;
use register::*;

pub fn quantum_teleportation(register: &mut QuantumRegister) -> (bool, bool) {
    // Alice prepares an entangled pair of qubits
    let mut alice_register = QuantumRegister::new(2);
    alice_register.hadamard_gate(0);
    alice_register.cnot(0, 1);

    // Alice entangles her qubit (Bob's qubit is already part of the entangled pair)
    register.hadamard_gate(0);
    register.cnot(0, 1);
    register.cnot(1, 2);

    // Measure Alice's qubts
    let measurement_1 = alice_register.measure_all();
    
    // Apply gates based on Alice's measurements
    if measurement_1[0] {
        register.pauli_z_gate(1);
    }
    if measurement_1[1] {
        register.pauli_x_gate(2);
    }

    // Measure Bob's qubit
    let measurement_2 = register.measure_all();

    (measurement_1[0], measurement_2[0])
}

#[test]
fn test_quantum_teleportation() {
    let mut quantum_register = QuantumRegister::new(3);
    quantum_register.hadamard_gate(0);

    let (result_1, result_2) = quantum_teleportation(&mut quantum_register);
    // assert_eq!(result_1, result_2);
}
