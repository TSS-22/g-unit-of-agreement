#[derive(Clone)]
// Colonne of the input data matrix => one of the variable

pub struct ColVar {
    pub data: Vec<f32>,
    pub std: f32,
    pub load_std:f32,
}

impl ColVar{
    pub fn new(data: Vec<f32>, std: f32, load_std: f32) -> Self {
        Self{
            data,
            std,
            load_std,
        }
    }
}

// G matrix type, used for results and before results aggregation
pub struct GMatrix{
    pub data: Vec<Vec<f32>>,
    pub std: Vec<f32>,
    pub load_std: Vec<f32>,
}

impl GMatrix{
    pub fn new(data: Vec<Vec<f32>>, std: Vec<f32>, load_std: Vec<f32>) -> Self {
       Self{
        data,
        std,
        load_std,
       } 
    }
}
