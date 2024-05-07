
use std::ops::Sub;
use std::sync::atomic::AtomicBool;

pub fn ext_translation(mat: &glam::Mat4) -> (f32,f32,f32) {
    (mat.x_axis.w, mat.y_axis.w, mat.z_axis.w)
}


pub fn map(value: f32, rg1: (f32, f32), rg2: (f32, f32)) -> f32 {
    let rg1unit = rg1.1 - rg1.0; // eq 1 unit
    let rg2unit = rg2.1 - rg2.0; // eq 1 unit

    let scaledif = rg1unit / rg2unit;

    let value_inunits = value / rg1unit; //value in rg1 units

    value_inunits * scaledif
}

pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
    let mut tmp = 0f32;
    if input < min {
        return min;
    }
    if input > max {
        return max;
    }
    input
}

pub fn subv3f(a: (f32,f32,f32), b: (f32,f32,f32)) -> (f32,f32,f32){
    (a.0-b.0,a.1-b.1,a.2-b.2)
}

pub fn addv3f(a: (f32,f32,f32), b: (f32,f32,f32)) -> (f32,f32,f32){
    (a.0+b.0,a.1+b.1,a.2+b.2)
}

pub fn mulv3f(a: (f32,f32,f32), b: (f32,f32,f32)) -> (f32,f32,f32){
    (a.0*b.0,a.1*b.1,a.2*b.2)
}

//TODO: make this work

// pub fn on_change<T,F>(check_me: T, change_store: &mut T, do_this: F)
// where F: FnMut() -> (){
//     if change_store != check_me {
//         do_this();
//         *change_store = check_me;
//     }
// }
