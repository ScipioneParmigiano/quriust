use super::super::registers;
use registers::QuantumRegister;

pub fn deutsch_jozsa_algorithm(q: &mut QuantumRegister, function: fn(&mut QuantumRegister)) -> bool {
    let num_qubits = q.len();
    // println!("{}", num_qubits);

    // Apply Hadamard gate
    for i in 1..=num_qubits {
        // println!("{}", i);
        q.h(i);
    }

    // Apply function f(x) to the quantum register
    function(q);


    // Apply Hadamard gate
    for i in 1..=num_qubits- 1 {
        // println!("h su {}", i);
        q.h(i);
    }

    // Measure qubits to get the result
    let mut m: Vec<bool> = Vec::with_capacity(num_qubits-1);
    for i in 1..=num_qubits - 1 {
        let is_one = q.measure_qubit(i);
        m.push(is_one);
        println!("m bro {:?} alla i {i}", m);
    }

    println!("{:?}", m);
    let result = has_true_value(m);
    // println!("resultt: {}", result);
    !result
}


fn has_true_value(vec: Vec<bool>) -> bool {
    for value in vec {
        if value {
            // If a true value is found, return true immediately
            return true;
        }
    }
    // If no true value is found, return false
    false
}



#[test]
fn test_deutsch_jozsa_constant_function() {
    fn contant_function(_q: &mut QuantumRegister) {}
    
    let mut q = QuantumRegister::init(3);
    q.x(3);
    
    let is_constant = deutsch_jozsa_algorithm(& mut q, contant_function);
    
    assert_eq!(is_constant, true);
}

#[test]
fn test_deutsch_jozsa_balanced_function() {
    fn balanced_function(q: &mut QuantumRegister) {
        q.cnot(2,1);
        q.x(1);
    }

    let mut q = QuantumRegister::init(2);
    q.x(2);

    let is_constant = deutsch_jozsa_algorithm(& mut q, balanced_function);

    assert_eq!(is_constant, false);
}
    
#[test]
fn test_has_true_function() {
    let vec = vec![true, false, false, false, false];
    assert!(has_true_value(vec));

    let vec = vec![false, false, false, false];
    assert_eq!(has_true_value(vec), false);    
}