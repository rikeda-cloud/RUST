use crate::camera::frame_handler;
use opencv::prelude::{MatTraitConst, VideoCaptureTrait, VideoCaptureTraitConst};
use opencv::{core, videoio};

pub struct Camera {
    pub frame: core::Mat,
    capture: videoio::VideoCapture,
    frame_handler: frame_handler::FrameHandler,
}

impl Camera {
    pub fn new(camera_index: i32, frame_mode: &str) -> Self {
        let capture = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)
            .expect("Error: new VideoCapture");
        capture.is_opened().expect("Error: Camera Init");

        Self {
            capture,
            frame: core::Mat::default(),
            frame_handler: frame_handler::search_frame_handler(frame_mode).unwrap(),
        }
    }

    pub fn capture_frame(&mut self) {
        self.capture.read(&mut self.frame).expect("Error: read");
        if self.frame.empty() {
            panic!("Error: read");
        }
        self.frame = (self.frame_handler)(&self.frame);
    }

    pub fn switch_frame_handler(&mut self, mode: String) {
        match frame_handler::search_frame_handler(&mode) {
            Some(frame_handler) => {
                self.frame_handler = frame_handler;
            }
            None => {}
        }
    }
}
