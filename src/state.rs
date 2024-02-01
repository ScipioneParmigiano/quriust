use num_complex::Complex;
use super::registers::{ClassicalRegister, QuantumRegister};

use nalgebra::{DMatrix, DVector};

/// Represents the state of a quantum system, defined by a vector of complex amplitudes
#[derive(Debug, Clone)]
pub struct State{
    amplitudes: Vec<Complex<f64>>
}

impl State{
     /// Creates a new quantum state with the specified number of amplitudes, initialized to zero
    pub fn new(n: usize)-> State{
        State{amplitudes: vec![Complex{re: 0.0, im: 0.0}; n]}
    }

    /// Creates a quantum state from a classical register with all amplitudes set to zero
    /// except the amplitude corresponding to the value of the classical register, which is set to one
    pub fn from_cr(cr: &ClassicalRegister) -> State{
        let mut state = State::new(cr.len());
        state.amplitudes[cr.value() as usize] = Complex{re: 1.0, im: 0.0};
        
        state
    }

    /// Returns the amplitudes of the quantum state    
    pub fn amplitudes(&self) -> Vec<Complex<f64>>{
        self.amplitudes.clone()
    }

    /// Returns the number of qubits represented by the quantum state
    pub fn get_qubit_count(&self) -> usize {
        (self.amplitudes.len() as f64).log2() as usize
    }

    /// Applies the Pauli-X gate (NOT gate) to the specified target qubit.
    ///
    /// The Pauli-X gate flips the state of the target qubit
    pub fn pauli_x_gate(&mut self, target_qubit: usize){
        let matr = vec![
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ];

        let pauli_x_matrix = DMatrix::<Complex<f64>>::from_row_slice(2,2,&matr);
        self.apply_gate_to_qubit(pauli_x_matrix, target_qubit);

    }

    /// Applies the Pauli-Y gate to the specified target qubit.
    ///
    /// The Pauli-Y gate introduces a phase flip if the qubit is in the |1⟩ state
    pub fn pauli_y_gate(&mut self, target_qubit: usize) {
        let matr = vec![
            Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
            Complex::new(0.0, 1.0), Complex::new(0.0, 0.0),
        ];
    
        let pauli_y_matrix = DMatrix::<Complex<f64>>::from_row_slice(2, 2, &matr);
        self.apply_gate_to_qubit(pauli_y_matrix, target_qubit);

    }

    /// Applies the Pauli-Z gate to the specified target qubit.
    ///
    /// The Pauli-Z gate introduces a phase flip iif the qubit is in the |1⟩ state  
    pub fn pauli_z_gate(&mut self, target_qubit: usize) {
        let matr = vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0),
        ];
    
        let pauli_z_matrix = DMatrix::<Complex<f64>>::from_row_slice(2, 2, &matr);
        self.apply_gate_to_qubit(pauli_z_matrix, target_qubit);
    }  

    /// Applies the Hadamard gate to the specified target qubit.
    ///
    /// The Hadamard gate creates superposition by putting the qubit in a state of equal probability of |0⟩ and |1⟩
    pub fn hadamard_gate(&mut self, target_qubit: usize) {
        let q = 1.0 / (2.0 as f64).sqrt();
        let matr = vec![
            Complex::new(q, 0.0),
            Complex::new(q, 0.0),
            Complex::new(q, 0.0),
            Complex::new(-q, 0.0),
        ];
        let hadamard_matrix = DMatrix::<Complex<f64>>::from_row_slice(2, 2, &matr);

        self.apply_gate_to_qubit(hadamard_matrix, target_qubit);
    }
    
    /// Applies a quantum gate to the specified target qubit.
    ///
    /// This method applies the given gate to the target qubit using the Kronecker product method. Note that you can use this
    /// method also for user-defined gates
    pub fn apply_gate_to_qubit(&mut self, gate: DMatrix<Complex<f64>>, target_qubit: usize) {
        let qubit_count = self.get_qubit_count();
        assert!(qubit_count >= target_qubit);
        assert!(target_qubit!=0);

        let mut full_gate = DMatrix::identity(2, 2);

        for i in 1..=qubit_count {
            let current_gate = if i == target_qubit {
                gate.clone() // Apply the provided gate if the current qubit is the target qubit
            } else {
                DMatrix::identity(2, 2) // Identity gate for other qubits
            };

            full_gate = if i == 1 {
                current_gate.clone() // Assign the gate to the first qubit directly
            } else {
                kronecker_product(&current_gate, &full_gate) // Apply Kronecker product for subsequent qubits
            };
        }

        self.apply_gate(full_gate);
    } 
    
    /// Applies a quantum gate to the entire quantum state.
    ///
    /// This method applies the given gate to the entire quantum state vector
    fn apply_gate(&mut self, gate: DMatrix<Complex<f64>>){
        let ampl = DVector::<Complex<f64>>::from_iterator(self.amplitudes.len(), self.amplitudes.clone().into_iter());

        let new_amplitudes = gate * ampl;
        self.amplitudes = new_amplitudes.as_slice().to_vec();
    }

    /// Applies the Controlled-NOT (CNOT) gate to the specified control and target qubits.
    ///
    /// The CNOT gate flips the target qubit if and only if the control qubit is in the |1⟩ state
    pub fn cnot_gate(&mut self, control_qubit: usize, target_qubit: usize) {
        let num_qubits = (self.amplitudes.len() as f32).log2() as usize;
        assert!(control_qubit > 0 && target_qubit > 0 && control_qubit <= num_qubits && target_qubit <= num_qubits);

        let mut cnot_gate = DMatrix::<Complex<f64>>::identity(1 << num_qubits, 1 << num_qubits);

        for i in 0..(1 << num_qubits) {
            let bit_i = 1 << i;

            if (i & bit_i) == bit_i {
                let gate = if i & (1 << (control_qubit - 1)) != 0 && i & (1 << (target_qubit - 1)) == 0 {
                    // Apply CNOT gate
                    DMatrix::<Complex<f64>>::from_row_slice(2, 2, &[
                        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                        Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
                    ])
                } else {
                    // Apply identity gate
                    DMatrix::<Complex<f64>>::identity(2, 2)
                };

                cnot_gate = kronecker_product(&cnot_gate, &gate);
            }
        }
        

        self.apply_gate(cnot_gate);
    }
}

