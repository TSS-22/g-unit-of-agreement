use rayon::prelude::*;
use crate::tools::{area2distri};
use crate::mat_op::{std_m2df32,mean_m1df32,variance_m2df32, sum_md2f32};

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
    let mut handles = Vec::new();
    let mut data_out: Vec<Vec<f32>> = Vec::new();
    
    handles = data_in.par_iter().map(|row1|{fn_row2(&data_in,&row1,load_std)}).collect();

    return handles;
}

fn fn_row2(data_in: &Vec<Vec<f32>>, row1: &Vec<f32>, load_std: &f32)->Vec<f32>{
    let mut handles = Vec::new();

    handles = data_in.par_iter().map(|row2|{compute_area(&row1, &row2, &load_std)}).collect();

    return handles;
}

fn compute_area(row1: &Vec<f32>, row2: &Vec<f32>, load_std: &f32)->f32{
    let mut temp_row: Vec<f32> = Vec::new();
    for (val1, val2) in row1.iter().zip(row2.iter()){
        if val1.max(*val2)==*val1{
            temp_row.push(area2distri(*val1, *val2, &load_std));
        } else {
            temp_row.push(area2distri(*val2, *val1, &load_std));
        }
    }
    return mean_m1df32(&temp_row);
}

fn find_opti_g_matrix(data_in: Vec<Vec<f32>>)->Vec<Vec<f32>>{
    let mut load_std:f32 =std_m2df32(&data_in);
    let learning_rate:f32 = 0.05f32;
    let mut opti_g_matrix:Vec<Vec<f32>> = Vec::new();
    let mut mat_n = compute_g_matrix(&data_in, &load_std);
    let mut mat_n1 = compute_g_matrix(&data_in, &(load_std+learning_rate));

    let mut objective_n = variance_m2df32(&mat_n)*sum_md2f32(&mat_n);
    let mut objective_n1 = variance_m2df32(&mat_n1)*sum_md2f32(&mat_n1);

    if objective_n > objective_n1{
        mat_n1 = compute_g_matrix(&data_in, &(load_std-learning_rate));
        objective_n1= variance_m2df32(&mat_n1)*sum_md2f32(&mat_n1);
        if objective_n >= objective_n1{
            opti_g_matrix = mat_n;
        }else{
            load_std = load_std-learning_rate;
            loop{
                load_std = load_std-learning_rate;
                objective_n = objective_n1;
                mat_n = mat_n1;
                mat_n1 = compute_g_matrix(&data_in, &load_std);
                objective_n1= variance_m2df32(&mat_n1)*sum_md2f32(&mat_n1); 
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
            mat_n1 = compute_g_matrix(&data_in, &load_std);
            objective_n1= variance_m2df32(&mat_n1)*sum_md2f32(&mat_n1); 
            if objective_n >= objective_n1 {
                opti_g_matrix = mat_n;
                break;
            }
        }
    }
    return opti_g_matrix;
}
