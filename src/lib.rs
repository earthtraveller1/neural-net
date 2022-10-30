// Everything that I consider neccessary to be put in another file is placed he-
// re.

use std::fs;
use std::ops::Index;

#[derive(Clone)]
pub struct DataLayer {
    elements: Vec<f32>,
}

impl DataLayer {
    pub fn new() -> DataLayer {
        DataLayer { elements: Vec::new() }
    }

    pub fn from_csv_file(file_path: &str) -> Result<DataLayer, std::io::Error> {
        let mut elements = Vec::new();

        let file_contents = fs::read_to_string(file_path)?;
        let line = file_contents.split("\n").collect::<Vec<&str>>()[0];
        line.split(",").for_each(|thing| {
            elements.push(thing.parse::<f32>().unwrap_or_else(|_| 0.0));
        });

        Ok(DataLayer { elements })
    }
}

impl Index<usize> for DataLayer {
    type Output = f32;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
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
    
    // Note that this method is written under the foolish assumption that the  
    // CSV file that was entered into this function is not irregular. In other 
    // words, the CSV file must be (for a lack of a better word) "rectangular" 
    // or weird things might happen (most likely a panic or corrupted layouts) XXXDD.
    pub fn from_csv_file(file_path: &str) -> Result<WeightLayer, std::io::Error> {
        let mut elements = Vec::new();
        let mut input_count = 0;
        let mut output_count = 0;
        
        let file_contents = fs::read_to_string(file_path)?;
        
        file_contents.split("\n").for_each(|line| {
            if input_count == 0 {
                let mut value_count = 0 as usize;
                
                line.split(",").for_each(|value| {
                    value_count += 1;
                    elements.push(value.parse::<f32>().unwrap_or_else(|_| 0.0));
                });
                
                input_count = value_count;
            } else {
                line.split(",").for_each(|value| {
                    elements.push(value.parse::<f32>().unwrap_or_else(|_| 0.0));
                });
            }
            
            output_count += 1;
        });
        
        Ok(WeightLayer {
            elements, input_count, output_count
        })
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

#[cfg(test)]
mod tests;

