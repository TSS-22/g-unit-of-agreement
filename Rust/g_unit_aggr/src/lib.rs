use std::f32::consts::PI;

fn mean_vecf32(data: Vec<f32>) -> f32{
    let mut sum: f32 = 0f32;

    for val in &data{
        sum = sum+val;
    } 
    let mean = sum/(data.len() as f32);
    return mean;
}

pub fn entropy_info(matrix: Vec<Vec<f32>>) -> f32{
    let mut entropy_values: f32 = 0.0;

    for mat_row in matrix {
        for mat_val in mat_row{
            entropy_values = entropy_values+(mat_val.exp2()*(mat_val.exp2()+0.1f32).ln());
        }
    }
    return entropy_values;
}

pub fn standard_norm_dist(width_distri: f32, resolution: i16) -> Vec<f32>{

    let step: f32 = width_distri * 2.0f32 / resolution as f32;

    let mut density_func: Vec<f32> = Vec::new();
    let mut n_val = -width_distri;

    loop{
        density_func.push(((-0.5f32*n_val.exp2()).exp())/((2.0f32*PI).sqrt()));

        n_val = n_val+step;

        if n_val>width_distri{
            break;
        }
    }
    return density_func;
}

pub fn range_values(x: f32, width_distri: f32, resolution: i16) -> Vec<f32>{

    let step: f32 = width_distri * 2.0f32 / resolution as f32;

    let mut x_range: Vec<f32> = Vec::new();
    let mut n_val = x-width_distri;

    loop{
        x_range.push(n_val);

        n_val = n_val+step;

        if n_val>x+width_distri{
            break;
        }
    }
    return x_range;
}

pub fn area2distri(val1: Vec<f32>, val2: Vec<f32>, std_distri: Vec<f32>) -> f32{

    let mut area: f32 = 0.0;

    for (i,x) in val1.iter().enumerate(){
        if x>=&val2[0]{
            for (x1,x2) in val1[i..].iter().zip(val1[i..].iter().rev()){
                match x1<=x2{
                    true  => {area = area+x1;}
                    false => {area = area+x2;}
                }
            }
        }
    }
    return area;
}

pub fn g_unit_aggr(data: Vec<Vec<f32>>, width_distri: f32, resolution: i16) -> Vec<Vec<f32>>{
    // Create normaldistribution density function
    let std_distri = standard_norm_dist(width_distri, resolution);

    // Create the vec g matrix aka results
    let mut g_matrix:  Vec<Vec<f32>> = Vec::new();
    // Create a vec to store the range value to compute only once
    let mut range_matrix:  Vec<Vec<Vec<f32>>> = Vec::new();
    
    // Compute the range for all variables measurements
    for row_data in data {
        // Create/Refresh buffer vec
        let mut temp_range: Vec<Vec<f32>> = Vec::new();
        for val in row_data{
            // Fill buffer vec with row value
            temp_range.push(range_values(val, width_distri, resolution));
        }
        // Add the row
        range_matrix.push(temp_range);
    }
    let mut i=0;
    // Compute the g unit of each variables
    // Iter through the row aka occurence of range_matrix
    for row_m1 in range_matrix.iter(){
        // Create/Renew buffer vec
        let mut temp_g_mat: Vec<f32> = Vec::new();
        // Iter through the row aka occurence to test against the firt iteration of range_matrix
        for row_m2 in range_matrix.iter(){
            // Create/Renew buffer vec
            let mut temp_range: Vec<f32> = Vec::new();
            // Iter throught the variables of the iter1 and iter2 in order to compute the G unit
            for (x1,x2) in row_m1.iter().zip(row_m2.iter()){
                // Put the arguments in the adequate order, as the function are2distri() is order dependant
                if x1[0].max(x2[0])==x1[0]{
                    println!("{}", i);
                    temp_range.push(area2distri(x1.to_vec(), x2.to_vec(), std_distri.clone()));
                } else {
                    println!("{}", i);
                    temp_range.push(area2distri(x2.to_vec(), x1.to_vec(), std_distri.clone()));
                }
                i=i+1;
            }
            temp_g_mat.push(mean_vecf32(temp_range));
        }
        g_matrix.push(temp_g_mat);
    }
    // Return the results
    return g_matrix;
}



    
