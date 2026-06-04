# ternary-quantum

Quantum-inspired computing with ternary states — qutrits, ternary Pauli operators, generalized gates, entanglement, and quantum Fourier transforms over Z/3Z.

## Why This Exists

Classical quantum computing frameworks model qubits — two-level quantum systems with states {|0⟩, |1⟩}. But quantum theory naturally supports higher-dimensional systems (qudits), and the three-level case — **qutrits** — is especially elegant because the state space C³ maps naturally to ternary logic {-1, 0, +1}. Qutrits carry more information per particle, produce richer entanglement structures, and have been demonstrated experimentally in trapped ions and photonic systems.

This crate provides a complete simulation framework for qutrit-based quantum computing: complex arithmetic, 3×3 unitary matrices, generalized X/Z/H gates, qutrit entanglement detection, ternary Bell states, and quantum Fourier transforms. All pure Rust, no external dependencies.

This crate is part of the **Negative Space Intelligence** ecosystem.

## Core Concepts

- **Complex** — Complex number type (real + imaginary) with arithmetic operations, conjugation, and magnitude.
- **Matrix3** — A 3×3 complex matrix representing qutrit gates. Supports multiplication, application to state vectors, and tensor (Kronecker) products.
- **Qutrit** — A quantum trit with a 3-element state vector in C³. Supports gate application, probability measurement, and collapse.
- **TwoQutrit** — A two-qutrit system with a 9-dimensional state vector. Supports joint gates and entanglement detection.
- **Gates** — Generalized X (cyclic shift), Z (phase), and H (Hadamard/QFT) gates for qutrits, plus CNOT (SUM gate) for two qutrits.
- **Bell States** — Three maximally entangled two-qutrit states, the ternary analog of Bell pairs.
- **QFT** — Quantum Fourier Transform over Z/3Z for one and two qutrits.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-quantum = "0.1"
```

```rust
use ternary_quantum::*;

// Create a qutrit in basis state |1⟩
let mut q = Qutrit::new(1);
assert!((q.probability(1) - 1.0).abs() < 1e-9);

// Apply Hadamard gate → uniform superposition
q.apply(&h_gate());
for k in 0..3 {
    assert!((q.probability(k) - 1.0/3.0).abs() < 1e-9);
}

// Apply X gate: |0⟩ → |1⟩ → |2⟩ → |0⟩ (cyclic)
let mut q0 = Qutrit::new(0);
q0.apply(&x_gate());  // now |1⟩
q0.apply(&x_gate());  // now |2⟩
q0.apply(&x_gate());  // back to |0⟩

// Create a ternary Bell state and check entanglement
let bell = bell_state_0(); // (1/√3)(|00⟩ + |11⟩ + |22⟩)
assert!(is_entangled(&bell));

// Product states are not entangled
let prod = TwoQutrit::from_qutrits(&Qutrit::new(0), &Qutrit::new(1));
assert!(!is_entangled(&prod));

// Create entanglement via H + CNOT
let mut q = Qutrit::new(0);
q.apply(&h_gate()); // superposition
let mut two = TwoQutrit::from_qutrits(&q, &Qutrit::new(0));
two.apply(&cnot_gate()); // entangle
assert!(is_entangled(&two));

