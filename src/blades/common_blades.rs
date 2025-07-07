use crate::BasisBlade;

// Scalars (grade 0)
pub type Scalar<T> = BasisBlade<T, 0>;

// Vectors (grade 1) in various dimensions
pub type E1<T> = BasisBlade<T, 0b001>; // e₁
pub type E2<T> = BasisBlade<T, 0b010>; // e₂  
pub type E3<T> = BasisBlade<T, 0b100>; // e₃
pub type E4<T> = BasisBlade<T, 0b1000>; // e₄

// Bivectors (grade 2) in 3D
pub type E12<T> = BasisBlade<T, 0b011>; // e₁∧e₂
pub type E13<T> = BasisBlade<T, 0b101>; // e₁∧e₃
pub type E23<T> = BasisBlade<T, 0b110>; // e₂∧e₃

// Bivectors in 4D (additional)
pub type E14<T> = BasisBlade<T, 0b1001>; // e₁∧e₄
pub type E24<T> = BasisBlade<T, 0b1010>; // e₂∧e₄
pub type E34<T> = BasisBlade<T, 0b1100>; // e₃∧e₄

// Trivectors (grade 3) in 3D and 4D
pub type E123<T> = BasisBlade<T, 0b111>; // e₁∧e₂∧e₃
pub type E124<T> = BasisBlade<T, 0b1011>; // e₁∧e₂∧e₄
pub type E134<T> = BasisBlade<T, 0b1101>; // e₁∧e₃∧e₄
pub type E234<T> = BasisBlade<T, 0b1110>; // e₂∧e₃∧e₄

// Quadvector (grade 4) in 4D
pub type E1234<T> = BasisBlade<T, 0b1111>; // e₁∧e₂∧e₃∧e₄
