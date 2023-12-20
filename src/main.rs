// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub mod qubit;
pub mod gates;
pub mod register;
pub mod algorithms;

use gates::*;
use qubit::*;
use num_complex::Complex;
use rand::*;
use algorithms::*;
use ds::*;
use register::*;


fn balanced_function(quantum_register: &mut QuantumRegister) {
    let num_qubits = quantum_register.qubits.len();

    // Create a clone of the quantum register's qubits
    let mut qubits_clone = quantum_register.qubits.clone();

    // Apply CNOT gates to entangle qubits
    for i in 0..num_qubits - 1 {
        cnot_gate(&mut qubits_clone[i], &mut quantum_register.qubits[num_qubits - 1]);
    }
}

fn constant_function(quantum_register: &mut QuantumRegister) {
    for qubit in quantum_register.qubits.iter_mut() {
        gates::pauli_x_gate(qubit);
    }
}


fn main() {
    // let mut quantum_register = QuantumRegister {
    //     qubits: vec![
    //         Qubit::zero(),
    //         Qubit::zero(),
    //         Qubit::zero(),
    //         Qubit::zero(),
    //         Qubit::zero(),
    //     ],
    // };

    // println!("Initial Quantum Register: {:?}", quantum_register);

    // // Deutsch-Jozsa algorithm
    // let is_constant1 = deutsch_jozsa_algorithm(&mut quantum_register, balanced_function);
    // let is_constant2 = deutsch_jozsa_algorithm(&mut quantum_register, constant_function);

    // if is_constant1 {
    //     println!("The function is balanced!");
    // } else {
    //     println!("The function is constant!");
    // }

    // if is_constant2 {
    //     println!("The function is balanced!");
    // } else {
    //     println!("The function is constant!");
    // }

    // let hidden_string: Vec<u8> = vec![0, 1, 1, 0];

    // // Initialize a quantum register with the number of qubits equal to the hidden string length
    // let mut quantum_register = QuantumRegister {
    //     qubits: vec![
    //         Qubit::zero();
    //         hidden_string.len() + 1 // Add one more qubit for the oracle
    //     ],
    // };

    // // println!("Initial Quantum Register: {:?}", quantum_register);

    // // Run the Bernstein-Vazirani algorithm with the known hidden string
    // let result = bernstein_vazirani_algorithm(&mut quantum_register, &hidden_string);

    // // Print the revealed hidden string
    // println!("Revealed Hidden String: {:?}", result);
}
