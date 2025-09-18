//! Quantum Computing Simulator using Smooth Cubical Type Theory
//! Where quantum mechanics meets type theory in a dance of possibilities

use sctt_core::{Type, Term, QuantumGate};
use nalgebra::{Complex, DMatrix, DVector};
use std::f64::consts::PI;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A quantum state in the type-theoretic framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    /// Number of qubits
    pub n_qubits: usize,
    
    /// State vector in computational basis
    pub amplitudes: DVector<Complex<f64>>,
    
    /// Type-theoretic representation
    pub type_rep: Type,
    
    /// Entanglement structure
    pub entanglement_map: HashMap<(usize, usize), f64>,
    
    /// Measurement history
    pub measurements: Vec<Measurement>,
    
    /// Coherence level (0 = fully decoherent, 1 = perfectly coherent)
    pub coherence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    pub qubit: usize,
    pub outcome: bool,
    pub probability: f64,
    pub timestamp: f64,
}

impl QuantumState {
    /// Create a new quantum state with n qubits, all initialized to |0⟩
    pub fn new(n_qubits: usize) -> Self {
        let size = 2_usize.pow(n_qubits as u32);
        let mut amplitudes = DVector::zeros(size);
        amplitudes[0] = Complex::new(1.0, 0.0);
        
        QuantumState {
            n_qubits,
            amplitudes,
            type_rep: Type::Quantum(Box::new(Type::Complex)),
            entanglement_map: HashMap::new(),
            measurements: Vec::new(),
            coherence: 1.0,
        }
    }
    
    /// Create a superposition state
    pub fn superposition(n_qubits: usize) -> Self {
        let size = 2_usize.pow(n_qubits as u32);
        let amplitude = 1.0 / (size as f64).sqrt();
        let amplitudes = DVector::from_element(size, Complex::new(amplitude, 0.0));
        
        QuantumState {
            n_qubits,
            amplitudes,
            type_rep: Type::Quantum(Box::new(Type::Complex)),
            entanglement_map: HashMap::new(),
            measurements: Vec::new(),
            coherence: 1.0,
        }
    }
    
    /// Apply a quantum gate to specified qubits
    pub fn apply_gate(&mut self, gate: &QuantumGate, qubits: &[usize]) {
        let matrix = self.gate_to_matrix(gate, qubits.len());
        self.apply_matrix(&matrix, qubits);
        
        // Update entanglement map for multi-qubit gates
        if qubits.len() > 1 {
            for i in 0..qubits.len() {
                for j in i+1..qubits.len() {
                    let entanglement = self.calculate_entanglement(qubits[i], qubits[j]);
                    self.entanglement_map.insert((qubits[i], qubits[j]), entanglement);
                }
            }
        }
    }
    
