// Everything that I consider neccessary to be put in another file is placed he-
// re.

#[derive(Clone)]
pub struct DataLayer {
    elements: Vec<f32>,
}

impl DataLayer {
    pub fn new() -> DataLayer {
        DataLayer { elements: Vec::new() }
    }
}

#[derive(Clone)]
pub struct WeightLayer {
    elements: Vec<f32>,
    input_count: usize,
    output_count: usize,
}

impl WeightLayer {
    pub fn new(input_count: usize, output_count: usize) -> WeightLayer {
        WeightLayer {
            elements: vec![0.0;input_count * output_count],
            input_count,
            output_count
        }
    }

    pub fn from_vec(elements: Vec<f32>, output_count: usize) -> WeightLayer {
        WeightLayer {
            elements: elements.clone(),
            input_count: elements.len() / output_count,
            output_count,
        }
    }

    pub fn pass_data_through(&self, input_data: &DataLayer) -> DataLayer {
        let mut output = Vec::with_capacity(self.output_count);

        {
            let mut i = 0;
            while i < self.output_count {
                let mut output_value = 0.0f32;

                {
                    let mut j = 0;
                    while j < self.input_count {
                        output_value += input_data.elements[j] * self.elements[(i * self.input_count) + j];
                        j += 1;
                    }
                }

                output.push(output_value);
                i += 1;
            }
        }

        DataLayer { elements: output }
    }
}

