mod math;

use math::Matrix;
use math::Vector;

struct Layer {
    weights: Matrix,
    biases: Vector,
}

impl Layer {
    fn new(input_count: usize, output_count: usize) -> Layer {
        Layer {
            weights: Matrix::new(output_count, input_count),
            biases: Vector::from_random(output_count),
        }
    }

    fn pass_data_through(&self, data: &Vector) -> Vector {
        self.weights.mul_with_vector(data) + self.biases.clone()
    }
}

fn main() {
    println!("Hello, World!");
}
