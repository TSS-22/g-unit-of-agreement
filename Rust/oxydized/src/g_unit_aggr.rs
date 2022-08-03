use rayon::prelude::*;

fn entropy_info(matrix: Vec<Vec<f32>>) -> f32{
    let mut entropy_values: f32 = 0.0;

    for mat_row in matrix {
        for mat_val in mat_row{
            entropy_values = entropy_values+(mat_val.exp2()*(mat_val.exp2()+0.1f32).ln());
        }
    }
    return entropy_values;
}

fn area2distri(val1: f32, val2: f32, width_distri: f32) -> f32{
    let mut area:f32 = 0f32;
    if  (val1-val2)/width_distri < 1f32{
        area = (((1f32-(val1-val2)/width_distri).powf(3.2))*1f32.exp())/((1f32-(val1-val2)/width_distri).powf(3.25).exp());
    }
    return area as f32;
}


fn aggregate_g_matrix(mat: Vec<GMatrix>) -> GMatrix{
    let mut temp_g_matrix: Vec<Vec<f32>> = vec![vec![0f32;mat[0].data[0].len()];mat[0].data.len()];
    let mut temp_row: Vec<f32> = Vec::new();
    let mut vec_std: Vec<f32> = Vec::new();
    let mut vec_load_std:Vec<f32> = Vec::new();

    for val1 in mat.iter(){
        vec_std.push(val1.std[0]);
        vec_load_std.push(val1.load_std[0]);
        temp_g_matrix = add_m2df32(temp_g_matrix,val1.data.clone());
    } 

    let end_g_matrix: GMatrix = GMatrix::new(temp_g_matrix,
        vec_std,
        vec_load_std);

    return end_g_matrix;
}

fn create_gmatrix(data: &ColVar) -> GMatrix {
    let mut g_matrix: GMatrix = GMatrix::new(Vec::new(), Vec::new(), Vec::new());
    let mut load_std:f32  = data.load_std;
    let mut temp_row: Vec<f32> = Vec::new();
    let mut temp_g_matrix: Vec<Vec<f32>> = Vec::new();
    let learning_bias: f32 = 0.05f32;
    // Compute the matrix for the first time, from ColVar data
    for x1 in data.data.iter(){
        for x2 in data.data.iter(){
            // Put the arguments in the adequate order, as the function are2distri() is order dependant
            if x1.max(*x2)==*x1{
                temp_row.push(area2distri(*x1, *x2, data.std*load_std));
            } else {
                temp_row.push(area2distri(*x2, *x1, data.std*load_std));
            }
        }
        temp_g_matrix.push(temp_row.clone());
        // Clear might not be needed as they all make the same length
        temp_row.clear();
    }
    // Find the side of the gradient descent
    let mut ntrp_n: f32 = entropy_info(temp_g_matrix);
    let mut ntrp_n1: f32 = entropy_info(compute_gmatrix(temp_g_matrix, data.std*(load_std+learning_bias)));

    if ntrp_n > ntrp_n1{
        ntrp_n1= entropy_info(compute_gmatrix(temp_g_matrix, data.std*(load_std-learning_bias)));
        if ntrp_n > ntrp_n1{
            g_matrix.data = temp_g_matrix;
        }else{
            load_std = load_std-learning_bias;
            loop{
                load_std = load_std-learning_bias;
                ntrp_n = ntrp_n1;
                temp_g_matrix = compute_gmatrix(temp_g_matrix, data.std*load_std);
                ntrp_n1= entropy_info(temp_g_matrix); 
                if ntrp_n >= ntrp_n1 {
                    g_matrix.data = temp_g_matrix;
                }
            }
        }
    }else{
        load_std = load_std+learning_bias;
        loop{
            load_std = load_std+learning_bias;
            ntrp_n = ntrp_n1;
            temp_g_matrix = compute_gmatrix(temp_g_matrix, data.std*load_std);
            ntrp_n1= entropy_info(temp_g_matrix); 
            if ntrp_n >= ntrp_n1 {
                g_matrix.data = temp_g_matrix;
            }
        }
    }

    
    g_matrix.std = vec![data.std];
    g_matrix.load_std = vec![load_std];

    return g_matrix;
}

