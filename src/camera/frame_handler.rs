use crate::camera::{haar_like, text, utils};
use opencv::core::{flip, Mat, Point, Rect, Scalar, Size, Vector, BORDER_DEFAULT};
use opencv::objdetect::CascadeClassifier;
use opencv::{dnn_superres, imgproc, prelude::*, ximgproc, xphoto};
use std::collections::HashMap;

pub type FrameHandler = fn(frame: &Mat) -> Result<Mat, opencv::Error>;

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
    frame_handler_map.insert("haar_like", convert_to_haar_like);
    frame_handler_map.insert("removed_red", convert_to_removed_red);
    frame_handler_map.insert("removed_blue", convert_to_removed_blue);
    frame_handler_map.insert("removed_green", convert_to_removed_green);
    frame_handler_map.insert("text", convert_to_text_frame);
    frame_handler_map.insert("face", convert_to_detect_faces);
    frame_handler_map.insert("reverse", convert_to_reverse);
    frame_handler_map
}

// グレースケール
pub fn convert_to_gray(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }
    let mut gray_frame = Mat::default();
    imgproc::cvt_color(frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
    Ok(gray_frame)
}

// cannyエッジ検出
pub fn convert_to_canny(frame: &Mat) -> Result<Mat, opencv::Error> {
    // THRESHOLD1 <= エッジとして判定 <= THRESHOLD2
    const THRESHOLD1: f64 = 100.0;
    const THRESHOLD2: f64 = 200.0;
    // エッジ検出時のソーベル演算子のサイズ(3, 5, 7)
    const APERTURE_SIZE: i32 = 3;
    // TRUE -> L2ノルム, FALSE -> L1ノルム
    const L2_GRADIENT: bool = false;

    let mut canny_frame = match utils::is_grayscale(frame)? {
        true => frame.clone(),
        false => convert_to_gray(frame)?,
    };

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
pub fn convert_to_color(frame: &Mat) -> Result<Mat, opencv::Error> {
    Ok(frame.clone())
}

// 色調補正(白をより現実の色に変える)
pub fn convert_to_white_balance(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }

    let mut white_balance_frame = Mat::default();
    let mut grayworld_wb = xphoto::create_grayworld_wb()?;
    grayworld_wb.balance_white(&frame, &mut white_balance_frame)?;
    Ok(white_balance_frame)
}

// ぼかし(ノイズ除去。エッジ検出と併用可能)
pub fn convert_to_bilateral_filter(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }

    let mut filtered_frame = Mat::default();
    imgproc::bilateral_filter(
        &frame,
        &mut filtered_frame,
        9,    // ダイアメータ
        75.0, // シグマ色
        75.0, // シグマ空間
        BORDER_DEFAULT,
    )?;
    Ok(filtered_frame)
}

// スーパーピクセル
pub fn convert_to_superpixel(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }

    let mut superpixeld_frame = Mat::default();
    let mut slic = ximgproc::create_superpixel_slic(frame, ximgproc::SLIC, 25, 100.0)?;
    slic.iterate(5)?;
    slic.get_labels(&mut superpixeld_frame)?;
    slic.get_label_contour_mask(&mut superpixeld_frame, true)?;
    Ok(superpixeld_frame)
}

// 輪郭
pub fn convert_to_countours(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }

    let mut result = frame.clone();
    let mut contours = Vector::<Vector<Point>>::new();
    let edges = convert_to_canny(frame)?;
    imgproc::find_contours(
        &edges,                       // 入力画像（エッジ検出後の画像）
        &mut contours,                // 検出された輪郭が格納されるベクター
        imgproc::RETR_EXTERNAL,       // 輪郭の検出モード（最外輪郭のみ）
        imgproc::CHAIN_APPROX_SIMPLE, // 輪郭の近似方法（簡略化された近似）
        Point::new(0, 0),             // 検出のオフセット（画像全体）
    )?;

    imgproc::draw_contours(
        &mut result,
        &contours,
        -1,                                // 全ての輪郭を描画
        Scalar::new(0.0, 255.0, 0.0, 0.0), // 青色で描画
        2,                                 // 線の太さ
        imgproc::LINE_8,                   // 線のタイプ
        &frame,                            // 階層情報
        0,                                 // 階層レベル 2
        Point::new(0, 0),                  // オフセット
    )?;
    Ok(result)
}

