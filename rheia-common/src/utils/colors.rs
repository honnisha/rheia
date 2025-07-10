use std::borrow::Cow;

use log::Level;

const COLORS_REGEX: &str = r"(?:&[0-9rabcdef]{1})";

// https://minecraft.fandom.com/wiki/Formatting_codes

pub enum Color {
    Reset,
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

impl Color {
    pub fn from_str(origin: &str) -> Option<Color> {
        let color = match origin {
            "&r" => Color::Reset,
            "&0" => Color::Black,
            "&1" => Color::DarkBlue,
            "&2" => Color::DarkGreen,
            "&3" => Color::DarkAqua,
            "&4" => Color::DarkRed,
            "&5" => Color::DarkPurple,
            "&6" => Color::Gold,
            "&7" => Color::Gray,
            "&8" => Color::DarkGray,
            "&9" => Color::Blue,
            "&a" => Color::Green,
            "&b" => Color::Aqua,
            "&c" => Color::Red,
            "&d" => Color::LightPurple,
            "&e" => Color::Yellow,
            "&f" => Color::White,
            _ => return None,
        };
        let color = color;
        return Some(color);
    }

    pub fn to_terminal_code(&self) -> Cow<'static, str> {
        match *self {
            Color::Reset => "0".into(),
            Color::Black => "30".into(),
            Color::DarkBlue => "34".into(),
            Color::DarkGreen => "32".into(),
            Color::DarkAqua => "36".into(),
            Color::DarkRed => "31".into(),
            Color::DarkPurple => "35".into(),
            Color::Gold => "33".into(),
            Color::Gray => "90".into(),
            Color::DarkGray => "90".into(),
            Color::Blue => "94".into(),
            Color::Green => "92".into(),
            Color::Aqua => "96".into(),
            Color::Red => "91".into(),
            Color::LightPurple => "95".into(),
            Color::Yellow => "93".into(),
            Color::White => "97".into(),
        }
    }

    pub fn to_terminal(&self) -> String {
        //format!("\\e[38;5;{}m", self.to_terminal_code())
        format!("\x1b[0;{}m", self.to_terminal_code())
    }

    pub fn to_godot_tag(&self) -> Cow<'static, str> {
        match *self {
            Color::Reset => "[/color]".into(),
            Color::Black => "[color=black]".into(),
            Color::DarkBlue => "[color=dark_blue]".into(),
            Color::DarkGreen => "[color=dark_green]".into(),
            Color::DarkAqua => "[color=dark_aqua]".into(),
            Color::DarkRed => "[color=dark_red]".into(),
            Color::DarkPurple => "[color=dark_purple]".into(),
            Color::Gold => "[color=gold]".into(),
            Color::Gray => "[color=gray]".into(),
            Color::DarkGray => "[color=dark_gray]".into(),
            Color::Blue => "[color=blue]".into(),
            Color::Green => "[color=green]".into(),
            Color::Aqua => "[color=aqua]".into(),
            Color::Red => "[color=red]".into(),
            Color::LightPurple => "[color=light_purple]".into(),
            Color::Yellow => "[color=yellow]".into(),
            Color::White => "[color=white]".into(),
        }
    }
}

pub fn parse_to_terminal_colors(origin: &String) -> String {
    let mut result = origin.clone();
    let re = regex::Regex::new(COLORS_REGEX).unwrap();

    let mut offset = 0;
    for c in re.find_iter(&origin) {
        if c.start() + offset >= 1 {
            let pre = result.as_bytes()[c.start() - 1 + offset] as char;
            if pre == '\\' {
                result.remove(c.start() - 1 + offset);
                offset -= 1;
                continue;
            }
        }

        let replace_str = match Color::from_str(c.as_str()) {
            Some(c) => c.to_terminal(),
            None => continue,
        };
        result.replace_range(c.start() + offset..c.end() + offset, &replace_str);
        offset += replace_str.len() - c.as_str().len();
    }
    return format!("{}{}", result, Color::Reset.to_terminal());
}

pub fn parse_to_console_godot(origin: &String) -> String {
    let mut result = origin.clone();
    let re = regex::Regex::new(COLORS_REGEX).unwrap();

    let mut offset = 0;
    for c in re.find_iter(&origin) {
        if c.start() + offset >= 1 {
            let pre = result.as_bytes()[c.start() - 1 + offset] as char;
            if pre == '\\' {
                result.remove(c.start() - 1 + offset);
                offset -= 1;
                continue;
            }
        }

        let replace_str = match Color::from_str(c.as_str()) {
            Some(c) => c.to_godot_tag(),
            None => continue,
        };
        result.replace_range(c.start() + offset..c.end() + offset, &replace_str);
        offset += replace_str.len() - c.as_str().len();
    }
    if result.find("[color").is_some() {
        result = format!("{}{}", result, Color::Reset.to_godot_tag());
    }
    result
}

pub fn get_log_level_color(level: &Level) -> Cow<'static, str> {
    match level {
        Level::Error => "&c".into(),
        Level::Warn => "&6".into(),
        Level::Info => "&a".into(),
        Level::Debug => "&e".into(),
        Level::Trace => "&8".into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::colors::parse_to_console_godot;

    use super::parse_to_terminal_colors;

    #[test]
    fn test_terminal_colors() {
        let r = parse_to_terminal_colors(&"&5magenta_blue-&1_skeep-\\&2_gold-&6_red-&4_test".to_string());
        assert_eq!(
            r,
            "\u{1b}[0;35mmagenta_blue-\u{1b}[0;34m_skeep-&2_gold-\u{1b}[0;33m_red-\u{1b}[0;31m_test\u{1b}[0;0m"
                .to_string()
        );
    }

    #[test]
    fn test_to_godot() {
        let r = parse_to_console_godot(&"time: &8main &aINFO&r: text".to_string());
        assert_eq!(
            r,
            "time: [color=dark_gray]main [color=green]INFO[/color]: text[/color]".to_string()
        );
    }
}
