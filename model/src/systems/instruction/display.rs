use super::*;

impl Display for TailwindInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in &self.variants {
            write!(f, "{}", v)?
        }
        self.negative.write(f)?;
        match self.arbitrary.is_some() {
            true => write!(f, "{}-{}", self.elements, self.arbitrary.get_class()),
            false => write!(f, "{}", self.elements),
        }
    }
}

impl Display for TailwindVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.not {
            write!(f, "not-")?
        }
        write!(f, "{}", self.names.join("-"))?;
        match self.pseudo {
            true => {
                write!(f, "::")
            }
            false => {
                write!(f, ":")
            }
        }
    }
}

impl Display for TailwindElements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.join("-"))
    }
}