// 超解像処理(FSRCNN)
pub fn convert_to_fsrcnn(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }

    let mut result = Mat::default();
    let mut sr = dnn_superres::DnnSuperResImpl::create()?;
    sr.read_model("model/fsrcnn.pb")?;
    sr.set_model("fsrcnn", 2)?;

    sr.upsample(&frame, &mut result)?;
    Ok(result)
}

// 超解像処理(ESPCN)
pub fn convert_to_espcn(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }

    let mut result = Mat::default();
    let mut sr = dnn_superres::DnnSuperResImpl::create()?;
    sr.read_model("model/espcn.pb")?;
    sr.set_model("espcn", 2)?;

    sr.upsample(&frame, &mut result)?;
    Ok(result)
}

// 白黒の二値化
pub fn convert_to_binary(frame: &Mat) -> Result<Mat, opencv::Error> {
    const THRESHOLD: f64 = 200.0;
    const MAX_VALUE: f64 = 255.0;
    let mut binary_frame = Mat::default();

    imgproc::threshold(
        &convert_to_gray(&frame)?,
        &mut binary_frame,
        THRESHOLD,
        MAX_VALUE,
        imgproc::THRESH_BINARY,
    )?;
    Ok(binary_frame)
}

pub fn convert_to_haar_like(frame: &Mat) -> Result<Mat, opencv::Error> {
    // 取得する特徴の数, frameを横に区切る数
    const DIVISIONS: i32 = 40;
    const RECT_HEIGHT: i32 = 15;
    const BLACK: Scalar = Scalar::new(0.0, 0.0, 0.0, 0.0);
    let mut haar_like_frame = frame.clone();
    let haar_like_vec = haar_like::calc_haar_like_vec(&frame, DIVISIONS, RECT_HEIGHT)?;
    let width = frame.cols();
    let height = frame.rows();
    let width_step = width / DIVISIONS as i32;

    for i in 0..DIVISIONS {
        let x = width_step * i;
        let y = (haar_like_vec[i as usize] * height as f64) as i32;
        let rect = Rect::new(x, y, width_step, 1);
        let _ = imgproc::rectangle(&mut haar_like_frame, rect, BLACK, 1, imgproc::LINE_8, 0);
    }
    Ok(haar_like_frame)
}

fn convert_to_removed_red(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }
    utils::remove_color_channel(frame, 2)
}

fn convert_to_removed_blue(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }
    utils::remove_color_channel(frame, 0)
}

fn convert_to_removed_green(frame: &Mat) -> Result<Mat, opencv::Error> {
    if utils::is_grayscale(frame)? {
        return Ok(frame.clone());
    }
    utils::remove_color_channel(frame, 1)
}

fn convert_to_text_frame(frame: &Mat) -> Result<Mat, opencv::Error> {
    let text: String = text::extract_text(frame)?;
    let mut result: Mat = frame.clone();

    let font_face = imgproc::FONT_HERSHEY_SIMPLEX;
    let font_scale = 1.0;
    let thickness = 2;
    let black = Scalar::new(0.0, 0.0, 0.0, 0.0);

    let text_size = imgproc::get_text_size(&text, font_face, font_scale, thickness, &mut 0)?;
    let text_org = Point::new(
        (frame.cols() - text_size.width) / 2,
        (frame.rows() + text_size.height) / 2,
    );

    imgproc::put_text(
        &mut result,
        &text,
        text_org,
        font_face,
        font_scale,
        black,
        thickness,
        imgproc::LINE_AA,
        false,
    )?;
    Ok(result)
}

fn convert_to_detect_faces(frame: &Mat) -> Result<Mat, opencv::Error> {
    let mut face_cascade = CascadeClassifier::new("model/haarcascade_frontalface_default.xml")?;
    let gray_frame = convert_to_gray(&frame)?;

    // 顔を検出する
    let mut faces = Vector::<Rect>::new();
    face_cascade.detect_multi_scale(
        &gray_frame,
        &mut faces,
        1.1,
        3,
        0,
        Size::new(30, 30),
        Size::new(0, 0),
    )?;

    // 検出された顔の周りに矩形を描画
    let mut output_frame = frame.clone();
    for face in faces {
        imgproc::rectangle(
            &mut output_frame,
            face,
            Scalar::new(0.0, 255.0, 0.0, 0.0),
            2,
            imgproc::LINE_8,
            0,
        )?;
    }

    Ok(output_frame)
}

fn convert_to_reverse(frame: &Mat) -> Result<Mat, opencv::Error> {
    let mut reversed_frame = Mat::default();
    let _ = flip(&frame, &mut reversed_frame, 1);
    Ok(reversed_frame)
}
