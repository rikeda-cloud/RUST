use crate::camera::frame_handler;
use opencv::prelude::{MatTraitConst, VideoCaptureTrait, VideoCaptureTraitConst};
use opencv::videoio::{
    VideoCapture, CAP_PROP_FRAME_HEIGHT as CAP_H, CAP_PROP_FRAME_WIDTH as CAP_W,
};
use opencv::{core::Mat, videoio};

pub struct Camera {
    pub frame: Mat,
    capture: videoio::VideoCapture,
    process_chain: Vec<String>,
}

impl Camera {
    pub fn new(camera_index: i32) -> Self {
        let mut capture = VideoCapture::new(camera_index, videoio::CAP_ANY).expect("Error: Camera");
        capture.is_opened().expect("Error: Camera Init");
        capture.set(CAP_W, 640.0).expect("Error: Width Size");
        capture.set(CAP_H, 480.0).expect("Error: Height Size");

        Self {
            capture,
            frame: Mat::default(),
            process_chain: vec![],
        }
    }

    pub fn capture_frame(&mut self) -> Result<(), opencv::Error> {
        self.capture.read(&mut self.frame).expect("Error: read");
        if self.frame.empty() {
            panic!("Error: read");
        }
        self.process_frame_by_process_chain()?;
        Ok(())
    }

    fn process_frame_by_process_chain(&mut self) -> Result<(), opencv::Error> {
        for process in self.process_chain.iter() {
            match frame_handler::search_frame_handler(&process) {
                Some(frame_handler) => self.frame = frame_handler(&self.frame)?,
                None => {}
            }
        }
        Ok(())
    }

    pub fn set_process_chain(&mut self, new_process_chain: Vec<String>) {
        self.process_chain = new_process_chain;
    }
}
