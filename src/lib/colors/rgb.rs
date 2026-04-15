#[derive(Debug)]
pub struct Rgb<'r, 'g, 'b> {
    pub r: &'r u8,
    pub g: &'g u8,
    pub b: &'b u8,
}
