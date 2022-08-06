use rayon::prelude::*;
use crate::tools::{area2distri,entropy_info};
use crate::mat_op::{std_m2df32,mean_m1df32,variance_m2df32};

pub fn g_unit_aggr(data: Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    println!("FINDING THAT BITCH!!!");
    let data_norm = z_norm(&data);

    let g_matrix = find_opti_g_matrix(data_norm);
    //let g_matrix = compute_g_matrix(&data_norm,&1f32);

    return g_matrix;
}

fn z_norm(data: &Vec<Vec<f32>>)->Vec<Vec<f32>>{
    let mut data_out: Vec<Vec<f32>> = data.clone();
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
        for row_data in data_out.iter_mut(){
            row_data[i]=(row_data[i]-mean)/std;
        }
    } 
    return data_out;
}

fn compute_g_matrix(data_in: &Vec<Vec<f32>>, load_std: &f32)->Vec<Vec<f32>>{
    let mut temp_row2: Vec<f32> = Vec::new();
    let mut temp_row1: Vec<f32> = Vec::new();
    let mut data_out: Vec<Vec<f32>> = Vec::new();
    
    for row1 in data_in.iter(){
        for row2 in data_in.iter(){
            for (val1, val2) in row1.iter().zip(row2.iter()){
                if val1.max(*val2)==*val1{
                    temp_row2.push(area2distri(*val1, *val2, &load_std));
                } else {
                    temp_row2.push(area2distri(*val2, *val1, &load_std));
                }
            }
            temp_row1.push(mean_m1df32(&temp_row2));
            temp_row2.clear();
        }
        data_out.push(temp_row1.clone());
        temp_row1.clear();
    }

    return data_out;
}

fn find_opti_g_matrix(data_in: Vec<Vec<f32>>)->Vec<Vec<f32>>{
    let mut load_std:f32 =std_m2df32(&data_in);
    let learning_rate:f32 = 0.05f32;
    let mut opti_g_matrix:Vec<Vec<f32>> = Vec::new();
    let mut mat_n = compute_g_matrix(&data_in, &load_std);
    let mut mat_n1 = compute_g_matrix(&data_in, &(load_std+learning_rate));

    let mut entropy_n = variance_m2df32(&mat_n);
    let mut entropy_n1 = variance_m2df32(&mat_n1);

    if entropy_n > entropy_n1{
        mat_n1 = compute_g_matrix(&data_in, &(load_std-learning_rate));
        entropy_n1= variance_m2df32(&mat_n1);
        if entropy_n >= entropy_n1{
            opti_g_matrix = mat_n;
        }else{
            load_std = load_std-learning_rate;
            loop{
                load_std = load_std-learning_rate;
                entropy_n = entropy_n1;
                mat_n = mat_n1;
                mat_n1 = compute_g_matrix(&data_in, &load_std);
                entropy_n1= variance_m2df32(&mat_n1); 
                if entropy_n >= entropy_n1 {
                    opti_g_matrix = mat_n;
                    break;
                }
            }
        }
    }else{
        load_std = load_std+learning_rate;
        loop{
            load_std = load_std+learning_rate;
            entropy_n = entropy_n1;
            mat_n = mat_n1;
            mat_n1 = compute_g_matrix(&data_in, &load_std);
            entropy_n1= variance_m2df32(&mat_n1); 
            if entropy_n >= entropy_n1 {
                opti_g_matrix = mat_n;
                break;
            }
        }
    }
    return opti_g_matrix;
}
