#![forbid(unsafe_code)]

//! Quantum-inspired computing with ternary states (qutrits).
//!
//! Provides qutrit representation, ternary Pauli operators, qutrit gates
//! (X, Z, H generalized), ternary entanglement, quantum Fourier transform
//! over Z/3Z, and ternary Bell states. All simulation-based.

/// A complex number (real and imaginary parts).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    pub fn real(r: f64) -> Self {
        Complex { re: r, im: 0.0 }
    }

    pub fn zero() -> Self {
        Complex { re: 0.0, im: 0.0 }
    }

    pub fn one() -> Self {
        Complex { re: 1.0, im: 0.0 }
    }

    pub fn norm_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn magnitude(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn conj(&self) -> Self {
        Complex { re: self.re, im: -self.im }
    }

    pub fn scale(&self, s: f64) -> Self {
        Complex { re: self.re * s, im: self.im * s }
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex { re: self.re + other.re, im: self.im + other.im }
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex { re: self.re - other.re, im: self.im - other.im }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

/// A 3x3 complex matrix (used for qutrit gates).
#[derive(Clone, Debug)]
pub struct Matrix3 {
    pub data: [[Complex; 3]; 3],
}

impl Matrix3 {
    pub fn identity() -> Self {
        let mut m = Matrix3 { data: [[Complex::zero(); 3]; 3] };
        for i in 0..3 {
            m.data[i][i] = Complex::one();
        }
        m
    }

    pub fn zero() -> Self {
        Matrix3 { data: [[Complex::zero(); 3]; 3] }
    }

    /// Multiply this matrix by another.
    pub fn mul(&self, other: &Matrix3) -> Matrix3 {
        let mut result = Matrix3::zero();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result.data[i][j] = result.data[i][j] + self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    /// Apply matrix to a 3-element state vector.
    pub fn apply(&self, state: &[Complex; 3]) -> [Complex; 3] {
        let mut result = [Complex::zero(); 3];
        for i in 0..3 {
            for j in 0..3 {
                result[i] = result[i] + self.data[i][j] * state[j];
            }
        }
        result
    }

    /// Tensor (Kronecker) product of two 3x3 matrices -> 9x9 (represented as flat array).
    pub fn tensor(&self, other: &Matrix3) -> [[Complex; 9]; 9] {
        let mut result = [[Complex::zero(); 9]; 9];
        for i1 in 0..3 {
            for j1 in 0..3 {
                for i2 in 0..3 {
                    for j2 in 0..3 {
                        let ri = i1 * 3 + i2;
                        let ci = j1 * 3 + j2;
                        result[ri][ci] = self.data[i1][j1] * other.data[i2][j2];
                    }
                }
            }
        }
        result
    }
}

/// Generalized X gate for qutrits (cyclic shift).
/// X|0⟩ = |1⟩, X|1⟩ = |2⟩, X|2⟩ = |0⟩
pub fn x_gate() -> Matrix3 {
    // X|0⟩=|1⟩, X|1⟩=|2⟩, X|2⟩=|0⟩
    // In matrix form: row i is the output for |i⟩
    // data[row][col] = <row|X|col>
    Matrix3 {
        data: [
            [Complex::zero(), Complex::zero(), Complex::one()],
            [Complex::one(), Complex::zero(), Complex::zero()],
            [Complex::zero(), Complex::one(), Complex::zero()],
        ],
    }
}

/// Generalized X^2 gate (shift by 2).
pub fn x2_gate() -> Matrix3 {
    let x = x_gate();
    x.mul(&x)
}

/// Generalized Z gate for qutrits (phase gate).
/// Z|k⟩ = ω^k |k⟩ where ω = e^(2πi/3)
pub fn z_gate() -> Matrix3 {
    let omega = Complex::new(-0.5, (3.0_f64).sqrt() / 2.0); // e^(2πi/3)
    let omega2 = omega * omega;
    Matrix3 {
        data: [
            [Complex::one(), Complex::zero(), Complex::zero()],
            [Complex::zero(), omega, Complex::zero()],
            [Complex::zero(), Complex::zero(), omega2],
        ],
    }
}

/// Generalized Hadamard gate for qutrits (QFT over Z/3Z for single qutrit).
pub fn h_gate() -> Matrix3 {
    let omega = Complex::new(-0.5, (3.0_f64).sqrt() / 2.0);
    let omega2 = omega * omega;
    let inv_sqrt3 = Complex::real(1.0 / 3.0_f64.sqrt());
    Matrix3 {
        data: [
            [inv_sqrt3, inv_sqrt3, inv_sqrt3],
            [inv_sqrt3, inv_sqrt3 * omega, inv_sqrt3 * omega2],
            [inv_sqrt3, inv_sqrt3 * omega2, inv_sqrt3 * omega],
        ],
    }
}

/// A qutrit (quantum trit) with state vector in C^3.
#[derive(Clone, Debug)]
pub struct Qutrit {
    pub state: [Complex; 3],
}

impl Qutrit {
    /// Create a qutrit in computational basis state |k⟩.
    pub fn new(k: u8) -> Self {
        assert!(k <= 2);
        let mut state = [Complex::zero(); 3];
        state[k as usize] = Complex::one();
        Qutrit { state }
    }

    /// Create a qutrit in uniform superposition.
    pub fn uniform() -> Self {
        let amp = Complex::real(1.0 / 3.0_f64.sqrt());
        Qutrit { state: [amp, amp, amp] }
    }

    /// Apply a gate to this qutrit.
    pub fn apply(&mut self, gate: &Matrix3) {
        self.state = gate.apply(&self.state);
    }

    /// Probability of measuring |k⟩.
    pub fn probability(&self, k: u8) -> f64 {
        assert!(k <= 2);
        self.state[k as usize].norm_sq()
    }

    /// Total probability (should be ~1.0).
    pub fn total_probability(&self) -> f64 {
        self.state.iter().map(|c| c.norm_sq()).sum()
    }

    /// Measure the qutrit (collapse to basis state).
    /// Returns the measured state and collapses the state.
    pub fn measure(&mut self) -> u8 {
        let r = self.probability(0);
        let roll = 0.5; // deterministic "measurement" for testing
        if roll < r {
            self.state = [Complex::one(), Complex::zero(), Complex::zero()];
            0
        } else if roll < r + self.probability(1) {
            self.state = [Complex::zero(), Complex::one(), Complex::zero()];
            1
        } else {
            self.state = [Complex::zero(), Complex::zero(), Complex::one()];
            2
        }
    }
}

/// Two-qutrit state (9-dimensional state vector).
#[derive(Clone, Debug)]
pub struct TwoQutrit {
    pub state: [Complex; 9],
}

impl TwoQutrit {
    /// Create from two qutrits via tensor product.
    pub fn from_qutrits(a: &Qutrit, b: &Qutrit) -> Self {
        let mut state = [Complex::zero(); 9];
        for i in 0..3 {
            for j in 0..3 {
                state[i * 3 + j] = a.state[i] * b.state[j];
            }
        }
        TwoQutrit { state }
    }

    /// Total probability.
    pub fn total_probability(&self) -> f64 {
        self.state.iter().map(|c| c.norm_sq()).sum()
    }

    /// Apply a 9x9 gate.
    pub fn apply(&mut self, gate: &[[Complex; 9]; 9]) {
        let mut new_state = [Complex::zero(); 9];
        for i in 0..9 {
            for j in 0..9 {
                new_state[i] = new_state[i] + gate[i][j] * self.state[j];
            }
        }
        self.state = new_state;
    }

    /// Probability of measuring state |i,j⟩ = |i*3+j⟩.
    pub fn probability(&self, idx: usize) -> f64 {
        assert!(idx < 9);
        self.state[idx].norm_sq()
    }
}

/// Ternary Pauli X operator (same as x_gate).
pub fn pauli_x() -> Matrix3 {
    x_gate()
}

/// Ternary Pauli Z operator (same as z_gate).
pub fn pauli_z() -> Matrix3 {
    z_gate()
}

/// Ternary Pauli Y = X * Z (up to phase).
pub fn pauli_y() -> Matrix3 {
    let x = x_gate();
    let z = z_gate();
    x.mul(&z)
}

/// Quantum Fourier Transform over Z/3Z for a single qutrit (same as H gate).
pub fn qft_single() -> Matrix3 {
    h_gate()
}

/// Quantum Fourier Transform over Z/3Z for two qutrits.
pub fn qft_two() -> [[Complex; 9]; 9] {
    let _h = h_gate();
    let omega = Complex::new(-0.5, (3.0_f64).sqrt() / 2.0);
    let inv3 = Complex::real(1.0 / 3.0);

    let mut result = [[Complex::zero(); 9]; 9];
    for j1 in 0..3 {
        for j2 in 0..3 {
            for k1 in 0..3 {
                for k2 in 0..3 {
                    let row = j1 * 3 + j2;
                    let col = k1 * 3 + k2;
                    // Phase = ω^(j1*k1 + j2*k2)
                    let exp = j1 * k1 + j2 * k2;
                    let phase = match exp % 3 {
                        0 => Complex::one(),
                        1 => omega,
                        _ => omega * omega,
                    };
                    result[row][col] = inv3 * phase;
                }
            }
        }
    }
    result
}

/// Create a ternary Bell state |Φ₀⟩ = (1/√3)(|00⟩ + |11⟩ + |22⟩).
pub fn bell_state_0() -> TwoQutrit {
    let amp = Complex::real(1.0 / 3.0_f64.sqrt());
    let mut state = [Complex::zero(); 9];
    state[0] = amp; // |00⟩
    state[4] = amp; // |11⟩
    state[8] = amp; // |22⟩
    TwoQutrit { state }
}

/// Create a ternary Bell state |Φ₁⟩ = (1/√3)(|01⟩ + |12⟩ + |20⟩).
pub fn bell_state_1() -> TwoQutrit {
    let amp = Complex::real(1.0 / 3.0_f64.sqrt());
    let mut state = [Complex::zero(); 9];
    state[1] = amp; // |01⟩
    state[5] = amp; // |12⟩
    state[6] = amp; // |20⟩
    TwoQutrit { state }
}

/// Create a ternary Bell state |Φ₂⟩ = (1/√3)(|02⟩ + |10⟩ + |21⟩).
pub fn bell_state_2() -> TwoQutrit {
    let amp = Complex::real(1.0 / 3.0_f64.sqrt());
    let mut state = [Complex::zero(); 9];
    state[2] = amp; // |02⟩
    state[3] = amp; // |10⟩
    state[7] = amp; // |21⟩
    TwoQutrit { state }
}

/// CNOT gate for qutrits (SUM gate): |a,b⟩ → |a, (a+b) mod 3⟩.
pub fn cnot_gate() -> [[Complex; 9]; 9] {
    let mut gate = [[Complex::zero(); 9]; 9];
    for a in 0..3u8 {
        for b in 0..3u8 {
            let row = ((a + b) % 3) as usize * 3 + a as usize; // target row: |a, (a+b)%3⟩
            let col = b as usize * 3 + a as usize;              // source col: |a, b⟩
            // Actually: CNOT maps |a,b⟩ → |a, (a+b) mod 3⟩
            // So gate[row][col] = 1 where row = a*3 + (a+b)%3, col = a*3 + b
            let input_idx = a as usize * 3 + b as usize;
            let output_b = ((a + b) % 3) as usize;
            let output_idx = a as usize * 3 + output_b;
            gate[output_idx][input_idx] = Complex::one();
        }
    }
    gate
}

/// Check if two qutrits are entangled by checking if the state is a product state.
/// A state |ψ⟩ = Σ aᵢⱼ |i⟩|j⟩ is a product state if the 3x3 matrix of amplitudes has rank 1.
/// We approximate by checking if any 2x2 minor is non-zero.
pub fn is_entangled(state: &TwoQutrit) -> bool {
    // Build 3x3 matrix of amplitudes
    let mut mat = [[Complex::zero(); 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            mat[i][j] = state.state[i * 3 + j];
        }
    }
    // Check all 2x2 minors
    for r1 in 0..3 {
        for r2 in (r1 + 1)..3 {
            for c1 in 0..3 {
                for c2 in (c1 + 1)..3 {
                    let det = mat[r1][c1] * mat[r2][c2] - mat[r1][c2] * mat[r2][c1];
                    if det.norm_sq() > 1e-10 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_complex_mul() {
        let a = Complex::new(1.0, 1.0);
        let b = Complex::new(1.0, -1.0);
        let c = a * b;
        assert!(approx_eq(c.re, 2.0));
        assert!(approx_eq(c.im, 0.0));
    }

    #[test]
    fn test_complex_norm() {
        let c = Complex::new(3.0, 4.0);
        assert!(approx_eq(c.norm_sq(), 25.0));
        assert!(approx_eq(c.magnitude(), 5.0));
    }

    #[test]
    fn test_matrix_identity() {
        let id = Matrix3::identity();
        let state = [Complex::one(), Complex::zero(), Complex::zero()];
        let result = id.apply(&state);
        assert!(approx_eq(result[0].norm_sq(), 1.0));
    }

    #[test]
    fn test_x_gate() {
        let x = x_gate();
        let mut state = [Complex::one(), Complex::zero(), Complex::zero()];
        state = x.apply(&state);
        assert!(approx_eq(state[1].norm_sq(), 1.0)); // |0⟩ → |1⟩
        state = x.apply(&state);
        assert!(approx_eq(state[2].norm_sq(), 1.0)); // |1⟩ → |2⟩
        state = x.apply(&state);
        assert!(approx_eq(state[0].norm_sq(), 1.0)); // |2⟩ → |0⟩
    }

    #[test]
    fn test_z_gate_phase() {
        let z = z_gate();
        let state = [Complex::zero(), Complex::one(), Complex::zero()];
        let result = z.apply(&state);
        // Z|1⟩ = ω|1⟩
        assert!(approx_eq(result[1].norm_sq(), 1.0));
        assert!(result[1].re < 0.0); // ω has negative real part
    }

    #[test]
    fn test_qutrit_basis() {
        let q = Qutrit::new(0);
        assert!(approx_eq(q.probability(0), 1.0));
        assert!(approx_eq(q.probability(1), 0.0));
        assert!(approx_eq(q.total_probability(), 1.0));
    }

    #[test]
    fn test_qutrit_uniform() {
        let q = Qutrit::uniform();
        assert!(approx_eq(q.probability(0), 1.0 / 3.0));
        assert!(approx_eq(q.total_probability(), 1.0));
    }

    #[test]
    fn test_qutrit_apply_x() {
        let mut q = Qutrit::new(0);
        q.apply(&x_gate());
        assert!(approx_eq(q.probability(1), 1.0));
    }

    #[test]
    fn test_qutrit_apply_h() {
        let mut q = Qutrit::new(0);
        q.apply(&h_gate());
        assert!(approx_eq(q.total_probability(), 1.0));
        // All probabilities should be 1/3
        for k in 0..3u8 {
            assert!(approx_eq(q.probability(k), 1.0 / 3.0));
        }
    }

    #[test]
    fn test_two_qutrit_product() {
        let a = Qutrit::new(0);
        let b = Qutrit::new(1);
        let ab = TwoQutrit::from_qutrits(&a, &b);
        assert!(approx_eq(ab.total_probability(), 1.0));
        assert!(approx_eq(ab.probability(1), 1.0)); // |01⟩ = index 1
    }

    #[test]
    fn test_bell_state_0() {
        let bell = bell_state_0();
        assert!(approx_eq(bell.total_probability(), 1.0));
        assert!(approx_eq(bell.probability(0), 1.0 / 3.0)); // |00⟩
        assert!(approx_eq(bell.probability(4), 1.0 / 3.0)); // |11⟩
        assert!(approx_eq(bell.probability(8), 1.0 / 3.0)); // |22⟩
    }

    #[test]
    fn test_bell_state_entangled() {
        let bell = bell_state_0();
        assert!(is_entangled(&bell));
    }

    #[test]
    fn test_product_state_not_entangled() {
        let a = Qutrit::new(0);
        let b = Qutrit::new(1);
        let ab = TwoQutrit::from_qutrits(&a, &b);
        assert!(!is_entangled(&ab));
    }

    #[test]
    fn test_cnot_gate() {
        let mut state = TwoQutrit::from_qutrits(&Qutrit::new(0), &Qutrit::new(1));
        state.apply(&cnot_gate());
        // |0,1⟩ → |0, (0+1)%3⟩ = |0,1⟩
        assert!(approx_eq(state.probability(1), 1.0));
    }

    #[test]
    fn test_cnot_gate_entangle() {
        let mut q = Qutrit::new(0);
        q.apply(&h_gate()); // superposition
        let mut two = TwoQutrit::from_qutrits(&q, &Qutrit::new(0));
        two.apply(&cnot_gate());
        assert!(is_entangled(&two));
    }

    #[test]
    fn test_pauli_x_squared() {
        let x = pauli_x();
        let x2 = x.mul(&x);
        let x3 = x2.mul(&x);
        // X^3 = I
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { Complex::one() } else { Complex::zero() };
                assert!(approx_eq((x3.data[i][j] - expected).norm_sq(), 0.0));
            }
        }
    }

    #[test]
    fn test_qft_preserves_probability() {
        let mut q = Qutrit::new(1);
        q.apply(&qft_single());
        assert!(approx_eq(q.total_probability(), 1.0));
    }

    #[test]
    fn test_qft_two_preserves_probability() {
        let two = TwoQutrit::from_qutrits(&Qutrit::new(0), &Qutrit::new(1));
        let qft = qft_two();
        let mut state = [Complex::zero(); 9];
        for i in 0..9 {
            for j in 0..9 {
                state[i] = state[i] + qft[i][j] * two.state[j];
            }
        }
        let total: f64 = state.iter().map(|c| c.norm_sq()).sum();
        assert!(approx_eq(total, 1.0));
    }

    #[test]
    fn test_x2_gate() {
        let x2 = x2_gate();
        let state = [Complex::one(), Complex::zero(), Complex::zero()];
        let result = x2.apply(&state);
        assert!(approx_eq(result[2].norm_sq(), 1.0)); // |0⟩ → |2⟩
    }

    #[test]
    fn test_bell_state_1() {
        let bell = bell_state_1();
        assert!(approx_eq(bell.total_probability(), 1.0));
        assert!(approx_eq(bell.probability(1), 1.0 / 3.0));
    }
}
