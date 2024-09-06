use opencv::{core, imgproc, prelude::*, ximgproc, xphoto};
use std::collections::HashMap;

pub type FuncConvertFrame = fn(frame: &core::Mat) -> core::Mat;

pub fn search_convert_frame(mode: &str) -> Option<FuncConvertFrame> {
    let mode_map = create_camera_mode_map();
    mode_map.get(mode).copied()
}

fn create_camera_mode_map() -> HashMap<&'static str, FuncConvertFrame> {
    let mut mode_map: HashMap<&str, FuncConvertFrame> = HashMap::new();
    mode_map.insert("color", convert_to_color);
    mode_map.insert("gray", convert_to_gray);
    mode_map.insert("canny", convert_to_canny);
    mode_map.insert("white_balance", convert_to_white_balance);
    mode_map.insert("filter", convert_to_bilateral_filter);
    mode_map.insert("superpixel", convert_to_superpixel);
    mode_map
}

// グレースケール
pub fn convert_to_gray(frame: &core::Mat) -> core::Mat {
    let mut gray_frame = core::Mat::default();
    imgproc::cvt_color(frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    gray_frame
}

// cannyエッジ検出
pub fn convert_to_canny(frame: &core::Mat) -> core::Mat {
    // THRESHOLD1 <= エッジとして判定 <= THRESHOLD2
    const THRESHOLD1: f64 = 100.0;
    const THRESHOLD2: f64 = 200.0;
    // エッジ検出に使用されるソーベル演算子のサイズ
    // (1, 3, 5, 7) のいずれかの値が有効
    const APERTURE_SIZE: i32 = 3;
    // TRUE -> (L2ノルムを使用, 精度向上 & 計算コスト増)
    // FALSE -> (L1ノルムが使用)
    const L2_GRADIENT: bool = false;
    let mut canny_frame = convert_to_gray(frame);

    imgproc::canny(
        &frame,
        &mut canny_frame,
        THRESHOLD1,
        THRESHOLD2,
        APERTURE_SIZE,
        L2_GRADIENT,
    )
    .unwrap();
    canny_frame
}

// そのまま
pub fn convert_to_color(frame: &core::Mat) -> core::Mat {
    frame.clone()
}

// 色調補正(白をより現実の色に変える)
pub fn convert_to_white_balance(frame: &core::Mat) -> core::Mat {
    let mut white_balance_frame = core::Mat::default();
    let mut grayworld_wb = xphoto::create_grayworld_wb().unwrap();
    grayworld_wb
        .balance_white(&frame, &mut white_balance_frame)
        .unwrap();
    white_balance_frame
}

// ぼかし(ノイズ除去。エッジ検出と併用可能)
pub fn convert_to_bilateral_filter(frame: &core::Mat) -> core::Mat {
    let mut filtered_frame = core::Mat::default();
    imgproc::bilateral_filter(
        &frame,
        &mut filtered_frame,
        9,    // ダイアメータ
        75.0, // シグマ色
        75.0, // シグマ空間
        core::BORDER_DEFAULT,
    )
    .unwrap();
    filtered_frame
}

// スーパーピクセル
pub fn convert_to_superpixel(frame: &core::Mat) -> core::Mat {
    let mut superpixeld_frame = core::Mat::default();
    let mut slic = ximgproc::create_superpixel_slic(frame, ximgproc::SLICO, 25, 200.0).unwrap();
    slic.iterate(5).unwrap();

    slic.get_labels(&mut superpixeld_frame).unwrap();
    slic.get_label_contour_mask(&mut superpixeld_frame, true)
        .unwrap();

    superpixeld_frame
}
