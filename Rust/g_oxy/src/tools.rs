
pub fn area2distri(diff: f64) -> f64{
    let mut area:f64 = 0f64;
    // To account for the small error as we don't take the full logistic distribution to have a more dynamic behavior 
    let error_margin:f64 = 0.0066928509242848f64;
    if  diff < 1f64{
        area = 1f64-(0.5f64+((5f64*diff-2.5f64).tanh()/2f64)+((error_margin*2f64*diff)-error_margin)); // we transform the logistic cumulative regression to be from [-inf; +inf] to [0;1]  
    }    
    return area as f64;
}

pub fn objective_fun(data: &Vec<f64>)->f64{
    let mut mean:f64 = 0f64;
    let mut variance:f64 = 0f64;
    let mut i:f64 = 0f64;
    for val in data.iter(){
            mean = mean+val;
            i = i+1f64;
    }
    let sum=mean-i;
    mean=mean/i;
    for val in data.iter(){
            variance = variance+(val-mean).powi(2);
    }
    variance=variance/i;
    //let obj_value = ((1.0/variance).tanh())*((1.0/sum).tanh());
    let obj_value = ((sum.tanh())*variance).tanh();
    //println!("{}",obj_value);
    //println!("{}",sum);
    //println!("{}",variance);
    return obj_value; 
}