    /// Convert gate to matrix representation
    fn gate_to_matrix(&self, gate: &QuantumGate, n_qubits: usize) -> DMatrix<Complex<f64>> {
        match gate {
            QuantumGate::Hadamard => {
                let sqrt2 = 2.0_f64.sqrt();
                DMatrix::from_row_slice(2, 2, &[
                    Complex::new(1.0/sqrt2, 0.0), Complex::new(1.0/sqrt2, 0.0),
                    Complex::new(1.0/sqrt2, 0.0), Complex::new(-1.0/sqrt2, 0.0),
                ])
            },
            QuantumGate::PauliX => {
                DMatrix::from_row_slice(2, 2, &[
                    Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
                    Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                ])
            },
            QuantumGate::PauliY => {
                DMatrix::from_row_slice(2, 2, &[
                    Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
                    Complex::new(0.0, 1.0), Complex::new(0.0, 0.0),
                ])
            },
            QuantumGate::PauliZ => {
                DMatrix::from_row_slice(2, 2, &[
                    Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0),
                ])
            },
            QuantumGate::Phase(theta) => {
                DMatrix::from_row_slice(2, 2, &[
                    Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0), Complex::new(theta.cos(), theta.sin()),
                ])
            },
            QuantumGate::CNOT => {
                DMatrix::from_row_slice(4, 4, &[
                    Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                ])
            },
            QuantumGate::Toffoli => {
                let mut matrix = DMatrix::identity(8, 8);
                matrix[(6, 6)] = Complex::new(0.0, 0.0);
                matrix[(6, 7)] = Complex::new(1.0, 0.0);
                matrix[(7, 6)] = Complex::new(1.0, 0.0);
                matrix[(7, 7)] = Complex::new(0.0, 0.0);
                matrix
            },
            QuantumGate::Custom(name) => {
                // Custom gates can be defined here
                match name.as_str() {
                    "QFT" => self.quantum_fourier_transform_matrix(n_qubits),
                    _ => DMatrix::identity(2_usize.pow(n_qubits as u32), 2_usize.pow(n_qubits as u32)),
                }
            },
        }
    }
    
    /// Apply a matrix to specified qubits
    fn apply_matrix(&mut self, matrix: &DMatrix<Complex<f64>>, qubits: &[usize]) {
        let n = self.amplitudes.len();
        let mut new_amplitudes = DVector::zeros(n);
        
        // This is simplified - full implementation would handle arbitrary qubit positions
        if qubits.len() == 1 {
            let qubit = qubits[0];
            for i in 0..n {
                let bit = (i >> qubit) & 1;
                let other_bits = i & !(1 << qubit);
                
                for j in 0..2 {
                    let src_idx = other_bits | (j << qubit);
                    new_amplitudes[i] += matrix[(bit, j)] * self.amplitudes[src_idx];
                }
            }
        } else {
            // Multi-qubit gate - full tensor product needed
            new_amplitudes = matrix * &self.amplitudes;
        }
        
        self.amplitudes = new_amplitudes;
    }
    
    /// Measure a qubit, collapsing the wave function
    pub fn measure(&mut self, qubit: usize) -> bool {
        let prob_one = self.probability_of_one(qubit);
        let outcome = rand::random::<f64>() < prob_one;
        
        // Collapse the state
        self.collapse(qubit, outcome);
        
        // Record measurement
        self.measurements.push(Measurement {
            qubit,
            outcome,
            probability: if outcome { prob_one } else { 1.0 - prob_one },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        });
        
        // Decoherence from measurement
        self.coherence *= 0.95;
        
        outcome
    }
    
    /// Calculate probability of measuring |1⟩ for a qubit
    fn probability_of_one(&self, qubit: usize) -> f64 {
        let mut prob = 0.0;
        for i in 0..self.amplitudes.len() {
            if (i >> qubit) & 1 == 1 {
                prob += self.amplitudes[i].norm_sqr();
            }
        }
        prob
    }
    
    /// Collapse the state after measurement
    fn collapse(&mut self, qubit: usize, outcome: bool) {
        let mut norm = 0.0;
        
        // Zero out incompatible amplitudes and calculate normalization
        for i in 0..self.amplitudes.len() {
            if ((i >> qubit) & 1 == 1) != outcome {
                self.amplitudes[i] = Complex::new(0.0, 0.0);
            } else {
                norm += self.amplitudes[i].norm_sqr();
            }
        }
        
        // Renormalize
        norm = norm.sqrt();
        for i in 0..self.amplitudes.len() {
            self.amplitudes[i] /= norm;
        }
    }
    
    /// Calculate entanglement between two qubits
    fn calculate_entanglement(&self, qubit1: usize, qubit2: usize) -> f64 {
        // Simplified entanglement measure using reduced density matrix
        let reduced_density = self.reduced_density_matrix(&[qubit1, qubit2]);
        let eigenvalues = reduced_density.eigenvalues().unwrap();
        
        // Von Neumann entropy as entanglement measure
        let mut entropy = 0.0;
        for lambda in eigenvalues.iter() {
            let l = lambda.re;
            if l > 1e-10 {
                entropy -= l * l.log2();
            }
        }
        
        entropy / 2.0 // Normalize to [0, 1]
    }
    
    /// Compute reduced density matrix for specified qubits
    fn reduced_density_matrix(&self, qubits: &[usize]) -> DMatrix<Complex<f64>> {
        let size = 2_usize.pow(qubits.len() as u32);
        let mut density = DMatrix::zeros(size, size);
        
        // This is simplified - full implementation would trace out other qubits
        for i in 0..size {
            for j in 0..size {
                density[(i, j)] = self.amplitudes[i] * self.amplitudes[j].conj();
            }
        }
        
        density
    }
    
    /// Quantum Fourier Transform matrix
    fn quantum_fourier_transform_matrix(&self, n_qubits: usize) -> DMatrix<Complex<f64>> {
        let n = 2_usize.pow(n_qubits as u32);
        let mut matrix = DMatrix::zeros(n, n);
        let omega = Complex::new(0.0, 2.0 * PI / n as f64).exp();
        
        for j in 0..n {
            for k in 0..n {
                matrix[(j, k)] = omega.powi((j * k) as i32) / (n as f64).sqrt();
            }
        }
        
        matrix
    }
    
    /// Get the type-theoretic representation of the quantum state
    pub fn to_type(&self) -> Type {
        Type::Quantum(Box::new(Type::Sigma {
            param: "amplitude".to_string(),
            domain: Box::new(Type::Complex),
            codomain: Box::new(Type::Path {
                space: Box::new(Type::Interval),
                start: Box::new(Term::ComplexLit { real: 0.0, imag: 0.0 }),
                end: Box::new(Term::ComplexLit { real: 1.0, imag: 0.0 }),
                curvature: Some(self.coherence),
            }),
        }))
    }
    
    /// Create Bell state (maximally entangled)
    pub fn bell_state(variant: usize) -> Self {
        let mut state = Self::new(2);
        
        match variant {
            0 => {
                // |Φ+⟩ = (|00⟩ + |11⟩) / √2
                state.amplitudes[0] = Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
                state.amplitudes[3] = Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
            },
            1 => {
                // |Φ-⟩ = (|00⟩ - |11⟩) / √2
                state.amplitudes[0] = Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
                state.amplitudes[3] = Complex::new(-1.0 / 2.0_f64.sqrt(), 0.0);
            },
            2 => {
                // |Ψ+⟩ = (|01⟩ + |10⟩) / √2
                state.amplitudes[1] = Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
                state.amplitudes[2] = Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
            },
            _ => {
                // |Ψ-⟩ = (|01⟩ - |10⟩) / √2
                state.amplitudes[1] = Complex::new(1.0 / 2.0_f64.sqrt(), 0.0);
                state.amplitudes[2] = Complex::new(-1.0 / 2.0_f64.sqrt(), 0.0);
            },
        }
        
        state.entanglement_map.insert((0, 1), 1.0);
        state
    }
}