/// Calculates the Kronecker product of two matrices
pub fn kronecker_product(a: &DMatrix<Complex<f64>>, b: &DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
    let mut result = DMatrix::zeros(a.nrows() * b.nrows(), a.ncols() * b.ncols());

    for i in 0..a.nrows() {
        for j in 0..a.ncols() {
            let submatrix = b * a[(i, j)];
            let (submatrix_nrows, submatrix_ncols) = submatrix.shape();

            for k in 0..submatrix_nrows {
                for l in 0..submatrix_ncols {
                    result[(i * submatrix_nrows + k, j * submatrix_ncols + l)] = submatrix[(k, l)];
                }
            }
        }
    }

    result
}


#[test]
fn length_test() {  
    let cr = ClassicalRegister::new(vec![0,0,0,0,0,0,0,0]);
    let qr: QuantumRegister = QuantumRegister::new(&cr);
    let qr_len = qr.get_qubit_count();
    
    assert_eq!(3, qr_len);
}

#[test]
fn kronecker_product_test() {  
    let a = DMatrix::from_row_slice(2, 1, &[
        Complex::new(1.0, 0.0), Complex::new(2.0, 0.0),
    ]);

    let b = DMatrix::from_row_slice(3, 1, &[
        Complex::new(1.0, 1.0), Complex::new(2.0, 2.0), Complex::new(2.0, 2.0),
    ]);

    let result = kronecker_product(&a, &b);
    let expected = DMatrix::from_row_slice(6, 1, &[
        Complex::new(1.0, 1.0),
        Complex::new(2.0, 2.0),
        Complex::new(2.0, 2.0),
        Complex::new(2.0, 2.0),
        Complex::new(4.0, 4.0),
        Complex::new(4.0, 4.0),
    ]);

    assert_eq!(expected, result);
}

#[test]
fn pauli_x_test() {  
    let cr = ClassicalRegister::new(vec![0,0,0,0,0,0,0,0]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.x(3);
    let qr_state = qr.state();
    let measured_qr = qr.measure();
    
    assert_eq!(qr_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
    assert_eq!(measured_qr,ClassicalRegister::new(vec![0,0,0,0,0,1,0,0]));
    
    
    let cr = ClassicalRegister::new(vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.x(1);
    let qr_state = qr.state();
    let measured_qr = qr.measure();
    
    assert_eq!(qr_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
    assert_eq!(measured_qr,ClassicalRegister::new(vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0]));
}

#[test]
fn pauli_y_test() {
    let cr = ClassicalRegister::new(vec![0,0,1,0]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.y(1);
    let qr_state = qr.state();

    assert_eq!(qr_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 1.0 }]);
    
    let cr = ClassicalRegister::new(vec![0,0,0,0]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.y(2);
    let qr_state = qr.state();
    
    assert_eq!(qr_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 1.0 }, Complex { re: 0.0, im: 0.0 }]);
}

#[test]
fn pauli_z_test() {
    let cr = ClassicalRegister::new(vec![0,0,1,0]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.z(2);
    let qr_state = qr.state();
    
    assert_eq!(qr_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: -1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
    
    let cr = ClassicalRegister::new(vec![0,0,0,0,0,1,0,1]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.z(3);
    let qr_state = qr.state();

    assert_eq!(qr_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: -1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
}


#[test]
fn hadamard_test() {
    let q = 1.0/(2.0 as f64).sqrt();

    let cr1 = ClassicalRegister::new(vec![0,0,0,0]);
    let mut qr1: QuantumRegister = QuantumRegister::new(&cr1);
    qr1.h(2);
    let qr1_state = qr1.state();
    assert_eq!(qr1_state, vec![Complex { re: q, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: q, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);


    let cr2 = ClassicalRegister::new(vec![0,0,0,0,0,1,0,0]);
    let mut qr2: QuantumRegister = QuantumRegister::new(&cr2);
    qr2.h(1);       
    let qr2_state = qr2.state();
    assert_eq!(qr2_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: q, im: 0.0 }, Complex { re: q, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
}




#[test]
fn cnot_test() {
    let cr1 = ClassicalRegister::new(vec![0,0,0,0]);
    let mut qr1: QuantumRegister = QuantumRegister::new(&cr1);
    qr1.x(1);
    qr1.cnot(1, 2);
    let qr1_state = qr1.state();
    assert_eq!(qr1_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);


    let cr2 = ClassicalRegister::new(vec![0,0,0,0,0,0,0,0]);
    let mut qr2: QuantumRegister = QuantumRegister::new(&cr2);
    let qr2_state = qr2.state();
    qr2.cnot(1, 2);       
    let qr2_state = qr2.state();
    assert_eq!(qr2_state, vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
}
