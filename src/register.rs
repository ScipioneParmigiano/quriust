use num_complex::Complex; // Import the `num_complex` crate for complex numbers
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

    pub fn apply_hadamard_gate(&mut self, qubit_index: usize) {
        self.qubits[qubit_index].apply_hadamard_gate();
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
            // If control qubit is |0⟩, do nothing (CNOT is a controlled operation)
            return;
        }

        // Apply NOT operation (X gate) to the target qubit
        // (Assuming you have defined a method to apply X gate on Qubit)
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
            // If either control qubit is |0⟩, do nothing (Toffoli is a controlled operation)
            return;
        }

        // Apply NOT operation (X gate) to the target qubit
        // (Assuming you have defined a method to apply X gate on Qubit)
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