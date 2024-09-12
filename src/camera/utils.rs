use opencv::core::Mat;
use opencv::prelude::MatTraitConst;

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
