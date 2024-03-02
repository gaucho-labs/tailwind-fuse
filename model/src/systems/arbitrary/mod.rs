use crate::*;
use std::fmt::Formatter;

mod methods;

#[derive(Debug, Clone)]
pub struct TailwindArbitrary<'a> {
    inner: &'a str,
}

impl TailwindArbitrary<'_> {
    pub fn new(inner: &str) -> TailwindArbitrary {
        TailwindArbitrary { inner }
    }
}

impl<'a> From<&'a str> for TailwindArbitrary<'a> {
    fn from(inner: &'a str) -> Self {
        TailwindArbitrary { inner }
    }
}

impl<'a> TailwindArbitrary<'a> {
    pub fn get_class(&self) -> String {
        let mut class = String::with_capacity(self.inner.len() + 2);
        class.push('[');
        for c in self.inner.chars() {
            match c {
                ' ' => class.push('_'),
                _ => class.push(c),
            }
        }
        class.push(']');
        class
    }
    pub fn write(&self, f: &mut Formatter) -> std::fmt::Result {
        self.write_class(f, "")
    }
    pub fn write_class(&self, f: &mut Formatter, before: &str) -> std::fmt::Result {
        write!(f, "{}{}", before, self.get_class())
    }
    pub fn get_properties(&self) -> String {
        self.inner.to_string()
    }
}
