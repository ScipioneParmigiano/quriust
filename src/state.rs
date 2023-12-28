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
        // println!("{:?}", state);
        // println!("{}", cr.value());
        state.amplitudes[cr.value() as usize] = Complex{re: 1.0, im: 0.0};

        state
    }

    pub fn amplitudes(&self) -> Vec<Complex<f64>>{
        self.amplitudes.clone()
    }

    pub fn pauli_x_gate(&mut self){
        let matr = vec![
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ];

        let pauli_x_matrix = DMatrix::<Complex<f64>>::from_row_slice(2,2,&matr);
        // println!("{}", pauli_x_matrix);
        self.apply_gate(pauli_x_matrix);

    }

    pub fn pauli_y_gate(&mut self) {
        let matr = vec![
            Complex::new(0.0, 0.0),
            Complex::new(0.0, -1.0),
            Complex::new(0.0, 1.0),
            Complex::new(0.0, 0.0),
        ];
    
        let pauli_y_matrix = DMatrix::<Complex<f64>>::from_row_slice(2, 2, &matr);
        self.apply_gate(pauli_y_matrix);

    }

    pub fn pauli_z_gate(&mut self) {
        let matr = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
        ];
    
        let pauli_z_matrix = DMatrix::<Complex<f64>>::from_row_slice(2, 2, &matr);
        self.apply_gate(pauli_z_matrix);
    }    

    fn apply_gate(&mut self, gate: DMatrix<Complex<f64>>){
        let new_amplitudes = gate * DVector::<Complex<f64>>::from_iterator(self.amplitudes.len(), self.amplitudes.clone().into_iter());
        self.amplitudes = new_amplitudes.as_slice().to_vec();
    }
}

#[test]
fn pauli_x_test() {
        
    // |1> goes to |0> i.e. 00
    let cr = ClassicalRegister::new(vec![0,1]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.x();
    let measured_qr = qr.measure();

    assert_eq!(measured_qr,ClassicalRegister::new(vec![0,0]));


    // |0> goes to |1>, i.e. 01
    let cr = ClassicalRegister::new(vec![0,0]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.x();
    let measured_qr = qr.measure();

    assert_eq!(measured_qr,ClassicalRegister::new(vec![0,1]));
}

#[test]
fn pauli_y_test() {
        
}

#[test]
fn pauli_z_test() {
        
}