/// Quantum circuit builder with type-safe operations
pub struct QuantumCircuit {
    pub n_qubits: usize,
    pub gates: Vec<(QuantumGate, Vec<usize>)>,
    pub measurements: Vec<usize>,
}

impl QuantumCircuit {
    pub fn new(n_qubits: usize) -> Self {
        QuantumCircuit {
            n_qubits,
            gates: Vec::new(),
            measurements: Vec::new(),
        }
    }
    
    /// Add a gate to the circuit
    pub fn add_gate(&mut self, gate: QuantumGate, qubits: Vec<usize>) -> &mut Self {
        self.gates.push((gate, qubits));
        self
    }
    
    /// Add measurement
    pub fn measure(&mut self, qubit: usize) -> &mut Self {
        self.measurements.push(qubit);
        self
    }
    
    /// Execute the circuit
    pub fn execute(&self, initial_state: Option<QuantumState>) -> (QuantumState, Vec<bool>) {
        let mut state = initial_state.unwrap_or_else(|| QuantumState::new(self.n_qubits));
        
        // Apply all gates
        for (gate, qubits) in &self.gates {
            state.apply_gate(gate, qubits);
        }
        
        // Perform measurements
        let results: Vec<bool> = self.measurements
            .iter()
            .map(|&q| state.measure(q))
            .collect();
        
        (state, results)
    }
    
