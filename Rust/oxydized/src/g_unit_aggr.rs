use crate::tools::{entropy_info,area2distri};
use crate::g_types::{GMatrix, ColVar};
use crate::mat_op::{add_m2df32,std_m1df32};
use rayon::prelude::*;

// Function to compute the g matrix
pub fn g_unit_aggr(data: Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    // Find the widthof the std of the data
    // Get the mean first
    let mut handles = Vec::new();
    let mut variable_vector: Vec<ColVar> = Vec::new();
    let mut temp_var_vec: Vec<f32> = Vec::new();
    let mut g_matrix:  Vec<Vec<f32>> = Vec::new();

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
    //aggregate_g_matrix();
    // Return the results
    return g_matrix;
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
