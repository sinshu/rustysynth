#![allow(dead_code)]

use std::f64::consts;

#[non_exhaustive]
pub(crate) struct Chorus {
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,

    delay_table: Vec<f32>,

    buffer_index_l: usize,
    buffer_index_r: usize,

    delay_table_index_l: usize,
    delay_table_index_r: usize,
}

impl Chorus {
    pub(crate) fn new(sample_rate: i32, delay: f64, depth: f64, frequency: f64) -> Self {
        let buffer_l = vec![0_f32; ((sample_rate as f64) * (delay + depth)) as usize + 2];
        let buffer_r = vec![0_f32; ((sample_rate as f64) * (delay + depth)) as usize + 2];

        let mut delay_table = vec![0_f32; ((sample_rate as f64) / frequency).round() as usize];
        let delay_table_length = delay_table.len();
        for (t, input) in delay_table.iter_mut().enumerate().take(delay_table_length) {
            let phase = 2.0 * consts::PI * (t as f64) / (delay_table_length as f64);
            *input = ((sample_rate as f64) * (delay + depth * phase.sin())) as f32;
        }

        let buffer_index_l: usize = 0;
        let buffer_index_r: usize = 0;

        let delay_table_index_l: usize = 0;
        let delay_table_index_r: usize = delay_table_length / 4;

        Self {
            buffer_l,
            buffer_r,
            delay_table,
            buffer_index_l,
            buffer_index_r,
            delay_table_index_l,
            delay_table_index_r,
        }
    }

    pub(crate) fn process(
        &mut self,
        input_left: &[f32],
        input_right: &[f32],
        output_left: &mut [f32],
        output_right: &mut [f32],
    ) {
        let buffer_length = self.buffer_l.len();
        let delay_table_length = self.delay_table.len();
        let output_length = output_left.len();

        for t in 0..output_length {
            let mut position =
                self.buffer_index_l as f64 - self.delay_table[self.delay_table_index_l] as f64;
            if position < 0.0 {
                position += buffer_length as f64;
            }

            let index1 = position as usize;
            let mut index2 = index1 + 1;
            if index2 == buffer_length {
                index2 = 0;
            }

            let x1 = self.buffer_l[index1] as f64;
            let x2 = self.buffer_l[index2] as f64;
            let a = position - index1 as f64;
            output_left[t] = (x1 + a * (x2 - x1)) as f32;

            self.buffer_l[self.buffer_index_l] = input_left[t];
            self.buffer_index_l += 1;
            if self.buffer_index_l == buffer_length {
                self.buffer_index_l = 0;
            }

            self.delay_table_index_l += 1;
            if self.delay_table_index_l == delay_table_length {
                self.delay_table_index_l = 0;
            }
        }

        for t in 0..output_length {
            let mut position =
                self.buffer_index_r as f64 - self.delay_table[self.delay_table_index_r] as f64;
            if position < 0.0 {
                position += buffer_length as f64;
            }

            let index1 = position as usize;
            let mut index2 = index1 + 1;
            if index2 == buffer_length {
                index2 = 0;
            }

            let x1 = self.buffer_r[index1] as f64;
            let x2 = self.buffer_r[index2] as f64;
            let a = position - index1 as f64;
            output_right[t] = (x1 + a * (x2 - x1)) as f32;

            self.buffer_r[self.buffer_index_r] = input_right[t];
            self.buffer_index_r += 1;
            if self.buffer_index_r == buffer_length {
                self.buffer_index_r = 0;
            }

            self.delay_table_index_r += 1;
            if self.delay_table_index_r == delay_table_length {
                self.delay_table_index_r = 0;
            }
        }
    }

    pub(crate) fn mute(&mut self) {
        let buffer_length = self.buffer_l.len();

        for t in 0..buffer_length {
            self.buffer_l[t] = 0_f32;
        }

        for t in 0..buffer_length {
            self.buffer_r[t] = 0_f32;
        }
    }
}