    /// Build a quantum Fourier transform circuit
    pub fn qft(n_qubits: usize) -> Self {
        let mut circuit = Self::new(n_qubits);
        
        for j in 0..n_qubits {
            circuit.add_gate(QuantumGate::Hadamard, vec![j]);
            
            for k in j+1..n_qubits {
                let angle = PI / 2_f64.powi((k - j) as i32);
                circuit.add_gate(QuantumGate::Phase(angle), vec![j, k]);
            }
        }
        
        // Swap qubits to get correct order
        for i in 0..n_qubits/2 {
            circuit.swap(i, n_qubits - 1 - i);
        }
        
        circuit
    }
    
    /// Swap two qubits
    fn swap(&mut self, q1: usize, q2: usize) {
        self.add_gate(QuantumGate::CNOT, vec![q1, q2]);
        self.add_gate(QuantumGate::CNOT, vec![q2, q1]);
        self.add_gate(QuantumGate::CNOT, vec![q1, q2]);
    }
    
    /// Build Grover's search algorithm circuit
    pub fn grover(n_qubits: usize, marked: usize, iterations: usize) -> Self {
        let mut circuit = Self::new(n_qubits);
        
        // Initial superposition
        for i in 0..n_qubits {
            circuit.add_gate(QuantumGate::Hadamard, vec![i]);
        }
        
        // Grover iterations
        for _ in 0..iterations {
            // Oracle
            circuit.oracle(marked, n_qubits);
            
            // Diffusion operator
            circuit.diffusion(n_qubits);
        }
        
        // Measure all qubits
        for i in 0..n_qubits {
            circuit.measure(i);
        }
        
        circuit
    }
    
    /// Oracle for Grover's algorithm
    fn oracle(&mut self, marked: usize, n_qubits: usize) {
        // Simplified oracle - marks the state |marked⟩
        for i in 0..n_qubits {
            if (marked >> i) & 1 == 0 {
                self.add_gate(QuantumGate::PauliX, vec![i]);
            }
        }
        
        // Multi-controlled Z gate
        if n_qubits > 1 {
            self.add_gate(QuantumGate::Custom("MCZ".to_string()), (0..n_qubits).collect());
        }
        
        for i in 0..n_qubits {
            if (marked >> i) & 1 == 0 {
                self.add_gate(QuantumGate::PauliX, vec![i]);
            }
        }
    }
    
    /// Diffusion operator for Grover's algorithm
    fn diffusion(&mut self, n_qubits: usize) {
        for i in 0..n_qubits {
            self.add_gate(QuantumGate::Hadamard, vec![i]);
            self.add_gate(QuantumGate::PauliX, vec![i]);
        }
        
        // Multi-controlled Z
        if n_qubits > 1 {
            self.add_gate(QuantumGate::Custom("MCZ".to_string()), (0..n_qubits).collect());
        }
        
        for i in 0..n_qubits {
            self.add_gate(QuantumGate::PauliX, vec![i]);
            self.add_gate(QuantumGate::Hadamard, vec![i]);
        }
    }
}

/// Quantum teleportation protocol
pub fn quantum_teleportation(state_to_teleport: &QuantumState) -> (QuantumState, Vec<bool>) {
    // Create Bell pair between Alice and Bob
    let mut system = QuantumState::new(3);
    
    // Initialize qubit 0 with the state to teleport
    system.amplitudes[0] = state_to_teleport.amplitudes[0];
    system.amplitudes[1] = state_to_teleport.amplitudes[1];
    
    // Create Bell pair between qubits 1 and 2
    system.apply_gate(&QuantumGate::Hadamard, &[1]);
    system.apply_gate(&QuantumGate::CNOT, &[1, 2]);
    
    // Alice's operations
    system.apply_gate(&QuantumGate::CNOT, &[0, 1]);
    system.apply_gate(&QuantumGate::Hadamard, &[0]);
    
    // Alice measures her qubits
    let m1 = system.measure(0);
    let m2 = system.measure(1);
    
    // Bob's corrections based on measurement results
    if m2 {
        system.apply_gate(&QuantumGate::PauliX, &[2]);
    }
    if m1 {
        system.apply_gate(&QuantumGate::PauliZ, &[2]);
    }
    
    (system, vec![m1, m2])
}

/// Shor's algorithm for factoring
pub struct ShorsAlgorithm {
    pub n: usize,  // Number to factor
    pub a: usize,  // Random coprime
}

