mod camera;
mod keyboard;
use camera::Camera;
use keyboard::KeyNum;
use opencv::{highgui, Result};
use std::time::Instant;

fn main() -> Result<()> {
    const CAMERA_NUMBER: i32 = 14;
    let mut camera = Camera::new(CAMERA_NUMBER, "countours");

    loop {
        match highgui::wait_key(1) {
            Ok(key) => match key {
                KeyNum::ESC => break,
                KeyNum::Num1 => camera.switch_frame_handler("color".to_string()),
                KeyNum::Num2 => camera.switch_frame_handler("gray".to_string()),
                KeyNum::Num3 => camera.switch_frame_handler("canny".to_string()),
                KeyNum::Num4 => camera.switch_frame_handler("white_balance".to_string()),
                _ => {}
            },
            Err(_) => {
                break;
            }
        }
        let start_time = Instant::now();
        camera.capture_frame();
        highgui::imshow("VIDEO", &camera.frame)?;
        println!("CaptureTime: {:.4}", start_time.elapsed().as_secs_f64());
    }
    Ok(())
}
