<div align="center">
  <img src="https://img.shields.io/crates/d/quriust.svg" alt="Downloads">
   <img src="https://img.shields.io/crates/v/quriust.svg" alt="Version">
</div>

# Quriust
Just a quantum computing simulator in Rust for curious ones, built to run and learn some simple algorithms. 

## Installation

Add this library as a dependency in your `Cargo.toml`:

```toml
[dependencies]
quriust = "0.2.0"
```

## Usage
Here's a basic example demonstrating how to use this library:

```rust
use quriust::registers::{ClassicalRegister, QuantumRegister};

// Create a new quantum register 
let classical_register: ClassicalRegister = ClassicalRegister::from_value(4, 1);
let mut register = QuantumRegister::new(&classical_register);
 
// Apply a Hadamard gate to the first qubit
register.h(1);
 
// Measure the register
let measurement = register.measure();
 
// Print the measurement outcome
println!("Measurement outcome: {:?}", measurement);
```

## Documentation
For detailed usage instructions and API documentation, see the [documentation](https://docs.rs/quriust/latest/quriust/index.html).

## Contributing
Quriust welcomes contributions from the community to enhance its features, improve performance, and fix bugs. If you're interested in contributing, feel free to submit pull requests with your improvements.

## License
This library is licensed under the MIT License. See the [LICENSE](https://github.com/ScipioneParmigiano/quriust/blob/main/LICENSE) file for details.
