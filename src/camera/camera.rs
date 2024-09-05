use crate::camera::utils;
use opencv::{core, prelude::*, videoio, Result};

pub struct Camera {
    pub capture: videoio::VideoCapture,
    pub frame: core::Mat,
    gray_flag: bool,
}

impl Camera {
    pub fn new(camera_index: i32, gray_flag: bool) -> Self {
        let capture = utils::init_camera(camera_index).unwrap();
        let frame = core::Mat::default();
        Self {
            capture,
            frame,
            gray_flag,
        }
    }

    pub fn capture_frame(&mut self) -> Result<(), String> {
        self.capture
            .read(&mut self.frame)
            .map_err(|e| e.to_string())?;
        if self.frame.empty() {
            return Err("Error: read".to_string());
        }
        if self.gray_flag {
            self.frame = utils::get_gray_frame(&self.frame);
        }
        Ok(())
    }
}
