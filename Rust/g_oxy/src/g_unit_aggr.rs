use rayon::prelude::*;
use crate::tools::{area2distri, objective_fun};
use crate::mat_op::{std_m1df32,mean_m1df32,reorder_m1d2m2d};

// REPLACE SUM AND VAR FUNCTION BY OBJECTIVE FUNCTION

pub fn g_unit_aggr(data: Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    println!("Let's go!!!");
    let col_len: usize = data[0].len();
    let data_norm = z_norm(&data);
    let data_norm_flat = diff_flat(data_norm);
    let g_matrix = find_opti_g_matrix(data_norm_flat, col_len);
    let g_matrix = reorder_m1d2m2d(g_matrix, col_len);
    return g_matrix;
}

fn z_norm(data_in: &Vec<Vec<f32>>)->Vec<Vec<f32>>{
    let mut data = data_in.clone();
    for i in 0..data[0].len(){
        let mut mean:f32 = 0f32;
        let mut micro:f32 = 0f32;
        let mut std:f32 = 0f32;
        // Find the mean and start to compute 
        for row_data in data.iter(){
            micro = micro+row_data[i];
            mean = mean+row_data[i];
        }
        mean=mean/data.len() as f32;
        micro = micro/data.len() as f32;
        // Compute std
        for row_data in data.iter(){
            std=(row_data[i]-micro).powi(2);
        }
        std = (std/data.len() as f32).sqrt();
        // Z norm
        for row_data in data.iter_mut(){
            row_data[i]=(row_data[i]-mean)/std;
        }
    } 
    return data;
}

fn diff_flat(data_in: Vec<Vec<f32>>)->Vec<f32>{
    let mut data_out: Vec<f32> = Vec::new();
    for row_1 in data_in.iter(){
        for row_2 in data_in.iter(){
            for (val1,val2) in row_1.iter().zip(row_2.iter()){
                data_out.push(val1-val2);
            }
        }
    }
    return data_out;
}

fn find_opti_g_matrix(data_in: Vec<f32>, chunk_size: usize)->Vec<f32>{
    let mut load_std:f32 =std_m1df32(&data_in);
    let learning_rate:f32 = 0.05f32;
    let mut opti_g_matrix:Vec<f32> = Vec::new();
    let mut mat_n = compute_g_matrix(&data_in, &load_std, &chunk_size);
    let mut mat_n1 = compute_g_matrix(&data_in, &(load_std+learning_rate), &chunk_size);

    let mut objective_n = objective_fun(&mat_n1);
    let mut objective_n1 = objective_fun(&mat_n1);

    if objective_n > objective_n1{
        mat_n1 = compute_g_matrix(&data_in, &(load_std-learning_rate), &chunk_size);
        objective_n1= objective_fun(&mat_n1);
        if objective_n >= objective_n1{
            opti_g_matrix = mat_n;
        }else{
            load_std = load_std-learning_rate;
            loop{
                load_std = load_std-learning_rate;
                objective_n = objective_n1;
                mat_n = mat_n1;
                mat_n1 = compute_g_matrix(&data_in, &load_std, &chunk_size);
                objective_n1= objective_fun(&mat_n1);
                if objective_n >= objective_n1 {
                    opti_g_matrix = mat_n;
                    break;
                }
            }
        }
    }else{
        load_std = load_std+learning_rate;
        loop{
            load_std = load_std+learning_rate;
            objective_n = objective_n1;
            mat_n = mat_n1;
            mat_n1 = compute_g_matrix(&data_in, &load_std, &chunk_size);
            objective_n1= objective_fun(&mat_n1);
            if objective_n >= objective_n1 {
                opti_g_matrix = mat_n;
                break;
            }
        }
    }
    return opti_g_matrix;
}

fn compute_g_matrix(data_in: &Vec<f32>, load_std: &f32, chunk_size: &usize)->Vec<f32>{
    let mut handles = Vec::new();
    handles = data_in.par_chunks(*chunk_size).map(|diff_x_y|{compute_area(diff_x_y,load_std)}).collect();
    return handles;
}

fn compute_area(diff_x_y: &[f32], load_std: &f32)->f32{
    let mut temp_row: Vec<f32> = Vec::new();
    for val in diff_x_y.iter(){
        temp_row.push(area2distri(val.abs()/load_std));
    }
    return mean_m1df32(&temp_row);
}


