use super::super::register;
use register::*;

pub fn deutsch_algorithm(quantum_register: &mut QuantumRegister, function: fn(&mut QuantumRegister)) -> bool {
    // Apply Hadamard gate to qubits 0 and 1
    quantum_register.hadamard_gate(0);
    quantum_register.hadamard_gate(1);

    // Apply the function on the quantum state
    function(quantum_register);

    // Apply Hadamard gate to the first qubit
    quantum_register.hadamard_gate(0);

    // Measure the first qubit to determine the function's nature (constant or balanced)
    let measurement_result = quantum_register.qubits[0].measure();

    // True for constant function, false otherwise
    measurement_result
}


#[test]
fn test_deutsch_algorithm_constant_function() {
    fn constant_function(_q: &mut QuantumRegister) {}
    
    let mut quantum_register = QuantumRegister::new(2);
    let is_constant = deutsch_algorithm(&mut quantum_register, constant_function);
    
    assert!(is_constant);
}

#[test]
fn test_deutsch_algorithm_balanced_function() {
    fn balanced_function(q: &mut QuantumRegister) {
        q.pauli_z_gate(0);
    }
    
    let mut quantum_register = QuantumRegister::new(2);
    let is_constant = deutsch_algorithm(&mut quantum_register, balanced_function);

    assert!(!is_constant);
}