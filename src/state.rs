use std::thread::current;

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

    pub fn get_qubit_count(&self) -> usize {
        (self.amplitudes.len() as f64).log2() as usize
    }

    pub fn pauli_x_gate(&mut self, target_qubit: usize){
        let matr = vec![
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ];

        let pauli_x_matrix = DMatrix::<Complex<f64>>::from_row_slice(2,2,&matr);
        // println!("{}", pauli_x_matrix);
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
        println!{"##"};

        let q = 1.0 / (2.0 as f64).sqrt();
        let matr = vec![
            Complex::new(q, 0.0),
            Complex::new(q, 0.0),
            Complex::new(q, 0.0),
            Complex::new(-q, 0.0),
        ];
        let hadamard_matrix = DMatrix::<Complex<f64>>::from_row_slice(2, 2, &matr);
        println!{"###"};

        self.apply_gate_to_qubit(hadamard_matrix, target_qubit);
    }
    
    fn apply_gate_to_qubit(&mut self, gate: DMatrix<Complex<f64>>, target_qubit: usize) {
        let qubit_count = self.get_qubit_count();
        // println!("q: {qubit_count}");

        let mut full_gate = DMatrix::identity((2 as usize).pow((1) as u32), (2 as usize).pow((1) as u32));//DMatrix::identity(2usize.pow((qubit_count-1) as u32), 2usize.pow((qubit_count-1) as u32));
        if qubit_count==1 {
        // let mut full_gate = DMatrix::identity((2 as usize).pow((qubit_count - 1) as u32), (2 as usize).pow((qubit_count - 1) as u32));//DMatrix::identity(2usize.pow((qubit_count-1) as u32), 2usize.pow((qubit_count-1) as u32));
        full_gate = gate.clone();
        }
        // println!("full gat3:{}", full_gate);
        for i in 0..qubit_count-1{
            println!("sono dentro");
            let current_gate = if i == target_qubit { gate.clone() } else { DMatrix::identity(2, 2) };
            // println!("curr gate: {}", current_gate);
            full_gate = kronecker_product(&full_gate, &current_gate);
        }
        // println!{"####"};

        println!("full gate:{}", full_gate);
                
        self.apply_gate(full_gate);
        // println!{"##### finito"};

    } 
    
    fn apply_gate(&mut self, gate: DMatrix<Complex<f64>>){
        let ampl = DVector::<Complex<f64>>::from_iterator(self.amplitudes.len(), self.amplitudes.clone().into_iter());
        println!("{:?}", gate.shape());
        println!("{:?}", ampl.shape());

        let new_amplitudes = gate * ampl;
        self.amplitudes = new_amplitudes.as_slice().to_vec();
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
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
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
    let cr = ClassicalRegister::new(vec![0,0,0,1]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    println!("\nprob 1: {:?}\n", qr.state());
    qr.x(0);
    println!("\nprob 2: {:?}\n", qr.state());
    let qr_state = qr.state();
    let measured_qr = qr.measure();
    
    assert_eq!(qr_state, vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
    assert_eq!(measured_qr,ClassicalRegister::new(vec![0,0,0,0]));
    

    let cr = ClassicalRegister::new(vec![0,0,0,0,0,0,0,0]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.x(1);
    let qr_state = qr.state();
    let measured_qr = qr.measure();
    
    assert_eq!(qr_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
    assert_eq!(measured_qr,ClassicalRegister::new(vec![0,0,0,0,0,0,0,1]));
}

#[test]
fn pauli_y_test() {
    let cr = ClassicalRegister::new(vec![0,0,0,1]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    println!("\nprob 1: {:?}\n", qr.state());
    qr.y(0);
    println!("\nprob 2: {:?}\n", qr.state());
    let qr_state = qr.state();

    assert_eq!(qr_state, vec![Complex { re: 0.0, im: -1.0 }, Complex { re: 0.0, im: 0.0 }, Complex{re: 0.0, im: 0.0}, Complex{re: 0.0, im: 0.0}]);
    
    let cr = ClassicalRegister::new(vec![0,1]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.y(0);
    let qr_state = qr.state();
    
    assert_eq!(qr_state, vec![Complex { re: 0.0, im: -1.0 }, Complex { re: 0.0, im: 0.0 }]);
}

#[test]
fn pauli_z_test() {
    let cr = ClassicalRegister::new(vec![0,0,0,1]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    qr.z(1);
    let qr_state = qr.state();
    
    assert_eq!(qr_state, vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
    
    let cr = ClassicalRegister::new(vec![0,0, 0, 1]);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    println!("\nprob 1: {:?}\n", qr.state());
    qr.z(0);
    println!("\nprob 2: {:?}\n", qr.state());
    let qr_state = qr.state();

    assert_eq!(qr_state, vec![Complex{re: 0.0, im: 0.0}, Complex{re: -1.0, im: 0.0}, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }]);
}


#[test]
fn hadamard_test() {
    // let q = 1.0/(2.0 as f64).sqrt();

    // // |0001>
    // let cr1 = ClassicalRegister::new(vec![0,0,1,1]);
    // // |01000000>
    // let cr2 = ClassicalRegister::new(vec![0,0,0,0,0,0,0,1]);
    // let mut qr1: QuantumRegister = QuantumRegister::new(&cr1);

    // println!("prob {:?}", qr1.state());
    // let mut qr2: QuantumRegister = QuantumRegister::new(&cr2);
    // qr1.h(0);
    // // qr2.h(0);
    // let qr1_state = qr1.state();
    // // let qr2_state = qr2.state();
    // println!("{:?}", qr1_state);
    // assert_eq!(qr1_state, vec![Complex{re: q, im: 0.2}, Complex{re: -q, im: 0.0}, Complex{re: 0.0, im: 0.0}, Complex{re: 0.0, im: 0.0}]);
    // // assert_eq!(qr2_state, vec![Complex{re: q, im: 0.0}, Complex{re: -q, im: 0.0}]);
}