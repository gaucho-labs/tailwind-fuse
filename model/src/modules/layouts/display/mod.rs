use super::*;

#[derive(Clone, Debug)]
pub struct TailwindDisplay {
    kind: &'static str,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/display#syntax
crate::macros::keyword_instance!(TailwindDisplay => "display", [
    "block",
    "inline-block",
    "inline",
    "flex",
    "inline-flex",
    "table",
    "inline-table",
    "table-caption",
    "table-cell",
    "table-column",
    "table-column-group",
    "table-footer-group",
    "table-header-group",
    "table-row-group",
    "table-row",
    "flow-root",
    "grid",
    "inline-grid",
    "contents",
    "list-item",
    "hidden",
]);

impl TailwindDisplay {}
