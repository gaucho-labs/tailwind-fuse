use super::*;

pub use self::{
    justify_content::TailwindJustifyContent, justify_item::TailwindJustifyItems,
    justify_self::TailwindJustifySelf,
};

mod justify_content;
mod justify_item;
mod justify_self;

pub(crate) fn justify_adaptor(str: &[&str]) -> Result<Box<dyn TailwindInstance>> {
    let out = match str {
        // https://tailwindcss.com/docs/justify-items
        ["items", rest @ ..] => TailwindJustifyItems::try_from(rest)?.boxed(),
        // https://tailwindcss.com/docs/justify-self
        ["self", rest @ ..] => TailwindJustifySelf::try_from(rest)?.boxed(),
        // https://tailwindcss.com/docs/justify-content
        _ => TailwindJustifyContent::try_from(str)?.boxed(),
    };
    Ok(out)
}
