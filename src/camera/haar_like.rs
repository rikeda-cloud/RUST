use opencv::core::{Mat, Rect};
use opencv::prelude::*;
use std::cmp::Ordering;

pub fn calc_haar_like_vec(
    frame: &Mat,
    divisions: i32,
    rect_height: i32,
) -> Result<Vec<f64>, opencv::Error> {
    let mut result: Vec<f64> = vec![];
    let width = frame.cols();
    let height = frame.rows();
    let width_step = width / divisions as i32;

    for i in 0..divisions {
        let x: i32 = i * width_step;
        let roi = Rect::new(x, 0, width_step, height);
        let cropped_mat = frame.roi(roi)?.try_clone()?;
        let pos: f64 = calc_haar_like(&cropped_mat, rect_height)?;
        result.push(pos);
    }
    Ok(result)
}

fn calc_haar_like(frame: &Mat, rect_height: i32) -> Result<f64, opencv::Error> {
    let rows = frame.rows();
    let cols = frame.cols();
    let mut array_1d = Vec::with_capacity(rows as usize);

    for r in 0..rows {
        let mut sum = 0.0;
        for c in 0..cols {
            let pixel = frame.at_2d::<u8>(r, c)?;
            sum += *pixel as f64;
        }
        array_1d.push(sum);
    }

    let kernel = vec![1.0 / rect_height as f64; rect_height as usize];
    let convolved_array = convolve(&array_1d, &kernel);
    let diff_array: Vec<f64> = convolved_array.windows(2).map(|w| w[1] - w[0]).collect();
    let max_idx = diff_array
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal))
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    Ok(max_idx as f64 / diff_array.len() as f64)
}

fn convolve(array: &[f64], kernel: &[f64]) -> Vec<f64> {
    let kernel_size = kernel.len();
    let mut result = Vec::with_capacity(array.len() - kernel_size + 1);

    for i in 0..=array.len() - kernel_size {
        let sum: f64 = array[i..i + kernel_size]
            .iter()
            .zip(kernel.iter())
            .map(|(a, b)| a * b)
            .sum();
        result.push(sum);
    }
    result
}
