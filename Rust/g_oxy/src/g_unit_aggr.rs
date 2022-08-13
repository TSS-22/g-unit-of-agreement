use rayon::prelude::*;
use crate::tools::{area2distri, objective_fun};
use crate::mat_op::{std_m1df64,mean_m1df64,reorder_m1d2m2d};

// REPLACE SUM AND VAR FUNCTION BY OBJECTIVE FUNCTION

pub fn g_unit_aggr(data: Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    println!("Computing the G units");
    let row_len: usize = data.len();
    let col_len: usize = data[0].len();
    // let data_norm = z_norm(&data);
    // let std_vec = compute_std_col(&data_norm);
    let std_vec = compute_std_col(&data);
    let data_norm_flat = diff_flat(data);
    let g_matrix = find_opti_g_matrix(data_norm_flat, std_vec, col_len);
    let g_matrix = reorder_m1d2m2d(g_matrix, row_len);
    return g_matrix;
    //let data_norm = reorder_m1d2m2d(data_norm_flat, col_len);
    //return data_norm;
}

fn compute_std_col(data_in: &Vec<Vec<f64>>)->Vec<f64>{
    let mut std_vec: Vec<f64>=Vec::new();
    for i in 0..data_in[0].len(){
        let mut col_diff_val: Vec<f64> = Vec::new();

        for row1 in data_in.iter(){
            for row2 in data_in.iter(){
                col_diff_val.push((row1[i]-row2[i]).abs());
            }
        }
        let mut mean:f64 = 0f64;
        let mut std:f64 = 0f64;
        for val in col_diff_val.iter(){
             mean = mean+val;
        }
        mean = mean/data_in.len() as f64;
        for val in col_diff_val.iter(){
             std=std+(val-mean).powi(2);
        }
        std_vec.push((std/data_in.len() as f64).sqrt());
    }
    return std_vec;
}


fn z_norm(data_in: &Vec<Vec<f64>>)->Vec<Vec<f64>>{
    let mut data = data_in.clone();
    for i in 0..data[0].len(){
        let mut mean:f64 = 0f64;
        let mut std:f64 = 0f64;
        let mut max_val:f64=1f64;

        for row_data in data.iter(){
            mean = mean+row_data[i];
            max_val = row_data[i].max(max_val);
        }
        mean = mean/data.len() as f64;

        for row_data in data.iter(){
            std=std+(row_data[i]-mean).powi(2);
        }
        std = (std/data.len() as f64).sqrt();
        for row_data in data.iter_mut(){
            row_data[i]=mean/row_data[i];
        }
    } 
    return data;
}


fn diff_flat(data_in: Vec<Vec<f64>>)->Vec<f64>{
    let mut data_out: Vec<f64> = Vec::new();
    for row_1 in data_in.iter(){
        for row_2 in data_in.iter(){
            for (val1,val2) in row_1.iter().zip(row_2.iter()){
                data_out.push((val1-val2).abs());
            }
        }
    }
    return data_out;
}

fn find_opti_g_matrix(data_in: Vec<f64>, std_vec: Vec<f64>, chunk_size: usize)->Vec<f64>{
    //let mut load_std:f64 =std_m1df64(&data_in);
    let mut load_std:f64 = 1f64;
    let learning_rate:f64 = 0.05f64;
    let mut opti_g_matrix:Vec<f64> = Vec::new();
    let mut mat_n = compute_g_matrix(&data_in, &load_std, &chunk_size, &std_vec);
    return mat_n;
    // let mut mat_n1 = compute_g_matrix(&data_in, &(load_std+learning_rate), &chunk_size, &std_vec);
    // let mut objective_n = objective_fun(&mat_n);
    // let mut objective_n1 = objective_fun(&mat_n1);
    // if objective_n < objective_n1{
    //     mat_n1 = compute_g_matrix(&data_in, &(load_std-learning_rate), &chunk_size, &std_vec);
    //     objective_n1 = objective_fun(&mat_n1);
    //     if objective_n <= objective_n1{
    //         opti_g_matrix = mat_n;
    //     }else{
    //         load_std = load_std-learning_rate;
    //         loop{
    //             load_std = load_std-learning_rate;
    //             objective_n = objective_n1;
    //             mat_n = mat_n1;
    //             mat_n1 = compute_g_matrix(&data_in, &load_std, &chunk_size, &std_vec);
    //             objective_n1 = objective_fun(&mat_n1);
    //             if objective_n <= objective_n1 {
    //                 opti_g_matrix = mat_n;
    //                 break;
    //             }
    //         }
    //     }
    // }else{
    //     load_std = load_std+learning_rate;
    //     loop{
    //         load_std = load_std+learning_rate;
    //         objective_n = objective_n1;
    //         mat_n = mat_n1;
    //         mat_n1 = compute_g_matrix(&data_in, &load_std, &chunk_size, &std_vec);
    //         objective_n1= objective_fun(&mat_n1);
    //         if objective_n <= objective_n1 {
    //             opti_g_matrix = mat_n;
    //             break;
    //         }
    //     }
    // }
    // return opti_g_matrix;
}

fn compute_g_matrix(data_in: &Vec<f64>, load_std: &f64, chunk_size: &usize, std_vec: &Vec<f64>)->Vec<f64>{
    let mut handles = Vec::new();
    handles = data_in.par_chunks(*chunk_size).map(|diff_x_y|{compute_area(diff_x_y,load_std,std_vec)}).collect();
    return handles;
}

fn compute_area(diff_x_y: &[f64], load_std: &f64, std_vec:&Vec<f64>)->f64{
    let mut temp_row: Vec<f64> = Vec::new();
    for (val,std) in diff_x_y.iter().zip(std_vec.iter()){
        temp_row.push(area2distri((val.powf(3.0))/(load_std*std)));
    }
    return mean_m1df64(&temp_row);
}


