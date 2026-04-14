pub struct Rgb<'r, 'g, 'b> {
    r: &'r u8,
    g: &'g u8,
    b: &'b u8,
}

impl<'r, 'g, 'b> Rgb<'r, 'g, 'b> {
    pub fn new(r: &'r u8, g: &'g u8, b: &'b u8) -> Self {
        Self { r: r, g: g, b: b }
    }
    pub fn insert<'s>(self) -> &'s str {
        let ansi: String = format!("{};{};{}", self.r, self.g, self.b);
        return (ansi.clone()).leak();
    }
    pub fn set_rgb(self, r: &'r u8, g: &'g u8, b: &'b u8) -> Rgb<'r, 'g, 'b> {
        let rgb: Rgb<'r, 'g, 'b> = Rgb::new(&r, &g, &b);
        return rgb;
    }
}
