use super::super::registers;
use registers::*;

/// Executes the Deutsch algorithm using the provided function.
///
/// The Deutsch algorithm determines whether a given function is constant or balanced
/// by evaluating it on a quantum computer.
///
/// # Arguments
///
/// * `q` - A mutable reference to the quantum register on which the algorithm operates.
/// * `function` - The function to be evaluated on the quantum register.
///
/// # Returns
///
/// A boolean indicating whether the provided function is constant (`false`) or balanced (`true`).

pub fn deutsch_algorithm(q: &mut QuantumRegister, function: fn(&mut QuantumRegister)) -> bool {
    // Apply hadamard gate
    for i in 1..=q.len(){
        q.h(i);
    }

    // Apply the function on the quantum state
    function(q);

    // Apply Hadamard gate to the first qubit
    q.h(1);
    q.h(2);

    // Measure the first qubit to determine the function's nature (constant or balanced)
    let is_one = q.measure_qubit(2);
    is_one
}


#[test]
fn test_deutsch_algorithm_constant_function() {
    fn constant_function(_q: &mut QuantumRegister) {
    }
    
    let mut q = QuantumRegister::init(2);
    q.x(2);
    let is_balanced = deutsch_algorithm(&mut q, constant_function);
    
    assert_eq!(is_balanced, false);
}

#[test]
fn test_deutsch_algorithm_balanced_function() {
    fn balanced_function(q: &mut QuantumRegister) {
        q.cnot(1,2);
        q.x(1);
    }
    
    let mut q = QuantumRegister::init(2);
    q.x(1);
    let is_balanced = deutsch_algorithm(&mut q, balanced_function);
    
    assert_eq!(is_balanced, true);
}