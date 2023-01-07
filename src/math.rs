//! A bunch of math-related utility functions that are useful for a neural network,
//! such as linear algebra types like vectors and matrices, plus the sigmoid function
//! and it's derivative.

use std::ops::{Add, Index};

struct Vector {
    data: Vec<f64>,
}

impl Vector {
    /// Creates a new Vector with the specified data.
    fn from_vec(data: &Vec<f64>) -> Vector {
        Vector { data: data.clone() }
    }

    /// Creates a new Vector and fills it with random data.
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

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.data[index])
    }
}

struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Creates a new Matrix and fills it with random data.
    fn new(rows: usize, cols: usize) -> Matrix {
        let mut data = Vec::with_capacity(rows * cols);
        for _ in 0..data.len() {
            data.push(rand::random());
        }

        Matrix { data, rows, cols }
    }

    /// Creates a new Matrix from a 2D array. The 2D slice must not have an
    /// irregular shape, or else it will be a runtime error.
    fn from_2d_slice(slice: &[&[f64]]) -> Matrix {
        let rows = slice.len();
        let cols = slice[0].len();

        let mut data = Vec::with_capacity(rows * cols);

        slice
            .iter()
            .for_each(|row| row.iter().for_each(|x| data.push(*x)));

        Matrix { data, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.data[col + row * self.rows]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut (self.data[col + row * self.rows])
    }
}
