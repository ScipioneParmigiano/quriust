use super::super::qubit;
use qubit::*;
use num_complex::Complex;

pub fn superdense_coding(message_to_send: &str) -> String {
    let alice_qubit = Qubit::new(Complex::new(1.0 / f64::sqrt(2.0), 0.0), Complex::new(0.0, 1.0 / f64::sqrt(2.0)));
    let mut alice_qubit_clone = alice_qubit;

    match message_to_send {
        "00" => {},
        "01" => alice_qubit_clone.pauli_x_gate(),
        "10" => alice_qubit_clone.pauli_z_gate(),
        "11" => {
            alice_qubit_clone.pauli_x_gate();
            alice_qubit_clone.pauli_z_gate();
        },
        _ => panic!("Invalid message!"),
    }

    let mut bob_qubit = alice_qubit.clone();
    bob_qubit.cnot(0, 1);

    let mut received_qubit = alice_qubit_clone;
    received_qubit.cnot(0, 1);

    let decoded_message = if received_qubit.measure() { "1" } else { "0" };
    let mut bell_qubit = bob_qubit;
    bell_qubit.measure();
    decoded_message.to_owned() + if bell_qubit.measure() { "1" } else { "0" }
}

#[test]
fn test_superdense_coding() {
    let message_to_send = "11"; // Change this to any valid 2-bit message
    let decoded_message = superdense_coding(message_to_send);
    // assert_eq!(decoded_message, message_to_send);
}
