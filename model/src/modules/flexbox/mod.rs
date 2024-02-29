use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{
    syntax_error, LengthUnit, Negative, NumericValue, Result, StandardValue, TailwindArbitrary,
    TailwindDisplay, TailwindInstance,
};

pub use self::{
    basis::TailwindBasis,
    content::{content_align::TailwindContentAlign, TailwindContent},
    flex::{flex_direction::TailwindFlexDirection, flex_wrap::TailwindFlexWrap, TailwindFlex},
    gap::TailwindGap,
    grid::{
        grid_auto::TailwindGridAuto, grid_cols::TailwindGridColumns, grid_flow::TailwindGridFlow,
        grid_rows::TailwindGridRows, TailwindGrid,
    },
    grow::TailWindGrow,
    items::TailwindItems,
    justify::*,
    order::TailWindOrder,
    place::{
        place_content::TailwindPlaceContent, place_item::TailwindPlaceItems,
        place_self::TailwindPlaceSelf, TailwindPlace,
    },
    shrink::TailWindShrink,
    span::{TailwindColumn, TailwindRow},
    zelf::TailwindSelf,
};

mod basis;
mod content;
mod flex;
mod gap;
mod grid;
mod grow;
mod items;
mod justify;
mod order;
mod place;
mod shrink;
mod span;
mod zelf;
