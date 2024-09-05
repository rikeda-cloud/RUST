use opencv::prelude::VideoCaptureTraitConst;
use opencv::{core, imgproc, videoio, Result};

pub fn init_camera(camera_index: i32) -> Result<videoio::VideoCapture, String> {
    let capture =
        videoio::VideoCapture::new(camera_index, videoio::CAP_ANY).map_err(|e| e.to_string())?;
    if !capture.is_opened().map_err(|e| e.to_string())? {
        return Err("Open Error".to_string());
    }
    Ok(capture)
}

pub fn get_gray_frame(frame: &core::Mat) -> core::Mat {
    let mut gray_frame = core::Mat::default();
    imgproc::cvt_color(frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    gray_frame
}
