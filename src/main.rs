mod camera;
mod keyboard;
use camera::Camera;
use opencv::{highgui, Result};
use std::time::Instant;

fn main() -> Result<()> {
    const CAMERA_NUMBER: i32 = 14;
    let mut camera = Camera::new(CAMERA_NUMBER, "espcn");

    while camera.handle_key() {
        let start_time = Instant::now();
        camera.capture_frame();
        highgui::imshow("VIDEO", &camera.frame)?;
        println!("CaptureTime: {:.4}", start_time.elapsed().as_secs_f64());
    }
    Ok(())
}