// Quantum Fourier Transform
let qft = qft_two(); // 9×9 matrix for two-qutrit QFT
```

## API Overview

### State Types
| Type | Description |
|---|---|
| `Complex` | Complex number with arithmetic ops |
| `Matrix3` | 3×3 complex matrix (single-qutrit gate) |
| `Qutrit` | Single qutrit with state vector in C³ |
| `TwoQutrit` | Two-qutrit system with 9D state vector |

### Gates
| Function | Description |
|---|---|
| `x_gate()` | Generalized X: cyclic shift \|0⟩→\|1⟩→\|2⟩→\|0⟩ |
| `x2_gate()` | X²: shift by 2 |
| `z_gate()` | Generalized Z: phase gate with ω = e^(2πi/3) |
| `h_gate()` | Generalized Hadamard (single-qutrit QFT) |
| `pauli_x()` / `pauli_y()` / `pauli_z()` | Ternary Pauli operators |
| `cnot_gate()` | Two-qutrit SUM gate: \|a,b⟩ → \|a, (a+b) mod 3⟩ |
| `qft_single()` | QFT over Z/3Z for one qutrit |
| `qft_two()` | QFT over Z/3Z for two qutrits |

### Entanglement & Bell States
| Function | Description |
|---|---|
| `bell_state_0()` | \|Φ₀⟩ = (1/√3)(\|00⟩ + \|11⟩ + \|22⟩) |
| `bell_state_1()` | \|Φ₁⟩ = (1/√3)(\|01⟩ + \|12⟩ + \|20⟩) |
| `bell_state_2()` | \|Φ₂⟩ = (1/√3)(\|02⟩ + \|10⟩ + \|21⟩) |
| `is_entangled()` | Checks 2×2 minors of amplitude matrix for rank > 1 |

## How It Works

Qutrits live in a 3-dimensional Hilbert space C³. The generalized X gate performs a cyclic permutation of basis states, while the Z gate applies the cube root of unity ω = e^(2πi/3) as a phase factor. X³ = I (identity) and Z³ = I, reflecting the threefold symmetry. The Hadamard gate creates uniform superposition with equal-magnitude amplitudes, just as in the qubit case.

Entanglement detection uses the rank test: a two-qutrit state is entangled if and only if the 3×3 matrix of amplitudes Aᵢⱼ = ⟨ij|ψ⟩ has rank > 1. The implementation checks all 2×2 minors — if any minor has a non-zero determinant, the state is entangled. This is exact for two qutrits (no approximation).

The ternary CNOT (SUM gate) adds the control value to the target modulo 3, generalizing the binary CNOT's XOR. When combined with a Hadamard on the control qutrit, it produces genuine ternary entanglement — a superposition of all nine basis states with non-trivial correlations.

## Use Cases

1. **Quantum computing research** — Simulate qutrit-based quantum algorithms that exploit the larger Hilbert space for more compact encodings or richer gate decompositions.

2. **Quantum information education** — Teach quantum mechanics with a three-level system that naturally extends qubit concepts while introducing phase factors (cube roots of unity) and ternary entanglement.

3. **Ternary error correction** — Explore quantum error-correcting codes based on qutrits, which have different and sometimes more favorable error models than qubit codes.

4. **Cross-domain ternary modeling** — Use qutrit superposition as a model for ternary uncertainty in classical systems, bridging `ternary-logic` (definite values) with `ternary-bayesian` (probabilistic reasoning).

## Ecosystem

| Crate | Relationship |
|---|---|
| `ternary-hardware` | Trit/tryte primitives that qutrits generalize to quantum domain |
| `ternary-logic` | Classical ternary logic — the "measurement" side of qutrit states |
| `ternary-attention` | Uses similar matrix operations for classical attention mechanisms |
| `ternary-bayesian` | Probabilistic ternary reasoning — classical analog of quantum amplitudes |

## Known Limitations

- **`measure()` is deterministic, not random.** The implementation uses a hardcoded `roll = 0.5` instead of a random number generator. Measurement always collapses the same way for a given state, making this unsuitable for quantum simulation requiring genuine stochastic collapse.
- **No unitarity check on gates.** `Matrix3` does not verify that user-constructed gates are unitary. Non-physical gates can be created and applied without error.
- **Maximum two qutrits.** There is no generalized n-qutrit system. All multi-qutrit operations are limited to `TwoQutrit` (9-dimensional state space).
- **9×9 matrices have limited interface.** The `tensor` method produces `[[Complex; 9]; 9]` but there is no `Matrix9` type — you must use `TwoQutrit::apply` directly.
- **No noise model or decoherence.** The simulation is noiseless; qutrit states remain perfectly coherent indefinitely.

## License

MIT
