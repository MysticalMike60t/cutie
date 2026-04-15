pub struct Rgb<'r, 'g, 'b> {
    pub r: &'r u8,
    pub g: &'g u8,
    pub b: &'b u8,
}

impl<'r, 'g, 'b> Rgb<'r, 'g, 'b> {
    pub fn new(r: &'r u8, g: &'g u8, b: &'b u8) -> Self {
        Self { r: r, g: g, b: b }
    }
    pub fn set_rgb(self, r: &'r u8, g: &'g u8, b: &'b u8) -> Rgb<'r, 'g, 'b> {
        let rgb: Rgb<'r, 'g, 'b> = Rgb::new(&r, &g, &b);
        return rgb;
    }
}
