#![allow(dead_code)]

#[allow(unused)]
#[non_exhaustive]
pub(crate) struct ArrayMath {}

impl ArrayMath {
    pub(crate) fn multiply_add(a: f32, x: &[f32], destination: &mut [f32]) {
        for i in 0..destination.len() {
            destination[i] += a * x[i];
        }
    }

    pub(crate) fn multiply_add_slope(a: f32, step: f32, x: &[f32], destination: &mut [f32]) {
        let mut a = a;
        for i in 0..destination.len() {
            destination[i] += a * x[i];
            a += step;
        }
    }
}
