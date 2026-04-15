use std::ops::Add;

use crate::colors::rgb::Rgb;

pub type AnsiIndex = u16;

pub fn rgb_to_ansi256(rgb: &Rgb) -> u8 {
    let r: f32 = (*rgb.r).into();
    let g: f32 = (*rgb.g).into();
    let b: f32 = (*rgb.b).into();
    let rgb_add: f32 = (r.to_owned() * 36.0) + (g.to_owned() * 6.0) + b.to_owned();
    let ansi_index: AnsiIndex = Add::<u16>::add(16, rgb_add.floor() as u16);
    return ansi_index.clone() as u8;
}
