use super::state::State;
use num_complex::Complex;

#[derive(PartialEq, Clone, Debug)]
pub struct ClassicalRegister{
    bits: Vec<usize>
}

impl ClassicalRegister{
    pub fn new(bits: Vec<usize>)-> ClassicalRegister {
        // check bits are either 0s or 1s
        ClassicalRegister{bits}
    }

    pub fn zeros(len: usize) -> ClassicalRegister {
        ClassicalRegister::new(vec![0; len])
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    } 

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

    pub fn value(&self) -> u32 {
        let mut value = 0 as u32;
    
        for (pos, bit) in self.bits.iter().rev().enumerate() {
            if *bit != 0 {
                value += (2 as u32).pow(pos as u32);
            }
        }
    
        value
    }

    pub fn bits(&self) -> Vec<usize>{
        self.bits.clone()
    }

}

#[derive(Debug)]
pub struct QuantumRegister{
    measured: bool,
    prob_amplitudes: State,
    len: usize,
}

impl QuantumRegister {
    pub fn new(cr: &ClassicalRegister) -> QuantumRegister{
        QuantumRegister {
            measured: false,
            prob_amplitudes: State::from_cr(cr),
            len: cr.len(),
        }
    }

    pub fn init(n_qubit: usize) -> QuantumRegister{
        let cr = &ClassicalRegister::new(vec![0; 2_i32.pow(n_qubit as u32) as usize]);
        QuantumRegister {
            measured: false,
            prob_amplitudes: State::from_cr(cr),
            len: (cr.len()as f32).log2() as usize,
        }
    }

    pub fn len(&self) -> usize{
        self.len
    }

    pub fn get_qubit_count(&self) -> usize {
        println!("get qub count{}", (self.len() as f64).log2() as usize);
        (self.len() as f64).log2() as usize
    }

    pub fn measure(&mut self) -> ClassicalRegister {
        assert_eq!(false, self.measured);
        self.measured = true;
    
        let mut cum = 0.0;
        let rand_num: f64 = rand::random();

        for (val, coefficient) in self.prob_amplitudes.amplitudes().iter().enumerate() {
            let scaled_prob = coefficient.norm_sqr();
            cum += scaled_prob;
            // println!("cum: {}", cum);
            // println!("{}", val);

            if rand_num <= cum {
                return ClassicalRegister::from_value(self.len, val as u32);
            }

        }

        ClassicalRegister::from_value(self.len, 0)
    }

    pub fn state(&self)-> Vec<Complex<f64>> {
        self.prob_amplitudes.amplitudes()
    }

    pub fn x(&mut self, target_qubit: usize) {
        assert_eq!(false, self.measured);
        self.prob_amplitudes.pauli_x_gate(target_qubit);
    }

    pub fn y(&mut self, target_qubit: usize) {
        assert_eq!(false, self.measured);
        self.prob_amplitudes.pauli_y_gate(target_qubit);
    }

    pub fn z(&mut self, target_qubit: usize) {
        assert_eq!(false, self.measured);
        self.prob_amplitudes.pauli_z_gate(target_qubit);
    }

    pub fn h(&mut self, target_qubit: usize) {
        assert_eq!(false, self.measured);
        self.prob_amplitudes.hadamard_gate(target_qubit);
    }

    pub fn cnot(&mut self, control_qubit: usize, target_qubit: usize){
        self.prob_amplitudes.cnot_gate(control_qubit, target_qubit);
    }

    pub fn measure_qubit(&mut self, nth_qubit: usize) -> bool {
        let qubit_count = self.get_qubit_count();
        
        // assert!(qubit_count >= nth_qubit);

        let mut cumulative_prob = 0.0;
        let rand_num: f64 = rand::random();

        let num_states = 1 << qubit_count; // 2 ^ qubit_count

        for state_index in 0..num_states {
            // Check if the specified qubit in the current state is |1⟩
            if (state_index >> nth_qubit) & 1 == 1 {
                let amplitude = self.prob_amplitudes.amplitudes()[state_index];
                cumulative_prob += amplitude.norm_sqr();

                if rand_num <= cumulative_prob {
                    self.measured = true;
                    return true; // Return true for |1⟩
                }
            }
        }

        self.measured = true;
        false // Return false for |0⟩ (if no |1⟩ state is found)
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
}