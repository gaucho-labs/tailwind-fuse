use crate::{Result, StandardValue, TailwindArbitrary, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

pub use self::{border_collapse::TailwindBorderCollapse, table_layout::TailwindTableLayout};

mod border_collapse;
mod table_layout;
