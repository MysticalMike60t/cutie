#[cfg(feature = "ansi24bit")]
use crate::colors::rgb::Rgb;

#[cfg(feature = "ansi24bit")]
impl<'r, 'g, 'b> Rgb<'r, 'g, 'b> {
    pub fn new(r: &'r u8, g: &'g u8, b: &'b u8) -> Self {
        Self { r: r, g: g, b: b }
    }
    pub fn set_rgb(self, r: &'r u8, g: &'g u8, b: &'b u8) -> Rgb<'r, 'g, 'b> {
        let rgb: Rgb<'r, 'g, 'b> = Rgb::new(&r, &g, &b);
        return rgb;
    }
    pub fn insert<'s>(self) -> &'s str {
        let ansi: String = format!("{};{};{}", self.r, self.g, self.b);
        // println!("impl RGB > Insert > ansi_string: {:#?}", ansi);
        return (ansi.clone()).leak();
    }
}
