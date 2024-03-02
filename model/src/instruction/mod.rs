use crate::*;
use tailwind_ast::AstStyle;

mod parser;

/// `v:v:-a-a-[A]`
#[derive(Debug, Clone)]
pub struct TailwindInstruction<'a> {
    ast: AstStyle<'a>,
}

impl<'a> From<AstStyle<'a>> for TailwindInstruction<'a> {
    fn from(ast: AstStyle<'a>) -> Self {
        Self { ast }
    }
}

impl<'a> TailwindInstruction<'a> {
    #[inline]
    pub fn view_elements(&self) -> &[&str] {
        self.ast.elements.as_slice()
    }

    #[inline]
    pub fn view_arbitrary(&self) -> TailwindArbitrary {
        TailwindArbitrary::new(self.ast.arbitrary.unwrap_or_default())
    }

    pub fn negative(&self) -> Negative {
        Negative::from(self.ast.negative)
    }
}
