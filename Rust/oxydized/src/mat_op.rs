// mod mat_ops;

pub fn mean_m1df32(data: &Vec<f32>) -> f32{
    let mut sum: f32 = 0f32;

    for val in data{
        sum = sum+val;
    } 
    let mean = sum/(data.len() as f32);
    return mean;
}

pub fn mean_m2df32(data: Vec<Vec<f32>>) -> f32{
    let mut vec_mean: Vec<f32> = Vec::new();
    for row in data{
        vec_mean.push(mean_m1df32(&row));
    }
    let data_mean: f32 = mean_m1df32(&vec_mean);
    return data_mean;
}

pub fn std_m2df32(data: Vec<Vec<f32>>) -> f32{
    let data_mean = mean_m2df32(data.clone());
    let mut vec_std: Vec<f32> = Vec::new();
    for row in data.clone().iter(){
        for val in row.iter(){
            vec_std.push((val-data_mean).powi(2));
        }
    }
    let std: f32 = mean_m1df32(&vec_std).sqrt();
    return std;
}