fn compute_gmatrix(data: Vec<Vec<f32>>, load_std: f32) -> Vec<Vec<f32>>{
    let mut g_matrix: Vec<Vec<f32>> = Vec::new();
    return g_matrix;
}

// Function to compute the g matrix
pub fn g_unit_aggr(data: Vec<Vec<f32>>) -> GMatrix{
    // Find the widthof the std of the data
    // Get the mean first
    let mut handles = Vec::new();
    let mut variable_vector: Vec<ColVar> = Vec::new();
    let mut temp_var_vec: Vec<f32> = Vec::new();
    let mut g_matrix:  GMatrix = GMatrix::new(Vec::new(),Vec::new(),Vec::new());

    for i in 0..data[0].len(){
        for row_data in data.iter(){
            temp_var_vec.push(row_data[i].clone());
        }
        variable_vector.push(ColVar::new(temp_var_vec.clone(), std_m1df32(temp_var_vec.clone()), 1f32));
        // Shouldn't be needed
        temp_var_vec.clear();
    }

    // Compute the GMatrix for every variables, via gradient descent
    handles = variable_vector.par_iter().map(|x| {let result: GMatrix = create_gmatrix(x);}).collect();
    
    // Return the results
    return g_matrix;
}

// DEPRECATED
// pub fn g_unit_aggr(data: Vec<Vec<f32>>, distri_factor: f32) -> Vec<Vec<f32>>{
//     // Find the widthof the std of the data
//     // Get the mean first
//     let width_distri = std_m2df32(data.clone())*distri_factor;
//     // Create the vec g matrix aka results
//     let mut g_matrix:  Vec<Vec<f32>> = Vec::new();

//     let mut temp_row: Vec<f32> = Vec::new();
//     temp_row.reserve(data[0].len());

//     let mut temp_g_mat: Vec<f32> = Vec::new();
//     temp_g_mat.reserve(data.len());

//     // Compute the g unit of each variables
//     // Iter through the row aka occurence of range_matrix
//     for row_m1 in data.iter(){
//         // Iter through the row aka occurence to test against the firt iteration of range_matrix
//         for row_m2 in data.iter(){
//             // Iter throught the variables of the iter1 and iter2 in order to compute the G unit
//             for (x1,x2) in row_m1.iter().zip(row_m2.iter()){
//                 // Put the arguments in the adequate order, as the function are2distri() is order dependant
//                 if x1.max(*x2)==*x1{
//                     temp_row.push(area2distri(*x1, *x2, width_distri));
//                 } else {
//                     temp_row.push(area2distri(*x2, *x1, width_distri));
//                 }
//             }
//             temp_g_mat.push(mean_m1df32(&temp_row));
//             temp_row.clear();
//         }
//         g_matrix.push(temp_g_mat.clone());
//         temp_g_mat.clear();
//     }
//     // Return the results
//     return g_matrix;
// }








/////////////////////
/// MATH MODULE BECAUSE FUCK YOU RUST

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

fn add_m2df32(mat1: Vec<Vec<f32>>, mat2: Vec<Vec<f32>>)->Vec<Vec<f32>>{
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

// Function to compute the standard deviation sur une matrice 1D
fn std_m1df32(data: Vec<f32>) -> f32{
    let mut std_value: f32 = 0f32;
    let mut i: f32 = 0f32;
    for val in data.iter(){
        std_value = std_value + val.powi(2);
        i = i+1f32;
    }
    std_value = std_value.sqrt()/i;
    return std_value;
}


////////////////////////////////////////////////////////////////
/// TYPE MODULE BECAUSE FUCK YOU RUST

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
