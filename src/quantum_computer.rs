use super::registers::*;

/// Represents a quantum computer, i.e. a wrap up of a register object.
/// This is intended to be more beginner friendly and more accessible
#[derive(Clone, Debug)]
pub struct QuantumComputer{
    q_register: QuantumRegister,
    c_register: ClassicalRegister
}

impl QuantumComputer {
    /// Creates a new quantum register initialized to |0>
    pub fn new(n_qubit: usize) -> QuantumComputer{
        QuantumComputer {
            q_register: QuantumRegister::init(n_qubit),
            c_register: ClassicalRegister::zeros(n_qubit),
        }
    }

    /// Measure the quantum register, which collapses in the classical one
    pub fn measure(mut self) {
        let len = self.q_register.len();
        let mut cum = 0.0;
        let rand_num: f64 = rand::random();

        for (val, coefficient) in self.q_register.prob_amplitudes.amplitudes().iter().enumerate() {
            let scaled_prob = coefficient.norm_sqr();
            cum += scaled_prob;

            if rand_num <= cum {
                self.c_register = ClassicalRegister::from_value(len, val as u32);
            }

        }

        self.c_register = ClassicalRegister::from_value(len, 0)
    }

    /// Applies the Pauli-X gate (also known as the NOT gate) to the specified target qubit.
    ///
    /// The Pauli-X gate flips the state of the target qubit, changing |0⟩ to |1⟩ and vice versa.
    /// Matrix representation:
    ///     | 0 1 |
    ///     | 1 0 |
    ///
    /// # Arguments
    ///
    /// * `target_qubit` - The index of the target qubit to which the gate is applied.
    pub fn x(&mut self, target_qubit: usize) {
        self.q_register.x(target_qubit);
    }

    /// Applies the Pauli-Y gate to the specified target qubit.
    ///
    /// The Pauli-Y gate introduces a phase flip (rotation by π around the Y-axis) if the qubit is in |1⟩ state.
    /// Matrix representation:
    ///     | 0 -i |
    ///     | i  0 |
    ///
    /// # Arguments
    ///
    /// * `target_qubit` - The index of the target qubit to which the gate is applied.
    pub fn y(&mut self, target_qubit: usize) {
        self.q_register.y(target_qubit);    }
    
    /// Applies the Pauli-Z gate to the specified target qubit.
    ///
    /// The Pauli-Z gate introduces a phase flip (rotation by π around the Z-axis) if the qubit is in |1⟩ state.
    /// Matrix representation:
    ///     | 1  0 |
    ///     | 0 -1 |
    ///
    /// # Arguments
    ///
    /// * `target_qubit` - The index of the target qubit to which the gate is applied.
    pub fn z(&mut self, target_qubit: usize) {
        self.q_register.z(target_qubit); 
    }
    
    /// Applies the Hadamard gate to the specified target qubit.
    ///
    /// The Hadamard gate creates superposition by putting the qubit in a state of equal probability of |0⟩ and |1⟩.
    /// Matrix representation:
    ///     | 1/sqrt(2)  1/sqrt(2) |
    ///     | 1/sqrt(2) -1/sqrt(2) |
    ///
    /// # Arguments
    ///
    /// * `target_qubit` - The index of the target qubit to which the gate is applied.
    pub fn h(&mut self, target_qubit: usize) {
        self.q_register.h(target_qubit);    }
    
    /// Applies the Controlled-NOT (CNOT) gate to the specified control and target qubits.
    ///
    /// The CNOT gate flips the target qubit if and only if the control qubit is in the |1⟩ state.
    /// Matrix representation:
    ///     | 1 0 0 0 |
    ///     | 0 1 0 0 |
    ///     | 0 0 0 1 |
    ///     | 0 0 1 0 |
    ///
    /// # Arguments
    ///
    /// * `control_qubit` - The index of the control qubit.
    /// * `target_qubit` - The index of the target qubit.
    pub fn cnot(&mut self, control_qubit: usize, target_qubit: usize){
        self.q_register.cnot(control_qubit, target_qubit);
    }
}
