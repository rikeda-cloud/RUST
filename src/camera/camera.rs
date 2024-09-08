use crate::camera::frame_handler;
use crate::keyboard::KeyNum;
use opencv::prelude::{MatTraitConst, VideoCaptureTrait, VideoCaptureTraitConst};
use opencv::{core, highgui, videoio};

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

    pub fn capture_frame(&mut self) -> Result<(), opencv::Error> {
        self.capture.read(&mut self.frame).expect("Error: read");
        if self.frame.empty() {
            panic!("Error: read");
        }
        self.frame = (self.frame_handler)(&self.frame)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn handle_key(&mut self) -> bool {
        match highgui::wait_key(1) {
            Ok(key) => match KeyNum::try_from(key) {
                Ok(KeyNum::ESC) => false,
                Ok(KeyNum::Num0) => self.switch_frame_handler("binary".to_string()),
                Ok(KeyNum::Num1) => self.switch_frame_handler("color".to_string()),
                Ok(KeyNum::Num2) => self.switch_frame_handler("gray".to_string()),
                Ok(KeyNum::Num3) => self.switch_frame_handler("canny".to_string()),
                Ok(KeyNum::Num4) => self.switch_frame_handler("white_balance".to_string()),
                Ok(KeyNum::Num5) => self.switch_frame_handler("filter".to_string()),
                Ok(KeyNum::Num6) => self.switch_frame_handler("superpixel".to_string()),
                Ok(KeyNum::Num7) => self.switch_frame_handler("countours".to_string()),
                Ok(KeyNum::Num8) => self.switch_frame_handler("fsrcnn".to_string()),
                Ok(KeyNum::Num9) => self.switch_frame_handler("espcn".to_string()),
                _ => true,
            },
            Err(_) => false,
        }
    }

    pub fn handle_key_websocket(&mut self, c: i32) -> bool {
        match c {
            1 => self.switch_frame_handler("binary".to_string()),
            2 => self.switch_frame_handler("color".to_string()),
            3 => self.switch_frame_handler("gray".to_string()),
            4 => self.switch_frame_handler("canny".to_string()),
            5 => self.switch_frame_handler("white_balance".to_string()),
            6 => self.switch_frame_handler("filter".to_string()),
            7 => self.switch_frame_handler("superpixel".to_string()),
            8 => self.switch_frame_handler("countours".to_string()),
            9 => self.switch_frame_handler("fsrcnn".to_string()),
            0 => self.switch_frame_handler("espcn".to_string()),
            _ => true,
        }
    }

    fn switch_frame_handler(&mut self, mode: String) -> bool {
        match frame_handler::search_frame_handler(&mode) {
            Some(frame_handler) => {
                self.frame_handler = frame_handler;
                true
            }
            None => true,
        }
    }
}
