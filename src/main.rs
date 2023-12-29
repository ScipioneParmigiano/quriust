// pub mod qubit;
// pub mod register;
// pub mod algorithms;
pub mod registers;
pub mod state;

use registers::*;
use state::*;

fn main(){
    let q = 1.0/(2.0 as f64).sqrt();

    let cr1 = ClassicalRegister::new(vec![0,0]);
    let cr2 = ClassicalRegister::new(vec![0,0,0,0,0,0,0,1]);
    let mut qr1: QuantumRegister = QuantumRegister::new(&cr1);
    let mut qr2: QuantumRegister = QuantumRegister::new(&cr2);
    qr1.h(0);
    // qr2.h(0);
    let qr1_state = qr1.state();
    let qr2_state = qr2.state();
    println!("state: {:?}", qr1_state);
}

use num_complex::Complex;
use nalgebra::{DMatrix};
fn kronecker_product(a: &DMatrix<Complex<f64>>, b: &DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
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