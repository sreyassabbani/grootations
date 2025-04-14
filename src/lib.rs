use std::ops;

pub struct Rotor {}

/// A stack-stored representation of an [`N`]-dimensional, [`G`]-grade simple vector (a blade)
pub struct Blade<T: Copy, const G: usize, const N: usize>([[T; N]; G]);
// pub struct Multivector<T: Copy, const G: usize, const N: usize>([Blade<T, G, N>; G]);

pub type Vector<T, const N: usize> = Blade<T, 1, N>;
pub type Bivector<T, const N: usize> = Blade<T, 2, N>;

impl<T: Copy, const N: usize> ops::Index<usize> for Blade<T, 1, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[0][index]
    }
}

impl<T: Copy> Vector<T, 3> {
    pub fn x(self) -> T {
        self[0]
    }
    pub fn y(self) -> T {
        self[1]
    }
    pub fn z(self) -> T {
        self[2]
    }
}

impl<T: Copy, const N: usize> ops::Mul<Bivector<T, N>> for Vector<T, N> {
    type Output = ;
}

#[cfg(test)]
mod tests;
