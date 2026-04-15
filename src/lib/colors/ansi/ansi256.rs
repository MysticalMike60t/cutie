use crate::colors::rgb::Rgb;

impl<'r, 'g, 'b> Rgb<'r, 'g, 'b> {
    pub fn insert<'s>(self) -> &'s str {
        let ansi: String = format!("{};{};{}", self.r, self.g, self.b);
        return (ansi.clone()).leak();
    }
}
