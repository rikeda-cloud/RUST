use crate::camera::convert_frame;
use opencv::prelude::{MatTraitConst, VideoCaptureTrait, VideoCaptureTraitConst};
use opencv::{core, videoio};

pub struct Camera {
    pub frame: core::Mat,
    capture: videoio::VideoCapture,
    func_convert_frame: convert_frame::FuncConvertFrame,
}

impl Camera {
    pub fn new(camera_index: i32, frame_mode: &str) -> Self {
        let capture = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)
            .expect("Error: new VideoCapture");
        capture.is_opened().expect("Error: Camera Init");

        Self {
            capture,
            frame: core::Mat::default(),
            func_convert_frame: convert_frame::search_convert_frame(frame_mode).unwrap(),
        }
    }

    pub fn capture_frame(&mut self) {
        self.capture.read(&mut self.frame).expect("Error: read");
        if self.frame.empty() {
            panic!("Error: read");
        }
        self.frame = (self.func_convert_frame)(&self.frame);
    }
}
