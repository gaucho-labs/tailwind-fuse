pub(crate) use self::list::list_adaptor;
pub use self::{
    align::TailwindAlign,
    breaking::TailwindBreak,
    decoration::{
        color::TailwindDecorationColor, line::TailwindDecorationLine, style::TailwindDecorationStyle,
        thickness::TailwindDecorationThickness, TailwindDecoration,
    },
    font::{
        font_adaptor, font_family::TailwindFontFamily, font_size::TailwindFontSize, font_smoothing::TailwindFontSmoothing,
        font_style::TailwindFontStyle, font_variant_numeric::TailwindFontVariantNumeric, font_weight::TailwindFontWeight,
    },
    indent::TailwindIndent,
    leading::TailwindLeading,
    list::{list_position::TailwindListPosition, list_type::TailwindListStyle},
    text::{
        text_adaptor, text_align::TailwindTextAlignment, text_color::TailwindTextColor, text_overflow::TailwindTextOverflow,
        text_transform::TailwindTextTransform,
    },
    tracking::TailwindTracking,
    underline_offset::TailwindUnderlineOffset,
    whitespace::TailwindWhiteSpace,
};
use crate::{
    syntax_error, LengthUnit, Result, StandardValue, TailwindArbitrary, TailwindBreakAfter, TailwindBreakBefore,
    TailwindBreakInside, TailwindColor, TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

mod align;
mod breaking;
mod decoration;
mod font;
mod indent;
mod leading;
mod list;
mod text;
mod tracking;
mod underline_offset;
mod whitespace;
