use std::f32::consts::PI;

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