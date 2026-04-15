#[cfg(feature = "ansi24bit")]
use crate::colors::rgb::Rgb;

#[cfg(feature = "ansi24bit")]
pub struct Prefix<'rgb, 'display_text> {
    rgb: Rgb<'rgb, 'rgb, 'rgb>,
    display_text: &'display_text str,
}

#[cfg(feature = "ansi24bit")]
impl<'rgb, 'display_text> Prefix<'rgb, 'display_text> {
    pub fn new(display_text: &'display_text str) -> Self {
        Self {
            rgb: Self::return_rgb(display_text),
            display_text: Self::return_type(display_text),
        }
    }
    pub fn insert<'output>(self) -> &'output str {
        let ansi: String = format!(
            "\x1b[38;2;60;60;60m<\x1b[0m \x1b[38;2;{}m{}\x1b[0m \x1b[38;2;60;60;60m>\x1b[0m",
            self.rgb.insert(),
            self.display_text
        );
        return (ansi.clone()).leak();
    }
    pub fn return_type<'output>(display_text: &'display_text str) -> &'output str {
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
    pub fn return_rgb<'output>(display_text: &'display_text str) -> Rgb<'output, 'output, 'output> {
        let r: &u8;
        let g: &u8;
        let b: &u8;
        match display_text.to_lowercase().as_str() {
            "debug"   => {
                r = &200;
                g = &100;
                b = &240;
            },
            "success" => {
                r = &100;
                g = &240;
                b = &100;
            },
            "warning" => {
                r = &240;
                g = &240;
                b = &100;
            },
            "error" => {
                r = &240;
                g = &100;
                b = &100;
            },
            "info" => {
                r = &100;
                g = &140;
                b = &240;
            },
            "output" => {
                r = &100;
                g = &100;
                b = &100;
            },
            &_        => {
                r = &60 ;
                g = &60 ;
                b = &60 ;
            }
        }
        let rgb: Rgb<'output, 'output, 'output> = Rgb::new(&r, &g, &b);
        return rgb;
    }
}
