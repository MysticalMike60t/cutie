use crate::colors::ansi::ansi16::ColorCode;

pub struct Prefix<'color_code, 'display_text> {
    color_code: ColorCode<'color_code>,
    display_text: &'display_text str,
}

impl<'rgb, 'display_text> Prefix<'rgb, 'display_text> {
    pub fn new(display_text: &'display_text str) -> Self {
        Self {
            color_code: Self::process_inputted_color_code(display_text),
            display_text: Self::process_inputted_type(display_text),
        }
    }
    pub fn insert<'output>(self) -> &'output str {
        let ansi: String = format!(
            "\x1b[90m<\x1b[0m \x1b[{}m{}\x1b[0m \x1b[90m>\x1b[0m",
            self.color_code.insert(),
            self.display_text
        );
        return (ansi.clone()).leak();
    }
    pub fn process_inputted_type<'output>(display_text: &'display_text str) -> &'output str {
        match display_text.to_lowercase().as_str() {
            "debug"   => return "DEBUG  ",
            "success" => return "SUCCESS",
            "warning" => return "WARNING",
            "error"   => return "ERROR  ",
            "info"    => return "INFO   ",
            "output"  => return "OUTPUT ",
            &_ => return "None"
        }
    }
    pub fn process_inputted_color_code<'output>(display_text: &'display_text str) -> ColorCode<'output> {
        let i: &u8;
        match display_text.to_lowercase().as_str() {
            "success" => i = &92,
            &_        => i = &90
        }
        let rgb: ColorCode<'output> = ColorCode::new(&i);
        return rgb;
    }
}
