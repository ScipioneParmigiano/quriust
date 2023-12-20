use super::super::{gates, register, qubit};
use register::*;
use gates::*;
use qubit::*;

// // Deutsch algorithm implementation
// pub fn deutsch_algorithm(q: &mut QuantumRegister, oracle: fn(&mut QuantumRegister)) -> bool {
        
//     // Step 1: Hadamard gate the first qubits
//     for qubit in q.qubits.iter_mut() {
//         hadamard_gate(qubit);
//     }

//     // Step 2: oracle function
//     oracle(q);

//     // Step 3: Apply Hadamard gate to the first qubit again
//     for qubit in q.qubits.iter_mut() {
//         hadamard_gate(qubit);
//     }

//     // Step 5: Measure the first qubit and return the result
//     q.qubits[0].measure()
// }

// pub fn deutsch_jozsa_algorithm(quantum_register: &mut QuantumRegister, oracle: fn(&mut QuantumRegister)) -> bool {
//     // Step 1: Apply Hadamard gate to all qubits to create a superposition
//     for qubit in quantum_register.qubits.iter_mut() {
//         hadamard_gate(qubit);
//     }

//     // Step 2: Apply the oracle (unknown function) to the register
//     oracle(quantum_register);

//     // Step 3: Apply Hadamard gate to all qubits again
//     for qubit in quantum_register.qubits.iter_mut() {
//         hadamard_gate(qubit);
//     }

//     // Step 4: Measure all qubits to obtain the result
//     // If all qubits collapse to |0⟩, the function is constant; otherwise, it's balanced
//     let mut result = true;
//     for qubit in quantum_register.qubits.iter_mut() {
//         if qubit.measure() {
//             result = false;
//             break;
//         }
//     }

//     result
// }


// #[test]
// fn test_ds_const0(){
//     fn constant_oracle(q: &mut QuantumRegister) {
//         q.apply_hadamard_gate();
//     }


//     let mut quantum_reg = QuantumRegister {
//         qubits: vec![
//             Qubit::zero(),
//             Qubit::zero(),
//             Qubit::zero(),
//             Qubit::one(),
//         ],
//     };
//     println!("{:?}", quantum_reg);
//     let result = deutsch_algorithm(&mut quantum_reg, constant_oracle);

//     assert_eq!(true, result);
// }


// #[test]
// fn test_ds_const1(){
//     fn constant_oracle(q: &mut QuantumRegister) {
//         q.apply_hadamard_gate();
//     }


//     let mut quantum_reg = QuantumRegister {
//         qubits: vec![
//             Qubit::zero(),
//             Qubit::zero(),
//             Qubit::zero(),
//             Qubit::one(),
//         ],
//     };
//     println!("{:?}", quantum_reg);
//     let result = deutsch_algorithm(&mut quantum_reg, constant_oracle);

//     assert_eq!(true, result);
// }

// #[test]
// fn test_ds_balanced() {
//     fn balanced_oracle(q: &mut QuantumRegister) {
//         let num_qubits = q.qubits.len();
    
//         // Apply Hadamard gates to all qubits
//         for i in 0..num_qubits {
//             q.qubits[i].apply_hadamard_gate();
//         }
    
//         // Create a sequence of CNOT gates
//         for i in 0..num_qubits - 1 {
//             q.qubits[i].apply_cnot(&mut q.qubits[num_qubits - 1]);
//         }
    
//         // Apply Hadamard gates to all qubits except the last one
//         for i in 0..num_qubits - 1 {
//             q.qubits[i].apply_hadamard_gate();
//         }
//     }
    

//     // Create a quantum register with 2 qubits
//     let mut quantum_reg = QuantumRegister::new(2);

//     // Execute the Deutsch algorithm with a balanced oracle
//     let result = {
//         quantum_reg.qubits[0].apply_hadamard_gate();
//         quantum_reg.qubits[1].apply_hadamard_gate();
//         balanced_oracle(&mut quantum_reg);
//         quantum_reg.qubits[0].apply_hadamard_gate();
//         quantum_reg.qubits[0].measure()
//     };

//     // In a balanced oracle, the result should be determined based on the oracle function
//     // For a balanced function, the result should be different from both constant cases
//     assert_ne!(result, false); // Different from constant zero
//     assert_ne!(result, true);  // Different from constant one
// }


use num_complex::Complex;
use rand::*;

// Define the Qubit and QuantumRegister structs

// Define the Qubit and QuantumRegister structs

fn deutsch_algorithm(quantum_register: &mut QuantumRegister, function: fn(&mut QuantumRegister)) -> bool {
    // Apply Hadamard gate to qubits 0 and 1
    quantum_register.apply_hadamard_gate(0);
    quantum_register.apply_hadamard_gate(1);

    // Apply the function directly on the quantum state
    function(quantum_register);

    // Apply Hadamard gate to the first qubit
    quantum_register.apply_hadamard_gate(0);

    // Measure the first qubit to determine the function's nature (constant or balanced)
    let measurement = quantum_register.qubits[0].measure();

    // Interpret the measurement result and the oracle result to determine the function nature
    measurement
}





#[test]
fn test_d(){
    fn constant_function(q:&mut  QuantumRegister){
        q.qubits[1].pauli_x_gate();
    }

    fn balanced_function(q:&mut  QuantumRegister){
        q.cnot(1, 0);
        q.qubits[1].pauli_x_gate();
    }


    let mut quantum_register_1 = QuantumRegister {
        qubits: vec![
            Qubit::zero(),
            Qubit::one(),
        ],
    };
    let is_constant_true = deutsch_algorithm(&mut quantum_register_1, constant_function);
    assert_eq!(true, is_constant_true);

    // Test with a balanced function
    let mut quantum_register_2 = QuantumRegister {
        qubits: vec![
            Qubit::zero(),
            Qubit::one(),
        ],
    };
    let is_constant_false = deutsch_algorithm(&mut quantum_register_2, balanced_function);
    // assert_eq!(false, is_constant_false);
}