use num_complex::Complex;

#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    pub alpha: Complex<f64>, // Coefficient of |0⟩ state
    pub beta: Complex<f64>,  // Coefficient of |1⟩ state
}

impl Qubit {
    // Create a new qubit in a specific state
    pub fn new(alpha: Complex<f64>, beta: Complex<f64>) -> Self {
        Qubit { alpha, beta }
    }

    // Set qubit to |0⟩ state
    pub fn zero() -> Self {
        Qubit {
            alpha: Complex::new(1.0, 0.0),
            beta: Complex::new(0.0, 0.0),
        }
    }

    // Set qubit to |1⟩ state
    pub fn one() -> Self {
        Qubit {
            alpha: Complex::new(0.0, 0.0),
            beta: Complex::new(1.0, 0.0),
        }
    }
  
    // Pauli-X gate (NOT gate)
    pub fn pauli_x_gate(&mut self) {
        let pauli_x_matrix = [
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        ];
        self.apply_gate(pauli_x_matrix);
    }

    // Pauli-Y gate
    pub fn pauli_y_gate(&mut self) {
        let pauli_y_matrix = [
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
        ];
        self.apply_gate(pauli_y_matrix);
    }

    // Pauli-Z gate
    pub fn pauli_z_gate(&mut self) {
        let pauli_z_matrix = [
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
        ];
        self.apply_gate(pauli_z_matrix);
    }

    // S gate (sqrt(Z) gate)
    pub fn s_gate(&mut self) {
        let s_matrix = [
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)],
            ];
        self.apply_gate(s_matrix);
    }
    
    // S' gate (conjugate of S)
    pub fn s_conjugate_gate(&mut self) {
        let s_conjugate_matrix = [
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
        ];
        self.apply_gate(s_conjugate_matrix);
    }

    // T gate
    pub fn t_gate(&mut self) {
        let t_matrix = [
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.70710678118, 0.70710678118)],
        ];
        self.apply_gate(t_matrix);
    }
    
    // T' gate (conjugate of T)
    pub fn t_conjugate_gate(&mut self) {
        let t_conjugate_matrix = [
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0), Complex::new(0.70710678118, -0.70710678118)],
        ];
        self.apply_gate(t_conjugate_matrix);
    }
    
    // Rotation gate (around the Z axis by theta radians)
    pub fn rotation_gate(&mut self, theta: f64) {
        let rotation_matrix = [
            [Complex::new(theta.cos(), 0.0), Complex::new(-theta.sin(), 0.0)],
            [Complex::new(theta.sin(), 0.0), Complex::new(theta.cos(), 0.0)],
        ];
        self.apply_gate(rotation_matrix);
    }
    
    // Hadamard gate
    pub fn hadamard_gate(&mut self) {
        let sqrt2_inv = 1.0 / 2.0_f64.sqrt();
        let hadamard_matrix = [
            [Complex::new(sqrt2_inv, 0.0), Complex::new(sqrt2_inv, 0.0)],
            [Complex::new(sqrt2_inv, 0.0), Complex::new(-sqrt2_inv, 0.0)],
            ];
        self.apply_gate(hadamard_matrix);
    }

    // Helper function to apply the gate operation on the qubit
    fn apply_gate(&mut self, gate_matrix: [[Complex<f64>; 2]; 2]) {
        let alpha_old = self.alpha;
        let beta_old = self.beta;
        
        self.alpha = gate_matrix[0][0] * alpha_old + gate_matrix[0][1] * beta_old;
        self.beta = gate_matrix[1][0] * alpha_old + gate_matrix[1][1] * beta_old;
    }

    pub fn cnot(&mut self, control: usize, target: usize) {
        let control_state = if control == 0 { self.alpha } else { self.beta };
        if control_state == Complex::new(0.0, 0.0) {
            return;
        }
        if control == 0 {
            self.pauli_x_gate();
        }
        if target == 1 {
            self.hadamard_gate();
        }
    }
    
    // Measure the qubit to collapse it into |0⟩ or |1⟩ state probabilistically
    pub fn measure(&mut self) -> bool {
        let prob_0 = self.alpha.norm_sqr();
        let rand_num: f64 = rand::random(); // Generate a random number between 0 and 1

        // Perform the measurement probabilistically based on the probability amplitudes
        let measurement_result = rand_num <= prob_0;

        // Collapse the qubit state based on the measurement result
        if measurement_result {
            self.alpha = Complex::new(1.0, 0.0);
            self.beta = Complex::new(0.0, 0.0);
        } else {
            self.alpha = Complex::new(0.0, 0.0);
            self.beta = Complex::new(1.0, 0.0);
        }

        measurement_result
    }
}


