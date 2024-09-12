use opencv::core::{merge, no_array, prelude::*, split, Mat, Scalar, Vector};

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
