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
