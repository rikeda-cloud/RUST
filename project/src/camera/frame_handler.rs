use opencv::{core, dnn_superres, imgproc, prelude::*, ximgproc, xphoto};
use std::collections::HashMap;

pub type FrameHandler = fn(frame: &core::Mat) -> Result<core::Mat, opencv::Error>;

pub fn search_frame_handler(mode: &str) -> Option<FrameHandler> {
    let frame_handler = create_frame_handler_map();
    frame_handler.get(mode).copied()
}

fn create_frame_handler_map() -> HashMap<&'static str, FrameHandler> {
    let mut frame_handler_map: HashMap<&str, FrameHandler> = HashMap::new();
    frame_handler_map.insert("color", convert_to_color);
    frame_handler_map.insert("gray", convert_to_gray);
    frame_handler_map.insert("canny", convert_to_canny);
    frame_handler_map.insert("white_balance", convert_to_white_balance);
    frame_handler_map.insert("filter", convert_to_bilateral_filter);
    frame_handler_map.insert("superpixel", convert_to_superpixel);
    frame_handler_map.insert("countours", convert_to_countours);
    frame_handler_map.insert("fsrcnn", convert_to_fsrcnn);
    frame_handler_map.insert("espcn", convert_to_espcn);
    frame_handler_map.insert("binary", convert_to_binary);
    frame_handler_map
}

// グレースケール
pub fn convert_to_gray(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    let mut gray_frame = core::Mat::default();
    imgproc::cvt_color(frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
    Ok(gray_frame)
}

// cannyエッジ検出
pub fn convert_to_canny(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    // THRESHOLD1 <= エッジとして判定 <= THRESHOLD2
    const THRESHOLD1: f64 = 100.0;
    const THRESHOLD2: f64 = 200.0;
    // エッジ検出に使用されるソーベル演算子のサイズ
    // (3, 5, 7) のいずれかの値が有効
    const APERTURE_SIZE: i32 = 3;
    // TRUE -> (L2ノルムを使用, 精度向上 & 計算コスト増)
    // FALSE -> (L1ノルムが使用)
    const L2_GRADIENT: bool = false;
    let mut canny_frame = convert_to_gray(frame)?;

    imgproc::canny(
        &frame,
        &mut canny_frame,
        THRESHOLD1,
        THRESHOLD2,
        APERTURE_SIZE,
        L2_GRADIENT,
    )?;
    Ok(canny_frame)
}

// そのまま
pub fn convert_to_color(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    Ok(frame.clone())
}

// 色調補正(白をより現実の色に変える)
pub fn convert_to_white_balance(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    let mut white_balance_frame = core::Mat::default();
    let mut grayworld_wb = xphoto::create_grayworld_wb()?;
    grayworld_wb.balance_white(&frame, &mut white_balance_frame)?;
    Ok(white_balance_frame)
}

// ぼかし(ノイズ除去。エッジ検出と併用可能)
pub fn convert_to_bilateral_filter(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    let mut filtered_frame = core::Mat::default();
    imgproc::bilateral_filter(
        &frame,
        &mut filtered_frame,
        9,    // ダイアメータ
        75.0, // シグマ色
        75.0, // シグマ空間
        core::BORDER_DEFAULT,
    )?;
    Ok(filtered_frame)
}

// スーパーピクセル
pub fn convert_to_superpixel(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    let mut superpixeld_frame = core::Mat::default();
    let mut slic = ximgproc::create_superpixel_slic(frame, ximgproc::SLIC, 25, 100.0)?;
    slic.iterate(5)?;
    slic.get_labels(&mut superpixeld_frame)?;
    slic.get_label_contour_mask(&mut superpixeld_frame, true)?;
    Ok(superpixeld_frame)
}

// 輪郭
fn convert_to_countours(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    let mut contours = core::Vector::<core::Vector<core::Point>>::new();
    let edges = convert_to_canny(frame)?;
    imgproc::find_contours(
        &edges,                       // 入力画像（エッジ検出後の画像）
        &mut contours,                // 検出された輪郭が格納されるベクター
        imgproc::RETR_EXTERNAL,       // 輪郭の検出モード（最外輪郭のみ）
        imgproc::CHAIN_APPROX_SIMPLE, // 輪郭の近似方法（簡略化された近似）
        core::Point::new(0, 0),       // 検出のオフセット（画像全体）
    )?;

    let mut result = frame.clone();
    imgproc::draw_contours(
        &mut result,
        &contours,
        -1,                                      // 全ての輪郭を描画
        core::Scalar::new(0.0, 255.0, 0.0, 0.0), // 青色で描画
        2,                                       // 線の太さ
        imgproc::LINE_8,                         // 線のタイプ
        &frame,                                  // 階層情報
        0,                                       // 階層レベル 2
        core::Point::new(0, 0),                  // オフセット
    )?;
    Ok(result)
}

// 超解像処理(FSRCNN)
fn convert_to_fsrcnn(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    let mut sr = dnn_superres::DnnSuperResImpl::create()?;
    sr.read_model("model/fsrcnn.pb")?;
    sr.set_model("fsrcnn", 2)?;

    let mut result = Mat::default();
    sr.upsample(&frame, &mut result)?;
    Ok(result)
}

// 超解像処理(ESPCN)
fn convert_to_espcn(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    let mut sr = dnn_superres::DnnSuperResImpl::create()?;
    sr.read_model("model/espcn.pb")?;
    sr.set_model("espcn", 2)?;

    let mut result = Mat::default();
    sr.upsample(&frame, &mut result)?;
    Ok(result)
}

// 白黒の二値化
fn convert_to_binary(frame: &core::Mat) -> Result<core::Mat, opencv::Error> {
    const THRESHOLD: f64 = 200.0;
    const MAX_VALUE: f64 = 255.0;
    let mut binary_frame = core::Mat::default();

    imgproc::threshold(
        &convert_to_gray(&frame)?,
        &mut binary_frame,
        THRESHOLD,
        MAX_VALUE,
        imgproc::THRESH_BINARY,
    )?;
    Ok(binary_frame)
}
