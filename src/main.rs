mod camera;
use camera::Camera;
use opencv::{highgui, Result};
use std::time::Instant;

fn main() -> Result<()> {
    let mut camera = Camera::new(14, false);

    while highgui::wait_key(1).unwrap() != 113 {
        let start_time = Instant::now();
        match camera.capture_frame() {
            Ok(_) => {}
            Err(message) => {
                println!("{}", message);
                break;
            }
        }
        highgui::imshow("VIDEO", &camera.frame)?;
        println!("CaptureTime: {:.4}", start_time.elapsed().as_secs_f64());
    }
    Ok(())
}
