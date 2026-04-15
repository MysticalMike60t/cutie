use crate::colors::ansi::ansi16::ColorCode;

pub struct Prefix<'color_code, 'display_text> {
    color_code: ColorCode<'color_code>,
    display_text: &'display_text str,
}

impl<'rgb, 'display_text> Prefix<'rgb, 'display_text> {
    pub fn new(display_text: &'display_text str) -> Self {
        Self {
            color_code: Self::return_color_code(display_text),
            display_text: Self::return_type(display_text),
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
    pub fn return_type<'output>(display_text: &'display_text str) -> &'output str {
        if display_text.to_lowercase().as_str().contains("success") {
            return "SUCCESS";
        } else {
            return "NONE";
        }
    }
    pub fn return_color_code<'output>(display_text: &'display_text str) -> ColorCode<'output> {
        let i: &u8;
        if display_text.to_lowercase().as_str().contains("success") {
            i = &92;
        } else {
            i = &90;
        }
        let rgb: ColorCode<'output> = ColorCode::new(&i);
        return rgb;
    }
}
