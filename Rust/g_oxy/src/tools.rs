
pub fn area2distri(diff: f32) -> f32{
    let mut area:f32 = 0f32;
    // To account for the small error as we don't take the full logistic distribution to have a more dynamic behavior 
    let error_margin:f32 = 0.0066929f32;
    if  diff < 1f32{
        area = 1f32-(0.5f32+((5f32*diff-2.5f32).tanh()/2f32)+((error_margin*2f32*diff)-error_margin)); // we transform the logistic cumulative regression to be from [-inf; +inf] to [0;1]  
    }    
    return area as f32;
}

// pub fn objective_fun(data: &Vec<f32>)->f32{
//     let i = data.len() as f32;
//     let sum: f32 = data.par_iter().sum();
//     let mean=sum/i;
//     let mut variance: f32 = data.par_iter().map(|val|{let result = (val-mean).powi(2); return result;}).sum();
//     variance=variance/i;
//     let obj_value = variance*sum;
//     return obj_value; 
// }

pub fn objective_fun(data: &Vec<f32>)->f32{
    let mut mean:f32 = 0f32;
    let mut variance:f32 = 0f32;
    let mut i:f32 = 0f32;
    for val in data.iter(){
            mean = mean+val;
            i = i+1f32;
    }
    let sum=mean;
    mean=mean/i;
    for val in data.iter(){
            variance = variance+(val-mean).powi(2);
    }
    variance=variance/i;
    let obj_value = 2f32.powf(variance)*1f32.powf(sum);
    return obj_value; 
}