// pub mod qubit;
// pub mod register;
// pub mod algorithms;
pub mod registers;
pub mod state;

use registers::*;
use state::*;

fn main(){
    let cr = ClassicalRegister::new(vec![0, 1]);
    // let cr = ClassicalRegister::from_value(2, 1);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);

    // |0> goes to |1>
    qr.x();
    let measured_qr = qr.measure();

    println!("{:?}", cr);
    println!("{:?}", qr);
    println!("{:?}", measured_qr);

}