impl ShorsAlgorithm {
    pub fn new(n: usize) -> Self {
        // Find a random coprime to n
        let a = Self::find_coprime(n);
        ShorsAlgorithm { n, a }
    }
    
    fn find_coprime(n: usize) -> usize {
        // Simplified - in practice would use better algorithm
        for a in 2..n {
            if Self::gcd(a, n) == 1 {
                return a;
            }
        }
        2
    }
    
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    
    /// Run the quantum period finding subroutine
    pub fn find_period(&self) -> Option<usize> {
        let n_qubits = (self.n as f64).log2().ceil() as usize;
        let circuit = self.build_circuit(n_qubits);
        
        let (final_state, measurements) = circuit.execute(None);
        
        // Classical post-processing to extract period
        self.extract_period(measurements)
    }
    
    fn build_circuit(&self, n_qubits: usize) -> QuantumCircuit {
        let mut circuit = QuantumCircuit::new(n_qubits * 2);
        
        // Initialize first register in superposition
        for i in 0..n_qubits {
            circuit.add_gate(QuantumGate::Hadamard, vec![i]);
        }
        
        // Modular exponentiation (simplified)
        circuit.add_gate(
            QuantumGate::Custom(format!("ModExp({},{})", self.a, self.n)),
            (0..n_qubits*2).collect()
        );
        
        // Inverse QFT on first register
        let qft = QuantumCircuit::qft(n_qubits);
        for (gate, qubits) in qft.gates.iter().rev() {
            circuit.add_gate(gate.clone(), qubits.clone());
        }
        
        // Measure first register
        for i in 0..n_qubits {
            circuit.measure(i);
        }
        
        circuit
    }
    
    fn extract_period(&self, measurements: Vec<bool>) -> Option<usize> {
        // Convert measurements to number
        let mut measured = 0;
        for (i, &bit) in measurements.iter().enumerate() {
            if bit {
                measured |= 1 << i;
            }
        }
        
        // Use continued fractions to find period
        // Simplified implementation
        if measured > 0 {
            Some((self.n / measured).max(1))
        } else {
            None
        }
    }
    
    /// Factor the number using found period
    pub fn factor(&self) -> Option<(usize, usize)> {
        if let Some(r) = self.find_period() {
            if r % 2 == 0 {
                let x = self.mod_exp(self.a, r/2, self.n);
                let factor1 = Self::gcd(x + 1, self.n);
                let factor2 = Self::gcd(x - 1, self.n);
                
                if factor1 > 1 && factor1 < self.n {
                    return Some((factor1, self.n / factor1));
                }
                if factor2 > 1 && factor2 < self.n {
                    return Some((factor2, self.n / factor2));
                }
            }
        }
        None
    }
    
    fn mod_exp(&self, base: usize, exp: usize, modulus: usize) -> usize {
        let mut result = 1;
        let mut base = base % modulus;
        let mut exp = exp;
        
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            exp >>= 1;
            base = (base * base) % modulus;
        }
        
        result
    }
}

// Random number generation placeholder
mod rand {
    pub fn random<T>() -> T
    where
        T: Default,
    {
        T::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bell_state() {
        let bell = QuantumState::bell_state(0);
        assert_eq!(bell.n_qubits, 2);
        
        // Check entanglement
        let entanglement = bell.entanglement_map.get(&(0, 1)).unwrap();
        assert_eq!(*entanglement, 1.0);
    }
    
    #[test]
    fn test_hadamard_gate() {
        let mut state = QuantumState::new(1);
        state.apply_gate(&QuantumGate::Hadamard, &[0]);
        
        // Should be in equal superposition
        let prob = state.probability_of_one(0);
        assert!((prob - 0.5).abs() < 1e-10);
    }
    
    #[test]
    fn test_quantum_circuit() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.add_gate(QuantumGate::Hadamard, vec![0])
               .add_gate(QuantumGate::CNOT, vec![0, 1]);
        
        let (state, _) = circuit.execute(None);
        
        // Should create Bell state
        assert!(state.entanglement_map.contains_key(&(0, 1)));
    }
}