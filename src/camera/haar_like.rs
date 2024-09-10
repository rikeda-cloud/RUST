use opencv::core::{Mat, Rect};
use opencv::prelude::*;
use rayon::prelude::*;
use std::cmp::Ordering;

pub fn calc_haar_like_vec(
    frame: &Mat,
    divisions: i32,
    rect_height: i32,
) -> Result<Vec<f64>, opencv::Error> {
    let width = frame.cols();
    let height = frame.rows();
    let width_step = width / divisions as i32;

    // 並列処理を導入
    let result: Vec<f64> = (0..divisions)
        .into_par_iter()
        .map(|i| {
            let x: i32 = i * width_step;
            let roi = Rect::new(x, 0, width_step, height);
            let cropped_mat = frame.roi(roi).unwrap().try_clone().unwrap();
            calc_haar_like(&cropped_mat, rect_height).unwrap_or(0.0)
        })
        .collect();

    Ok(result)
}

fn calc_haar_like(frame: &Mat, rect_height: i32) -> Result<f64, opencv::Error> {
    let rows = frame.rows();
    let cols = frame.cols();

    // SIMD最適化可能な部分
    let mut array_1d = Vec::with_capacity(rows as usize);

    // OpenCVのsum関数を活用
    for r in 0..rows {
        let row_sum: f64 = (0..cols)
            .map(|c| *frame.at_2d::<u8>(r, c).unwrap() as f64)
            .sum();
        array_1d.push(row_sum);
    }

    let kernel = vec![1.0 / rect_height as f64; rect_height as usize];
    let convolved_array = convolve(&array_1d, &kernel);

    // SIMDや並列処理が可能な部分
    let diff_array: Vec<f64> = convolved_array.windows(2).map(|w| w[1] - w[0]).collect();

    let max_idx = diff_array
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal))
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    Ok(max_idx as f64 / diff_array.len() as f64)
}

// 畳み込み関数（シンプルに計算）
fn convolve(array: &[f64], kernel: &[f64]) -> Vec<f64> {
    let kernel_size = kernel.len();
    let mut result = Vec::with_capacity(array.len() - kernel_size + 1);

    // SIMDにより並列化が可能なループ
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
