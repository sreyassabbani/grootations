#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod blades;

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

#[cfg(test)]
mod tests;
