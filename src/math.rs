//! A bunch of math-related utility functions that are useful for a neural network,
//! such as linear algebra types like vectors and matrices, plus the sigmoid function
//! and it's derivative.

use std::ops::{Add, Mul};

struct Vector {
    data: Vec<f64>,
}

impl Vector {
    fn from_vec(data: &Vec<f64>) -> Vector {
        Vector { data: data.clone() }
    }

    fn from_random(len: usize) -> Vector {
        let mut data = Vec::with_capacity(len);

        for _ in 0..len {
            data.push(rand::random());
        }

        Vector { data }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = Vec::with_capacity(self.data.len());

        for i in 0..data.len() {
            data[i] = self.data[i] + rhs.data[i]
        }

        Vector { data }
    }
}
