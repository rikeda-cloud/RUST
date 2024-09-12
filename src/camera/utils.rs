use opencv::core::Mat;
use opencv::prelude::MatTraitConst;

pub fn is_grayscale(frame: &Mat) -> Result<bool, opencv::Error> {
    Ok(frame.channels() == 1)
}
