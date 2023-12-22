use super::super::register;
use register::QuantumRegister;

pub fn deutsch_jozsa_algorithm(quantum_register: &mut QuantumRegister, function: fn(&mut QuantumRegister)) -> bool {
    let num_qubits = quantum_register.qubits.len();
    // Apply Hadamard gate to create a superposition of all possible input states
    for i in 0..num_qubits {
        quantum_register.hadamard_gate(i);
    }

    // Apply function f(x) to the quantum register
    function(quantum_register);

    // Apply Hadamard gate to the input qubits again
    for i in 0..num_qubits {
        quantum_register.hadamard_gate(i);
    }

    // Measure all qubits to get the result
    let measurement_results = quantum_register.measure_all();

    // Check if all measurement results are the same
    let first_measurement = measurement_results[0];
    for &result in measurement_results.iter().skip(1) {
        if result != first_measurement {
            return false; // Function is balanced
        }
    }

    true // True if is constant
}




#[test]
fn test_deutsch_jozsa_constant_function() {
    fn contant_function(_q: &mut QuantumRegister) {}
    
    let num_qubits = 4;
    let mut quantum_register = QuantumRegister::new(num_qubits);
    
    let is_constant = deutsch_jozsa_algorithm(&mut quantum_register, contant_function);
    
    assert!(is_constant);
}

#[test]
fn test_deutsch_jozsa_balanced_function() {
    fn balanced_function(q: &mut QuantumRegister) {
        q.pauli_x_gate(1);
        q.pauli_z_gate(1)
    }

    let num_qubits = 4;
    let mut quantum_register = QuantumRegister::new(num_qubits);

    let is_constant = deutsch_jozsa_algorithm(&mut quantum_register, balanced_function);

    assert!(!is_constant);
}
