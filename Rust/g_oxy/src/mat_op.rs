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

pub fn std_m2df32(data: &Vec<Vec<f32>>) -> f32{
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

pub fn add_m2df32(mat1: Vec<Vec<f32>>, mat2: Vec<Vec<f32>>)->Vec<Vec<f32>>{
    let mut mat0: Vec<Vec<f32>> = vec![vec![0f32;mat1[0].len()];mat1.len()];
// https://stackoverflow.com/questions/53823252/what-is-a-faster-way-to-perform-element-wise-summation-of-different-length-vecto
    for (row0, (row1,row2)) in mat0.iter_mut().zip(mat1.iter().zip(mat2.iter())){
        for (val0, (val1,val2)) in row0.iter_mut().zip(row1.iter().zip(row2.iter())){
            let a = *val1 as f32;
            let b = *val2 as f32;
            *val0 = a+b;
        }  
    }
    return mat0;
}

pub fn mean_element_wise_m2df32(mat1: Vec<Vec<f32>>, mat2: Vec<Vec<f32>>)->Vec<Vec<f32>>{
    let mut mat0: Vec<Vec<f32>> = vec![vec![0f32;mat1[0].len()];mat1.len()];
// https://stackoverflow.com/questions/53823252/what-is-a-faster-way-to-perform-element-wise-summation-of-different-length-vecto
    for (row0, (row1,row2)) in mat0.iter_mut().zip(mat1.iter().zip(mat2.iter())){
        for (val0, (val1,val2)) in row0.iter_mut().zip(row1.iter().zip(row2.iter())){
            let a = *val1 as f32;
            let b = *val2 as f32;
            *val0 = (a+b)/2f32
        }  
    }
    return mat0;
}

// Function to compute the standard deviation on a 1D matrix
pub fn std_m1df32(data: Vec<f32>) -> f32{
    let mut std_value: f32 = 0f32;
    let mut i: f32 = 0f32;
    for val in data.iter(){
        std_value = std_value + val.powi(2);
        i = i+1f32;
    }
    std_value = std_value.sqrt()/i;
    return std_value;
}

pub fn variance_m2df32(data: &Vec<Vec<f32>>)->f32{
    let mut mean:f32 = 0f32;
    let mut variance:f32 = 0f32;

    for row in data.iter(){
        for val in row.iter(){
            mean = mean+val;
        }
    }
    mean=mean/((data.len()*data[0].len()) as f32);
    for row in data.iter(){
        for val in row.iter(){
            variance = (val-mean).powi(2);
        }
    }
    variance=variance/((data.len()*data[0].len()) as f32);
    return variance; 
}

pub fn sum_md2f32(data_in: &Vec<Vec<f32>>) -> f32{
    let mut sum: f32 = 0f32;

    for row in data_in.iter(){
        for val in row.iter(){
            sum=sum+val;
        }
    }
    return sum;
}