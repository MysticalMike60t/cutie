use crate::colors::rgb::Rgb;

pub struct Prefix<'rgb, 'display_text> {
    rgb: Rgb<'rgb, 'rgb, 'rgb>,
    display_text: &'display_text str,
}

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
        if display_text.to_lowercase().as_str().contains("success") {
            return "SUCCESS";
        } else {
            return "NONE";
        }
    }
    pub fn return_rgb<'output>(display_text: &'display_text str) -> Rgb<'output, 'output, 'output> {
        let r: &u8;
        let g: &u8;
        let b: &u8;
        if display_text.to_lowercase().as_str().contains("success") {
            r = &140;
            g = &240;
            b = &140;
        } else {
            r = &60;
            g = &60;
            b = &60;
        }
        let rgb: Rgb<'output, 'output, 'output> = Rgb::new(&r, &g, &b);
        return rgb;
    }
}
