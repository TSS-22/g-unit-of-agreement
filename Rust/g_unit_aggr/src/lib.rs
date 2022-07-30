fn mean_vecf32(data: &Vec<f32>) -> f32{
    let mut sum: f32 = 0f32;

    for val in data{
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

pub fn area2distri(val1: f32, val2: f32, width_distri: f32) -> f32{
    let area: f32 = (0.5+((((val1-val2)/width_distri)*(width_distri*2.0)-width_distri).tanh()/2.0)).round();
    return area;
}

pub fn g_unit_aggr(data: Vec<Vec<f32>>, width_distri: f32) -> Vec<Vec<f32>>{
    // Create the vec g matrix aka results
    let mut g_matrix:  Vec<Vec<f32>> = Vec::new();

    let mut temp_row: Vec<f32> = Vec::new();
    temp_row.reserve(data[0].len());

    let mut temp_g_mat: Vec<f32> = Vec::new();
    temp_g_mat.reserve(data.len());

    // Compute the g unit of each variables
    // Iter through the row aka occurence of range_matrix
    for row_m1 in data.iter(){
        // Iter through the row aka occurence to test against the firt iteration of range_matrix
        for row_m2 in data.iter(){
            // Iter throught the variables of the iter1 and iter2 in order to compute the G unit
            for (x1,x2) in row_m1.iter().zip(row_m2.iter()){
                // Put the arguments in the adequate order, as the function are2distri() is order dependant
                if x1.max(*x2)==*x1{
                    temp_row.push(area2distri(*x1, *x2, width_distri));
                } else {
                    temp_row.push(area2distri(*x2, *x1, width_distri));
                }
            }
            temp_g_mat.push(mean_vecf32(&temp_row));
            temp_row.clear();
        }
        g_matrix.push(temp_g_mat.clone());
        temp_g_mat.clear();
    }
    // Return the results
    return g_matrix;
}