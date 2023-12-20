use num_complex::Complex;
use super::qubit::*;

#[derive(Debug, Clone)]
pub struct QuantumRegister {
    pub qubits: Vec<Qubit>,
}

impl QuantumRegister {
    pub fn new(num_qubits: usize) -> Self {
        let mut qubits = Vec::with_capacity(num_qubits);
        for _ in 0..num_qubits {
            // Initialize qubits in |0⟩ state
            qubits.push(Qubit::zero());
        }
        QuantumRegister { qubits }
    }

    pub fn hadamard_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].hadamard_gate();
    }

    pub fn pauli_x_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].pauli_x_gate();
    }

    pub fn pauli_y_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].pauli_y_gate();
    }

    pub fn pauli_z_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].pauli_z_gate();
    }

    pub fn s_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].s_gate();
    }

    pub fn s_conjugate_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].s_conjugate_gate();
    }

    pub fn t_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].t_gate();
    }

    pub fn t_conjugate_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].t_conjugate_gate();
    }

    pub fn rotation_gate(&mut self, qubit_index: usize, theta: f64) {
        self.qubits[qubit_index].rotation_gate(theta);
    }

    pub fn cnot(&mut self, control: usize, target: usize) {
        // Apply CNOT gate: Flipping target qubit if control qubit is |1⟩
        let control_state = self.qubits[control].alpha;
        if control_state == Complex::new(0.0, 0.0) {
            // If control qubit is |0⟩, do nothing
            return;
        }

        // Apply NOT operation (X gate) to the target qubit
        self.qubits[target].pauli_x_gate();
    }

    pub fn swap(&mut self, qubit1: usize, qubit2: usize) {
        // Swap the states of qubit1 and qubit2 in the QuantumRegister
        self.qubits.swap(qubit1, qubit2);
    }

    pub fn toffoli(&mut self, control1: usize, control2: usize, target: usize) {
        // Apply Toffoli gate: Perform a CNOT operation on target qubit 
        // if both control1 and control2 qubits are |1⟩
        let control1_state = self.qubits[control1].alpha;
        let control2_state = self.qubits[control2].alpha;
        if control1_state == Complex::new(0.0, 0.0) || control2_state == Complex::new(0.0, 0.0) {
            // If either control qubit is |0⟩, do nothing
            return;
        }

        // Apply NOT operation (X gate) to the target qubit
        self.qubits[target].pauli_x_gate();
    }

    // Measure the entire QuantumRegister
    pub fn measure_all(&mut self) -> Vec<bool> {
        let mut measurement_results = Vec::new();
        for qubit in &mut self.qubits {
            measurement_results.push(qubit.measure());
        }
        measurement_results
    }
}


#[test]
fn test_quantum_register_creation() {
    let num_qubits = 10;
    let quantum_register = QuantumRegister::new(num_qubits);

    assert_eq!(quantum_register.qubits.len(), num_qubits);
    for qubit in &quantum_register.qubits {
        assert_eq!(qubit.alpha, Complex::new(1.0, 0.0));
        assert_eq!(qubit.beta, Complex::new(0.0, 0.0));
    }
}

#[test]
fn test_hadamard_gate_on_register() {
    let mut quantum_register = QuantumRegister::new(3);
    quantum_register.hadamard_gate(1);

    assert_eq!(quantum_register.qubits[1].alpha, Complex::new(0.7071067811865475, 0.0));
    assert_eq!(quantum_register.qubits[1].beta, Complex::new(0.7071067811865475, 0.0));
}

#[test]
fn test_cnot_operation() {
    let mut quantum_register = QuantumRegister::new(3);

    quantum_register.qubits[0].alpha = Complex::new(0.0, 0.0); // Set control qubit to |1⟩
    quantum_register.cnot(0, 1);

    assert_eq!(quantum_register.qubits[1].alpha, Complex::new(1.0, 0.0));
    assert_eq!(quantum_register.qubits[1].beta, Complex::new(0.0, 0.0));
}

#[test]
fn test_toffoli_operation() {
let mut quantum_register = QuantumRegister::new(3);

quantum_register.qubits[0].alpha = Complex::new(0.0, 0.0);
quantum_register.qubits[1].alpha = Complex::new(0.0, 0.0);
quantum_register.toffoli(0, 1, 2);

assert_eq!(quantum_register.qubits[2].alpha, Complex::new(1.0, 0.0));
assert_eq!(quantum_register.qubits[2].beta, Complex::new(0.0, 0.0));
}

#[test]
fn test_measurement_of_register() {
    let mut quantum_register = QuantumRegister::new(12);
    let measurement_results = quantum_register.measure_all();

    assert_eq!(measurement_results.len(), 12);
}