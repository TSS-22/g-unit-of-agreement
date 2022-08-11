pub fn entropy_info(matrix: &Vec<Vec<f32>>) -> f32{
    let mut entropy_values: f32 = 0.0;
    for mat_row in matrix {
        for mat_val in mat_row{
            entropy_values = entropy_values+(mat_val.exp2()*(mat_val.exp2()+0.1f32).ln());
        }
    }
    return entropy_values;
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

pub fn sum_md2f32(data_in: &Vec<Vec<f32>>) -> f32{
    let mut sum: f32 = 0f32;

    for row in data_in.iter(){
        for val in row.iter(){
            sum=sum+val;
        }
    }
    return sum;
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

pub fn sum_md1f32(data_in: &Vec<f32>) -> f32{
    let mut sum: f32 = 0f32;

    for val in data_in.iter(){
            sum=sum+val;
    }
    return sum;
}

pub fn variance_m1df32(data: &Vec<f32>)->f32{
    let mut mean:f32 = 0f32;
    let mut variance:f32 = 0f32;
    let mut i:f32 = 0f32;
    for val in data.iter(){
            mean = mean+val;
            i = i+1f32;
    }
    mean=mean/i;
    for val in data.iter(){
            variance = (val-mean).powi(2);
    }
    variance=variance/i;
    return variance; 
}