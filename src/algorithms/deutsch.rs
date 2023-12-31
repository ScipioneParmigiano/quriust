use super::super::registers;
use registers::*;

pub fn deutsch_algorithm(q: &mut QuantumRegister, function: fn(&mut QuantumRegister)) -> bool {
    // Apply hadamard gate
    println!("len{}",q.len());
    for i in 1..q.len(){
        println!("run: {}", i);
        q.h(i);
    }

    // Apply the function on the quantum state
    function(q);

    // Apply Hadamard gate to the first qubit
    q.h(1);

    // Measure the first qubit to determine the function's nature (constant or balanced)
    let is_one = q.measure_qubit(0);
    !is_one
}


#[test]
fn test_deutsch_algorithm_constant_function() {
    fn constant_function(_q: &mut QuantumRegister) {}
    
    let mut q = QuantumRegister::init(6);
    let is_constant = deutsch_algorithm(&mut q, constant_function);
    
    assert!(is_constant);
}

#[test]
fn test_deutsch_algorithm_balanced_function() {
    fn balanced_function(q: &mut QuantumRegister) {
        // q.z(1);

        // we need a cnot
    }
    
    let mut q = QuantumRegister::init(4);
    let is_constant = deutsch_algorithm(&mut q, balanced_function);
    
    // assert!(is_constant);
}