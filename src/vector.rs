use std::ops::{Add, AddAssign, Mul, Sub};

/// A programmatic representation of a mathematical vector. Supports basic ope-
/// rations such as addition, subtraction, and dot product. Does not support o-
/// perations beyond the requirements of this project (duh)
pub struct Vector<T> {
    elements: Vec<T>,
}

impl<T> Vector<T>
where
    T: Clone,
{
    pub fn new() -> Vector<T> {
        Vector {
            elements: Vec::new(),
        }
    }

    pub fn from_vec(vec: &Vec<T>) -> Vector<T> {
        Vector {
            elements: vec.clone(),
        }
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Vec::with_capacity(self.elements.len());
        for i in 0..self.elements.len() {
            result.push(self.elements[i] + rhs.elements[i]);
        }

        Vector { elements: result }
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Vec::with_capacity(self.elements.len());
        for i in 0..self.elements.len() {
            result.push(self.elements[i] - rhs.elements[i]);
        }

        Vector { elements: result }
    }
}

// Technically, there isn't really a default multiplication operation for math-
// ematical vectors, there is a dot product and the cross product. But I doubt
// that I'll be using the cross product, the the multiplication operator defau-
// lts to using dot products, whether you like it or not.
impl<T> Mul for Vector<T>
where
    T: Mul<Output = T> + Copy + From<i32> + AddAssign,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result: T = 0.into();
        for i in 0..self.elements.len() {
            result += self.elements[i] * rhs.elements[i];
        }

        return result;
    }
}
