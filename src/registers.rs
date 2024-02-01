use super::state::State;
use num_complex::Complex;

/// Represents a classical register
#[derive(PartialEq, Clone, Debug)]
pub struct ClassicalRegister{
    bits: Vec<usize>
}

impl ClassicalRegister{
    /// Creates a new classical register with the provided bits
    pub fn new(bits: Vec<usize>)-> ClassicalRegister {
        ClassicalRegister{bits}
    }

    /// Creates a classical register with all bits set to zero
    pub fn zeros(len: usize) -> ClassicalRegister {
        ClassicalRegister::new(vec![0; len])
    }

    /// Returns the length of the classical register
    pub fn len(&self) -> usize {
        self.bits.len()
    } 

    /// Creates a classical register from the given value and width
    pub fn from_value(width: usize, value: u32) -> ClassicalRegister {
        let mut bits = Vec::new();
        let mut remaining_value = value;

        for i in (0..width).rev() {
            let pos: u32 = i as u32;
            let bit_value = (2 as u32).pow(pos);
    
            // Insert a one or a zero at the end of the vector.
            if bit_value <= remaining_value {
                remaining_value -= bit_value;
                bits.push(1);
            } else {
                bits.push(0);
            }
        }

        ClassicalRegister::new(bits)
    }

    /// Returns the value represented by the classical register
    pub fn value(&self) -> u32 {
        let mut value = 0 as u32;
    
        for (pos, bit) in self.bits.iter().rev().enumerate() {
            if *bit != 0 {
                value += (2 as u32).pow(pos as u32);
            }
        }
    
        value
    }

    /// Returns the bits of the classical register
    pub fn bits(&self) -> Vec<usize>{
        self.bits.clone()
    }

}

/// Represents a quantum register
#[derive(Clone, Debug)]
pub struct QuantumRegister{
    pub measured: bool,
    pub prob_amplitudes: State,
    pub len: usize,
}

impl QuantumRegister {
    /// Creates a new quantum register from a classical register
    pub fn new(cr: &ClassicalRegister) -> QuantumRegister{
        QuantumRegister {
            measured: false,
            prob_amplitudes: State::from_cr(cr),
            len: cr.len(),
        }
    }

    /// Initializes a quantum register with the specified number of qubits
    pub fn init(n_qubit: usize) -> QuantumRegister{
        let cr = &ClassicalRegister::new(vec![0; 2_i32.pow(n_qubit as u32) as usize]);
        QuantumRegister {
            measured: false,
            prob_amplitudes: State::from_cr(cr),
            len: (cr.len()as f32).log2() as usize,
        }
    }

    /// Returns the length of the quantum register
    pub fn len(&self) -> usize{
        self.len
    }

    /// Returns the number of qubits in the quantum register
    pub fn get_qubit_count(&self) -> usize {
        (self.len() as f64).log2() as usize
    }

    /// Measures the quantum register
    pub fn measure(&mut self) -> ClassicalRegister {
        assert_eq!(false, self.measured);
        self.measured = true;
    
        let mut cum = 0.0;
        let rand_num: f64 = rand::random();

        for (val, coefficient) in self.prob_amplitudes.amplitudes().iter().enumerate() {
            let scaled_prob = coefficient.norm_sqr();
            cum += scaled_prob;

            if rand_num <= cum {
                return ClassicalRegister::from_value(self.len, val as u32);
            }

        }

        ClassicalRegister::from_value(self.len, 0)
    }

    /// Returns the state of the quantum register
    pub fn state(&self)-> Vec<Complex<f64>> {
        self.prob_amplitudes.amplitudes()
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
        assert_eq!(false, self.measured);
        self.prob_amplitudes.pauli_x_gate(target_qubit);
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
        assert_eq!(false, self.measured);
        self.prob_amplitudes.pauli_y_gate(target_qubit);
    }
    
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
        assert_eq!(false, self.measured);
        self.prob_amplitudes.pauli_z_gate(target_qubit);
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
        assert_eq!(false, self.measured);
        self.prob_amplitudes.hadamard_gate(target_qubit);
    }
    
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
        self.prob_amplitudes.cnot_gate(control_qubit, target_qubit);
    }
    
    /// Measures a specific qubit in the quantum register
    pub fn measure_qubit(&mut self, qubit_to_measure: usize) -> bool {
        let measured_classical_register = self.measure(); // Measure all qubits


        let qubit_state_index = measured_classical_register.value() >> (self.len() - qubit_to_measure);
        let qubit_state = (qubit_state_index & 1) != 0;

        qubit_state // Return true if the qubit state is |1>, else false
    }
}


#[test]
fn test_classical_value() {
    let cr = ClassicalRegister::new(vec![0, 1, 0, 1, 0]);

    assert_eq!(10, cr.value());
    assert_eq!(cr, ClassicalRegister::from_value(5, cr.value()));
}

#[test]
fn test_init() {
    let cr = ClassicalRegister::zeros(8);
    let qr: QuantumRegister = QuantumRegister::new(&cr);

    assert_eq!(false, qr.measured);
    assert_eq!(8, qr.len());
}

#[test]
fn test_measurement() {
    let cr = ClassicalRegister::zeros(2);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    let qr2 = qr.measure();

    assert_eq!(cr, qr2);
    assert!(qr.measured);

    let cr = ClassicalRegister::zeros(2);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.x(1);
    let qr2 = qr.measure();

    assert_eq!(ClassicalRegister{bits: vec![0,1]}, qr2);

    let mut qr: QuantumRegister = QuantumRegister::init(2);
    qr.x(1);
    let qr2 = qr.measure();

    assert_eq!(ClassicalRegister{bits: vec![0,1]}, qr2);

    let mut qr: QuantumRegister = QuantumRegister::init(3);
    qr.x(2);
    qr.x(1);
    qr.x(3);
    let qr2 = qr.measure();

    assert_eq!(ClassicalRegister{bits: vec![1,1,1]}, qr2);
}

#[test]
fn test_measure_single_qubit(){
    let mut qr1: QuantumRegister = QuantumRegister::init(1);
    let mut qr2: QuantumRegister = QuantumRegister::init(5);

    qr1.x(1); 

    let m1 = qr1.measure_qubit(1); 
    let m2 = qr2.measure_qubit(5); 

    assert_eq!(m1, true); 
    assert_eq!(m2, false);
}

