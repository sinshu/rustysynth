#![allow(dead_code)]

#[allow(unused)]
#[non_exhaustive]
pub(crate) struct ArrayMath {}

impl ArrayMath {
    pub(crate) fn multiply_add(a: f32, x: &[f32], destination: &mut [f32]) {
        for (x, destination) in x.iter().zip(destination.iter_mut()) {
            *destination += a * *x;
        }
    }

    pub(crate) fn multiply_add_slope(a: f32, step: f32, x: &[f32], destination: &mut [f32]) {
        let mut a = a;
        for (x, destination) in x.iter().zip(destination.iter_mut()) {
            *destination += a * *x;
            a += step;
        }
    }
}
