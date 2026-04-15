#[cfg(feature = "ansi16")]
pub struct ColorCode<'i> {
    pub i: &'i u8,
}

#[cfg(feature = "ansi16")]
impl<'i> ColorCode<'i> {
    pub fn new(i: &'i u8) -> Self {
        Self { i: i }
    }
    pub fn insert<'s>(self) -> &'s str {
        let ansi: String = format!("{}", self.i);
        return (ansi.clone()).leak();
    }
}
