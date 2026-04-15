#[cfg(feature = "ansi256")]
use crate::colors::rgb::Rgb;

// impl<'r, 'g, 'b> AnsiIndex<'r, 'g, 'b> {
//     pub fn new(r: &'r u8, g: &'g u8, b: &'b u8) -> Self {
//         Self { r: r, g: g, b: b }
//     }
//     pub fn set_rgb(self, r: &'r u8, g: &'g u8, b: &'b u8) -> Rgb<'r, 'g, 'b> {
//         let rgb: Rgb<'r, 'g, 'b> = Rgb::new(&r, &g, &b);
//         return rgb;
//     }
//     pub fn insert<'s>(self) -> &'s str {
//         let ansi: String = format!("{}", self.to_ansi256());
//         println!("impl RGB > Insert > ansi_string: {:#?}", ansi);
//         return (ansi.clone()).leak();
//     }
//     pub fn to_ansi256(self) -> u8 {
//         use crate::colors::conv::rgb_to_ansi256;
//         return rgb_to_ansi256(&self)
//     }
// }
