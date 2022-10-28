use neural_net::{WeightLayer, DataLayer};

struct NeuralNetwork {
    layers: Vec<WeightLayer>,
}

impl NeuralNetwork {
    fn new() -> NeuralNetwork {
        NeuralNetwork { layers: Vec::new() }
    }

    fn from_csv_file(csv_file_path: &str) -> NeuralNetwork {
        // This is placeholder, at least for now.
        NeuralNetwork::new()
    }


    /// Function that basically just runs the neural network and returns the output. This is rather
    /// simple as well.
    fn run(&self, input_layer: &DataLayer) -> DataLayer {
        let mut current_data_layer = input_layer.clone();
        self.layers.iter().for_each(|weight_layer| {
            current_data_layer = weight_layer.pass_data_through(&current_data_layer);
        });

        current_data_layer
    }

    fn add_layer(&mut self, layer: &WeightLayer) {
        self.layers.push(layer.clone());
    }
}

fn main() {
    println!("Hello, world!");
}
