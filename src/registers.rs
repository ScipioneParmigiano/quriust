use super::state::State;

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

        for i in 0..width {
            let pos: u32 = (width - i - 1) as u32;
            let value = (2 as u32).pow(pos);

            // Insert a one or a zero at the front of the vector.
            if value <= remaining_value {
                remaining_value -= value;
                bits.insert(0, 1);
            } else {
                bits.insert(0, 0);
            }
        }

        ClassicalRegister::new(bits)
    }

    pub fn value(&self) -> u32 {
        let mut value = 0 as u32;

        for (pos, bit) in self.bits.iter().enumerate() {
            if 0 != *bit {
                value += (2 as u32).pow(pos as u32);
            }
        }

        value
    }

}
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

    pub fn len(&self) -> usize{
        self.len
    }

    pub fn measure(&mut self) -> ClassicalRegister {
        assert_eq!(false, self.measured);
        self.measured = true;
        
        let mut cum = 0.0;
        let rand_num: f64 = rand::random();

        // println!("{rand_num}");
        
        for (val, coefficient) in self.prob_amplitudes.amplitudes().iter().enumerate() {
            cum += coefficient.norm_sqr();
            
            if rand_num < cum {
                return ClassicalRegister::from_value(self.len, val as u32);
            }
        }
        
        ClassicalRegister::from_value(self.len, 0)
    }

    pub fn x(&mut self) {
        assert_eq!(false, self.measured);
        self.prob_amplitudes.pauli_x_gate();
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
    let cr = ClassicalRegister::zeros(3);
    let mut qr: QuantumRegister = QuantumRegister::new(&cr);
    let qr2 = qr.measure();

    assert_eq!(cr, qr2);
    assert!(qr.measured);
}