use crate::camera::{frame_handler, utils};
use ndarray::{prelude::*, Array2, ArrayView2};
use opencv::{core::Mat, prelude::*};
use rayon::prelude::*;
use std::cmp::Ordering;

pub fn calc_haar_like_vec(
    frame: &Mat,
    divisions: i32,
    rect_height: i32,
) -> Result<Vec<f64>, opencv::Error> {
    let gray_frame = match utils::is_grayscale(frame)? {
        true => frame.clone(),
        false => frame_handler::convert_to_gray(&frame)?,
    };
    let width = gray_frame.cols() as usize;
    let height = gray_frame.rows() as usize;
    let width_step = width / divisions as usize;

    let array = Array2::<u8>::from_shape_vec(
        (height, width),
        gray_frame
            .data_bytes()
            .unwrap()
            .iter()
            .map(|&x| x)
            .collect(),
    )
    .unwrap();

    let result: Vec<f64> = (0..divisions)
        .into_par_iter()
        .map(|i| {
            let x = i as usize * width_step;
            let cropped_array = array.slice(s![.., x..x + width_step]);
            calc_haar_like_ndarray(&cropped_array, rect_height)
        })
        .collect();
    Ok(result)
}

fn calc_haar_like_ndarray(frame: &ArrayView2<u8>, rect_height: i32) -> f64 {
    let (rows, _) = frame.dim();

    let mut array_1d = vec![0.0; rows];
    for r in 0..rows {
        array_1d[r] = frame.row(r).iter().map(|&x| x as f64).sum();
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

    max_idx as f64 / diff_array.len() as f64
}

fn convolve(array: &[f64], kernel: &[f64]) -> Vec<f64> {
    let kernel_size = kernel.len();
    let array_len = array.len();
    let mut result = Vec::with_capacity(array_len - kernel_size + 1);

    let mut sum: f64 = 0.0;
    for i in 0..kernel_size {
        sum += array[i] * kernel[kernel_size - 1 - i];
    }
    result.push(sum);

    for i in kernel_size..array_len {
        sum += array[i] * kernel[0] - array[i - kernel_size] * kernel[kernel_size - 1];
        result.push(sum);
    }
    result
}
