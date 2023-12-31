use num_complex::Complex;
use super::registers::{ClassicalRegister, QuantumRegister};

use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone)]
pub struct State{
    amplitudes: Vec<Complex<f64>>
}

impl State{
    pub fn new(n: usize)-> State{
        State{amplitudes: vec![Complex{re: 0.0, im: 0.0}; n]}
    }

    pub fn from_cr(cr: &ClassicalRegister) -> State{
        let mut state = State::new(cr.len());
        state.amplitudes[cr.value() as usize] = Complex{re: 1.0, im: 0.0};
        
        state
    }

    pub fn amplitudes(&self) -> Vec<Complex<f64>>{
        self.amplitudes.clone()
    }

    pub fn get_qubit_count(&self) -> usize {
        (self.amplitudes.len() as f64).log2() as usize
    }

    pub fn pauli_x_gate(&mut self, target_qubit: usize){
        let matr = vec![
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ];

        let pauli_x_matrix = DMatrix::<Complex<f64>>::from_row_slice(2,2,&matr);
        self.apply_gate_to_qubit(pauli_x_matrix, target_qubit);

    }

    pub fn pauli_y_gate(&mut self, target_qubit: usize) {
        let matr = vec![
            Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
            Complex::new(0.0, 1.0), Complex::new(0.0, 0.0),
        ];
    
        let pauli_y_matrix = DMatrix::<Complex<f64>>::from_row_slice(2, 2, &matr);
        self.apply_gate_to_qubit(pauli_y_matrix, target_qubit);

    }

    pub fn pauli_z_gate(&mut self, target_qubit: usize) {
        let matr = vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0),
        ];
    
        let pauli_z_matrix = DMatrix::<Complex<f64>>::from_row_slice(2, 2, &matr);
        self.apply_gate_to_qubit(pauli_z_matrix, target_qubit);
    }  

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
    
    fn apply_gate_to_qubit(&mut self, gate: DMatrix<Complex<f64>>, target_qubit: usize) {
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
    
    fn apply_gate(&mut self, gate: DMatrix<Complex<f64>>){
        let ampl = DVector::<Complex<f64>>::from_iterator(self.amplitudes.len(), self.amplitudes.clone().into_iter());
        let new_amplitudes = gate * ampl;
        self.amplitudes = new_amplitudes.as_slice().to_vec();
    }

    pub fn cnot_gate(&mut self, control_qubit: usize, target_qubit: usize) {
        assert!(control_qubit > 0 && target_qubit > 0);

        let mut cnot_matrix = DMatrix::<Complex<f64>>::identity(2, 2);

        for i in 1..=self.get_qubit_count() {
            let gate = if i == control_qubit {
                DMatrix::<Complex<f64>>::from_row_slice(2, 2, &[
                    Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
                    Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                ])
            } else if i == target_qubit {
                DMatrix::<Complex<f64>>::identity(2, 2)
            } else {
                DMatrix::<Complex<f64>>::identity(2, 2)
            };

            cnot_matrix = if i == 1 {
                gate.clone()
            } else {
                kronecker_product(&gate, &cnot_matrix)
            };
        }

        self.apply_gate(cnot_matrix);
    }
}


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
    assert_eq!(qr1_state, vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);


    let cr2 = ClassicalRegister::new(vec![0,0,0,0,0,0,0,0]);
    let mut qr2: QuantumRegister = QuantumRegister::new(&cr2);
    let qr2_state = qr2.state();
    println!("aa{:?}", qr2_state);
    qr2.cnot(1, 2);       
    let qr2_state = qr2.state();
    assert_eq!(qr2_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
}


