use std::ops::Deref;


const COLORS_REGEX = r"(?:(?<!\\)(&[0-9]{1,2}|&r))";

pub enum Color {
    Reset,

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,

    BG_Black,
    BG_Red,
    BG_Green,
    BG_Yellow,
    BG_Blue,
    BG_Magenta,
    BG_Cyan,
    BG_White,
    BG_BrightBlack,
    BG_BrightRed,
    BG_BrightGreen,
    BG_BrightYellow,
    BG_BrightBlue,
    BG_BrightMagenta,
    BG_BrightCyan,
    BG_BrightWhite,
}

impl Color {
    pub fn to_str(&self) -> Cow<'static, str> {
        match *self {
            Color::Reset => "0".into(),

            Color::Black => "30".into(),
            Color::Red => "31".into(),
            Color::Green => "32".into(),
            Color::Yellow => "33".into(),
            Color::Blue => "34".into(),
            Color::Magenta => "35".into(),
            Color::Cyan => "36".into(),
            Color::White => "37".into(),
            Color::BrightBlack => "90".into(),
            Color::BrightRed => "91".into(),
            Color::BrightGreen => "92".into(),
            Color::BrightYellow => "93".into(),
            Color::BrightBlue => "94".into(),
            Color::BrightMagenta => "95".into(),
            Color::BrightCyan => "96".into(),
            Color::BrightWhite => "97".into(),

            Color::BG_Black => "40".into(),
            Color::BG_Red => "41".into(),
            Color::BG_Green => "42".into(),
            Color::BG_Yellow => "43".into(),
            Color::BG_Blue => "44".into(),
            Color::BG_Magenta => "45".into(),
            Color::BG_Cyan => "46".into(),
            Color::BG_White => "47".into(),
            Color::BG_BrightBlack => "100".into(),
            Color::BG_BrightRed => "101".into(),
            Color::BG_BrightGreen => "102".into(),
            Color::BG_BrightYellow => "103".into(),
            Color::BG_BrightBlue => "104".into(),
            Color::BG_BrightMagenta => "105".into(),
            Color::BG_BrightCyan => "106".into(),
            Color::BG_BrightWhite => "107".into(),
        }
    }
}

pub fn parse_to_terminal_colors(origin: &String) -> String {
    let result = origin.deref().clone();
    let re = regex::Regex::new(COLORS_REGEX).unwrap();
    let captures = re.captures(origin).unwrap();

    for capture in captures.iter() {
        let c = capture.unwrap();
        c.start();
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::parse_to_terminal_colors;

    #[test]
    fn test_terminal_colors() {
        let r = parse_to_terminal_colors(&"Test&1 console log&r output".to_string());
        assert_eq!(r, "".to_string());
    }
}
