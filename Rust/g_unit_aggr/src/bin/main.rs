use g_unit_aggr::{area2distri, range_values,standard_norm_dist};

fn main(){
    let test_mat: Vec<Vec<f32>> = [[1.0f32,2.0f32].to_vec(),[3.0f32,4.0f32].to_vec()].to_vec();
    let test_mat: Vec<Vec<f32>> = [[1.0f32,2.0f32].to_vec(),[3.0f32,4.0f32].to_vec()].to_vec();
    let a:Vec<f32> = range_values(11.1f32, 3.5f32, 100i16);
    let b:Vec<f32> = range_values(13.1f32, 3.5f32, 100i16);
    let c:Vec<f32> = standard_norm_dist(3.5f32, 100i16);


    area2distri(a,b,c);
}