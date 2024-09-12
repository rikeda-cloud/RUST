use crate::camera::frame_handler;
use opencv::core::{merge, no_array, prelude::*, split, Mat, Rect, Scalar, Size, Vector};
use opencv::objdetect::CascadeClassifier;
use opencv::{imgproc, prelude::*};

pub fn is_grayscale(frame: &Mat) -> Result<bool, opencv::Error> {
    Ok(frame.channels() == 1)
}

pub fn get_dev_number() -> i32 {
    const DEFAULT_DEV_NUMBER: i32 = 0;
    match std::env::var("DEV_NUMBER") {
        Ok(env) => match env.parse::<i32>() {
            Ok(dev_number) => dev_number,
            Err(_) => panic!("Error: DEV_NUMBER is invalid"),
        },
        Err(_) => {
            println!(
                "WARN: DEV_NUMBER NOT SET. USE DEFAULT DEV_NUMBER({})",
                DEFAULT_DEV_NUMBER
            );
            DEFAULT_DEV_NUMBER
        }
    }
}

pub fn remove_color_channel(frame: &Mat, channel_to_remove: usize) -> Result<Mat, opencv::Error> {
    let mut channels: Vector<Mat> = Vector::new();
    split(frame, &mut channels)?;

    channels
        .get(channel_to_remove as usize)?
        .set_to(&Scalar::all(0.0), &no_array())?;

    let mut result = Mat::default();
    merge(&channels, &mut result)?;
    Ok(result)
}

pub fn detect_object(frame: &Mat, model: &str, ract_color: Scalar) -> Result<Mat, opencv::Error> {
    let mut cascade = CascadeClassifier::new(model)?;
    let gray_frame = frame_handler::convert_to_gray(&frame)?;

    // 物体を検出する
    let mut objects = Vector::<Rect>::new();
    cascade.detect_multi_scale(
        &gray_frame,
        &mut objects,
        1.1,
        2,
        opencv::objdetect::CASCADE_FIND_BIGGEST_OBJECT,
        Size::new(100, 100),
        Size::new(0, 0),
    )?;

    // 検出された物体の周りに矩形を描画
    let mut detected_frame = frame.clone();
    for object in objects {
        imgproc::rectangle(
            &mut detected_frame,
            object,
            ract_color,
            2,
            imgproc::LINE_8,
            0,
        )?;
    }

    Ok(detected_frame)
}
