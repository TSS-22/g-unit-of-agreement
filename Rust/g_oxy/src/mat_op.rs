pub fn mean_m1df32(data: &Vec<f32>) -> f32{
    let mut sum_val: f32 = 0f32;
    let mut i: f32 = 0f32;
    for val in data{
        sum_val = sum_val+val;
        i = i+1f32;
    } 
    let mean = sum_val/i;
    return mean;
}

// Function to compute the standard deviation on a 1D matrix
pub fn std_m1df32(data: &Vec<f32>) -> f32{
    let data_mean: f32 = mean_m1df32(&data);
    let mut std_value: f32 = 0f32;
    let mut i: f32 = 0f32;
    for val in data.iter(){
        std_value = std_value + (val-data_mean).powi(2);
        i = i+1f32;
    }
    std_value = (std_value/i).sqrt();
    return std_value;
}

pub fn reorder_m1d2m2d(data_in: Vec<f32>, col_len: usize)-> Vec<Vec<f32>>{
    let mut data_out: Vec<Vec<f32>> = Vec::new();

    for vec_val in data_in.chunks(col_len){
        data_out.push(vec_val.to_vec());
    }
    return data_out;
}