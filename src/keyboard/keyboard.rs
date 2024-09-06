enum KeyNum {
    ESC = 27,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
}

impl TryFrom<i32> for KeyNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            27 => Ok(KeyNum::ESC),
            49 => Ok(KeyNum::Num1),
            50 => Ok(KeyNum::Num2),
            51 => Ok(KeyNum::Num3),
            52 => Ok(KeyNum::Num4),
            _ => Err(()),
        }
    }
}
