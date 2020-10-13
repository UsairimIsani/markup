use std::fmt;
use std::io::Write;
pub struct Heading {
    content: ColoredText,
}
impl Heading {
    pub fn new(content: &str) -> Self {
        use termcolor::Color::Red;
        let content = ColoredText::new(Red, content);
        Self { content }
    }
}
impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColoredText {
    inner: String,
}
impl ColoredText {
    pub fn new(color: termcolor::Color, text: impl AsRef<str>) -> Self {
        use termcolor::{Buffer, ColorSpec, WriteColor};
        let mut buffer = Buffer::ansi();
        buffer
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
        buffer.write_all(text.as_ref().as_bytes()).unwrap();
        buffer.reset().unwrap();

        Self {
            inner: String::from_utf8_lossy(buffer.as_slice()).into_owned(),
        }
    }

    pub fn get_ref(&self) -> &String {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut String {
        &mut self.inner
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.inner.into_bytes()
    }

    pub fn into_string(self) -> String {
        self.inner
    }
}
impl fmt::Display for ColoredText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}
impl AsRef<[u8]> for ColoredText {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}
impl AsRef<str> for ColoredText {
    fn as_ref(&self) -> &str {
        self.inner.as_str()
    }
}