#[test]
fn test_hadamard_gate() {
    let mut qubit = Qubit::zero();
    qubit.hadamard_gate();
    assert_eq!(qubit.alpha, Complex::new(0.7071067811865475, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.7071067811865475, 0.0));

    let mut qubit = Qubit::one();
    qubit.hadamard_gate();
    assert_eq!(qubit.alpha, Complex::new(0.7071067811865475, 0.0));
    assert_eq!(qubit.beta, Complex::new(-0.7071067811865475, 0.0));
}

#[test]
fn test_pauli_x_gate() {
    let mut qubit = Qubit::zero();
    qubit.pauli_x_gate();
    assert_eq!(qubit.alpha, Complex::new(0.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(1.0, 0.0));

    let mut qubit = Qubit::one();
    qubit.pauli_x_gate();
    assert_eq!(qubit.alpha, Complex::new(1.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 0.0));
}

#[test]
fn test_pauli_y_gate() {
    let mut qubit = Qubit::zero();
    qubit.pauli_y_gate();
    assert_eq!(qubit.alpha, Complex::new(0.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 1.0));

    let mut qubit = Qubit::one();
    qubit.pauli_y_gate();
    assert_eq!(qubit.alpha, Complex::new(0.0, -1.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 0.0));
}

#[test]
fn test_pauli_z_gate() {
    let mut qubit = Qubit::zero();
    qubit.pauli_z_gate();
    assert_eq!(qubit.alpha, Complex::new(1.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 0.0));

    let mut qubit = Qubit::one();
    qubit.pauli_z_gate();
    assert_eq!(qubit.alpha, Complex::new(0.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(-1.0, 0.0));
}

#[test]
fn test_s_gate() {
    let mut qubit = Qubit::zero();
    qubit.s_gate();
    assert_eq!(qubit.alpha, Complex::new(1.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 0.0));

    let mut qubit = Qubit::one();
    qubit.s_gate();
    assert_eq!(qubit.alpha, Complex::new(0.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 1.0));
}

#[test]
fn test_s_conjugate_gate() {
    let mut qubit = Qubit::zero();
    qubit.s_conjugate_gate();
    assert_eq!(qubit.alpha, Complex::new(1.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 0.0));

    let mut qubit = Qubit::one();
    qubit.s_conjugate_gate();
    assert_eq!(qubit.alpha, Complex::new(0.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, -1.0));
}

#[test]
fn test_t_gate() {
    let mut qubit = Qubit::zero();
    qubit.t_gate();
    assert_eq!(qubit.alpha, Complex::new(1.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 0.0));

    let mut qubit = Qubit::one();
    qubit.t_gate();
    assert_eq!(qubit.alpha, Complex::new(0.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.70710678118, 0.70710678118));
}

#[test]
fn test_t_conjugate_gate() {
    let mut qubit = Qubit::zero();
    qubit.t_conjugate_gate();
    assert_eq!(qubit.alpha, Complex::new(1.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.0, 0.0));

    let mut qubit = Qubit::one();
    qubit.t_conjugate_gate();
    assert_eq!(qubit.alpha, Complex::new(0.0, 0.0));
    assert_eq!(qubit.beta, Complex::new(0.70710678118, -0.70710678118));
}

#[test]
fn test_measure() {
    let mut qubit = Qubit::zero();
    let result = qubit.measure();
    if result {
        assert_eq!(qubit.alpha, Complex::new(1.0, 0.0));
        assert_eq!(qubit.beta, Complex::new(0.0, 0.0));
    } else {
        assert_eq!(qubit.alpha, Complex::new(0.0, 0.0));
        assert_eq!(qubit.beta, Complex::new(1.0, 0.0));
    }
}