use crate::BasisBlade;

use std::ops;

/// A multivector in N dimensions with up to grade G
pub struct Multivector<T: Copy, const N: usize, const G: usize>
where
    [(); 1 << N]: Sized,
{
    /// Coefficients for all 2^N possible basis blades
    coefficients: [T; 1 << N],
}

impl<T: Copy + Default, const N: usize, const G: usize> Default for Multivector<T, N, G>
where
    [(); 1 << N]: Sized,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Copy + Default, const N: usize, const G: usize> Multivector<T, N, G>
where
    [(); 1 << N]: Sized,
{
    pub fn new() -> Self {
        Self {
            coefficients: [T::default(); 1 << N],
        }
    }

    pub fn from_scalar(scalar: T) -> Self {
        let mut mv = Self::new();
        mv.coefficients[0] = scalar;
        mv
    }

    /// Get coefficient for a specific basis blade
    pub fn get_coefficient<const BASIS: usize>(&self) -> T {
        self.coefficients[BASIS]
    }

    /// Set coefficient for a specific basis blade
    pub fn set_coefficient<const BASIS: usize>(&mut self, coeff: T) {
        self.coefficients[BASIS] = coeff;
    }

    /// Add a basis blade to this multivector
    pub fn add_blade<const BASIS: usize>(&mut self, blade: BasisBlade<T, BASIS>)
    where
        T: ops::AddAssign,
    {
        self.coefficients[BASIS] += blade.coefficient;
    }

    /// Get all non-zero basis blades
    pub fn non_zero_blades(&self) -> Vec<(usize, T)>
    where
        T: PartialEq,
    {
        self.coefficients
            .iter()
            .enumerate()
            .filter(|(_, coeff)| **coeff != T::default())
            .map(|(basis, coeff)| (basis, *coeff))
            .collect()
    }

    /// Get the maximum grade present in this multivector
    pub fn max_grade(&self) -> usize
    where
        T: PartialEq,
    {
        self.coefficients
            .iter()
            .enumerate()
            .filter(|(_, coeff)| **coeff != T::default())
            .map(|(basis, _)| basis.count_ones() as usize)
            .max()
            .unwrap_or(0)
    }

    /// Check if this multivector is homogeneous (single grade)
    pub fn is_homogeneous(&self) -> bool
    where
        T: PartialEq,
    {
        let grades: std::collections::HashSet<usize> = self
            .coefficients
            .iter()
            .enumerate()
            .filter(|(_, coeff)| **coeff != T::default())
            .map(|(basis, _)| basis.count_ones() as usize)
            .collect();
        grades.len() <= 1
    }

    /// Extract the scalar part
    pub fn scalar(&self) -> T {
        self.coefficients[0]
    }

    /// Extract grade k part
    pub fn grade(&self, k: usize) -> Self {
        let mut result = Self::new();
        for (basis, coeff) in self.coefficients.iter().enumerate() {
            if basis.count_ones() as usize == k {
                result.coefficients[basis] = *coeff;
            }
        }
        result
    }
}

// Implement basic arithmetic operations for multivectors
impl<T, const N: usize, const G: usize> ops::Add for Multivector<T, N, G>
where
    T: Copy + ops::Add<Output = T>,
    [(); 1 << N]: Sized,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..(1 << N) {
            result.coefficients[i] = result.coefficients[i] + rhs.coefficients[i];
        }
        result
    }
}

impl<T, const N: usize, const G: usize> ops::Sub for Multivector<T, N, G>
where
    T: Copy + ops::Sub<Output = T>,
    [(); 1 << N]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..(1 << N) {
            result.coefficients[i] = result.coefficients[i] - rhs.coefficients[i];
        }
        result
    }
}
