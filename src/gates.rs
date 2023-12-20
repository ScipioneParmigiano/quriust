use super::qubit::*;
use num_complex::Complex;

// Implement quantum gates
pub fn pauli_x_gate(qubit: &mut Qubit) {
    let new_alpha = qubit.beta;
    let new_beta = qubit.alpha;
    qubit.alpha = new_alpha;
    qubit.beta = new_beta;
}

pub fn hadamard_gate(qubit: &mut Qubit) {
    let sqrt2_inv = 1.0 / 2.0_f64.sqrt();
    let hadamard_matrix = [
        [Complex::new(sqrt2_inv, 0.0), Complex::new(sqrt2_inv, 0.0)],
        [Complex::new(sqrt2_inv, 0.0), Complex::new(-sqrt2_inv, 0.0)],
    ];

    let alpha_old = qubit.alpha;
    let beta_old = qubit.beta;

    qubit.alpha = hadamard_matrix[0][0] * alpha_old + hadamard_matrix[0][1] * beta_old;
    qubit.beta = hadamard_matrix[1][0] * alpha_old + hadamard_matrix[1][1] * beta_old;
}

pub fn cnot_gate(control_qubit: &mut Qubit, target_qubit: &mut Qubit) {
    let alpha_t = target_qubit.alpha;
    let beta_t = target_qubit.beta;

    target_qubit.alpha = control_qubit.alpha * alpha_t + control_qubit.beta * beta_t;
    target_qubit.beta = control_qubit.beta * alpha_t + control_qubit.alpha * beta_t;
}