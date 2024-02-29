mod arbitrary;
mod display;
mod resolver;
pub use self::arbitrary::TailwindArbitrary;

use crate::*;
use css_color::Srgb;
use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};
use tailwind_ast::{parse_fraction, ASTVariant, AstStyle};

/// `v:v:-a-a-[A]`
#[derive(Debug, Clone)]
pub struct TailwindInstruction {
    negative: Negative,
    variants: Vec<TailwindVariant>,
    elements: TailwindElements,
    arbitrary: TailwindArbitrary,
}

#[derive(Debug, Clone)]
pub struct TailwindVariant {
    not: bool,
    pseudo: bool,
    names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TailwindElements {
    inner: Vec<String>,
}

/// <https://github.com/tw-in-js/twind/blob/main/src/twind/variants.ts>
#[derive(Copy, Clone, Debug)]
pub enum TailwindVariantKind {
    Dark,
    Sticky,
    MotionReduce,
    MotionSafe,
    First,
    Last,
    Even,
    Odd,
    Children,
    Siblings,
    Sibling,
    Override,
}

impl<'a> From<AstStyle<'a>> for TailwindInstruction {
    fn from(node: AstStyle<'a>) -> Self {
        Self {
            negative: Negative::from(node.negative),
            variants: node.variants.into_iter().map(|s| s.into()).collect(),
            elements: TailwindElements {
                inner: node.elements.into_iter().map(|s| s.to_string()).collect(),
            },
            arbitrary: TailwindArbitrary::from(node.arbitrary.unwrap_or_default()),
        }
    }
}

impl<'a> From<ASTVariant<'a>> for TailwindVariant {
    fn from(node: ASTVariant<'a>) -> Self {
        Self {
            not: node.not,
            pseudo: node.pseudo,
            names: node.names.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TailwindInstruction {
    #[inline]
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.inner.iter().map(|s| s.as_str()).collect()
    }
    #[inline]
    pub fn view_arbitrary(&self) -> &TailwindArbitrary {
        &self.arbitrary
    }
    // TODO
    pub fn normalization(self) -> Self {
        self
    }
}
