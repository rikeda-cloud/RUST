pub enum KeyNum {
    ESC = 27,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
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
            53 => Ok(KeyNum::Num5),
            54 => Ok(KeyNum::Num6),
            55 => Ok(KeyNum::Num7),
            56 => Ok(KeyNum::Num8),
            57 => Ok(KeyNum::Num9),
            _ => Err(()),
        }
    }
}
