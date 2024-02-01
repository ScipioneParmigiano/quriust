//! # Quantum Computer Simulation Library
//!
//! This library provides a framework for simulating quantum computers using classical computers.
//!
//! ## Modules
//!
//! - `algorithms`: Contains implementations of various quantum algorithms.
//! - `registers`: Defines data structures for quantum registers.
//! - `state`: Implements the quantum state and operations on it.
//!
//! ## Example
//!
//! ```rust
//! use quriust::registers::{ClassicalRegister, QuantumRegister};
//! // Create a new quantum register 
//! let classical_register: ClassicalRegister = ClassicalRegister::from_value(4, 1);
//! let mut register = QuantumRegister::new(&classical_register);
//! 
//! // Apply a Hadamard gate to the first qubit
//! register.h(1);
//! 
//! // Measure the register
//! let measurement = register.measure();
//! 
//! // Print the measurement outcome
//! println!("Measurement outcome: {:?}", measurement);
//! ```

pub mod algorithms;
pub mod registers;
pub mod state;
pub mod quantum_computer;