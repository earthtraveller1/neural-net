//! A bunch of math-related utility functions that are useful for a neural network,
//! such as linear algebra types like vectors and matrices, plus the sigmoid function
//! and it's derivative.

use std::ops::{Add, Index};

pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    /// Creates a new Vector with the specified data.
    pub fn from_vec(data: &Vec<f64>) -> Vector {
        Vector { data: data.clone() }
    }

    /// Creates a new Vector and fills it with random data.
    pub fn from_random(len: usize) -> Vector {
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

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.data[index])
    }
}

pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Creates a new Matrix and fills it with random data.
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let mut data = Vec::with_capacity(rows * cols);
        for _ in 0..data.len() {
            data.push(rand::random());
        }

        Matrix { data, rows, cols }
    }

    /// Creates a new Matrix from a 2D array. The 2D slice must not have an
    /// irregular shape, or else it will be a runtime error.
    pub fn from_2d_slice(slice: &[&[f64]]) -> Matrix {
        let rows = slice.len();
        let cols = slice[0].len();

        let mut data = Vec::with_capacity(rows * cols);

        slice
            .iter()
            .for_each(|row| row.iter().for_each(|x| data.push(*x)));

        Matrix { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[col + row * self.rows]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut (self.data[col + row * self.rows])
    }

    pub fn mul_with_vector(&self, vector: &Vector) -> Vector {
        let mut result = Vec::with_capacity(self.rows);

        for row in 0..self.rows {
            let mut sum = 0.0;

            for col in 0..self.cols {
                sum += self.get(row, col) * vector[col];
            }
        }

        Vector { data: result }
    }
}

pub fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn logistic_derivative(x: f64) -> f64 {
    logistic(x) * (1.0 - logistic(x))
}
