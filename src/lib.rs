#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::ops;

mod blades;
mod multivector;

pub use blades::*;
pub use multivector::*;

/// A single basis element with its coefficient
///
/// The BASIS constant encodes which basis vectors are present using a bitmask:
/// - Bit 0 (LSB) = e₁ present
/// - Bit 1 = e₂ present
/// - Bit 2 = e₃ present
/// - etc.
///
/// Examples in 3D:
/// - BASIS = 0b000 (0) = scalar (no basis vectors)
/// - BASIS = 0b001 (1) = e₁
/// - BASIS = 0b010 (2) = e₂
/// - BASIS = 0b011 (3) = e₁∧e₂ (bivector)
/// - BASIS = 0b111 (7) = e₁∧e₂∧e₃ (trivector)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BasisBlade<T: Copy, const BASIS: usize> {
    pub coefficient: T,
}

impl<T: Copy, const B: usize> BasisBlade<T, B> {
    pub const BASIS: usize = B;
}

impl<T: Copy, const BASIS: usize> BasisBlade<T, BASIS> {
    pub fn new(coefficient: T) -> Self {
        Self { coefficient }
    }

    /// Get the grade (number of basis vectors) of this blade
    pub const fn grade(&self) -> usize {
        BASIS.count_ones() as usize
    }

    /// Get the position of the most significant bit (highest basis index)
    /// Returns the minimum dimension needed to represent this blade
    pub const fn min_dim(&self) -> usize {
        if BASIS == 0 {
            0 // Scalar
        } else {
            // Finds the position of MSB (most significant bit)
            const fn msb_pos(mut n: usize) -> usize {
                if n == 0 {
                    return 0;
                }
                let mut pos = 0;
                while n > 1 {
                    n >>= 1;
                    pos += 1;
                }
                pos + 1
            }
            msb_pos(BASIS)
        }
    }

    /// Get the coefficient
    pub fn coefficient(&self) -> T {
        self.coefficient
    }

    /// Check if this basis blade contains a specific basis vector
    pub const fn contains_basis(index: usize) -> bool {
        (BASIS & (1 << index)) != 0
    }

    /// Get all basis vector indices in this blade
    pub fn basis_indices() -> Vec<usize> {
        (0..usize::BITS as usize) // assuming usize is at most 64 bits
            .filter(|&i| (BASIS & (1 << i)) != 0)
            .collect()
    }
}

/// A stack-stored representation of an N-dimensional, G-grade simple vector (a blade)
pub struct Blade<T: Copy, const G: usize, const N: usize>
where
    [(); binomial(N, G)]: Sized,
{
    coefficients: [T; binomial(N, G)],
}

impl<T: Copy + Default, const N: usize> Default for Vector<T, N>
where
    [(); binomial(N, 1)]: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate binomial coefficient C(n, k) at compile time
pub const fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        0
    } else if k == 0 || k == n {
        1
    } else {
        // Use the recursive formula: C(n,k) = C(n-1,k-1) + C(n-1,k)
        // But we need to implement this iteratively for const fn
        let mut result = 1;
        let mut i = 1;
        while i <= k {
            result = result * (n - i + 1) / i;
            i += 1;
        }
        result
    }
}

pub type Vector<T, const N: usize> = Blade<T, 1, N>;
pub type Bivector<T, const N: usize> = Blade<T, 2, N>;
pub type Trivector<T, const N: usize> = Blade<T, 3, N>;

impl<T: Copy + Default, const N: usize> Vector<T, N>
where
    [(); binomial(N, 1)]: Sized,
{
    pub fn new() -> Self {
        Self {
            coefficients: [T::default(); binomial(N, 1)],
        }
    }
}

impl<T: Copy, const N: usize> ops::Index<usize> for Vector<T, N>
where
    [(); binomial(N, 1)]: Sized,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl<T: Copy, const N: usize> ops::IndexMut<usize> for Vector<T, N>
where
    [(); binomial(N, 1)]: Sized,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coefficients[index]
    }
}

impl<T: Copy> Vector<T, 3>
where
    [(); binomial(3, 1)]: Sized,
{
    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
    pub fn z(&self) -> T {
        self[2]
    }
    pub fn set_x(&mut self, x: T) {
        self[0] = x;
    }
    pub fn set_y(&mut self, y: T) {
        self[1] = y;
    }
    pub fn set_z(&mut self, z: T) {
        self[2] = z;
    }
}

#[cfg(test)]
mod tests;
