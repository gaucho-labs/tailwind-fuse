use darling::{ast, util::Flag, FromDeriveInput, FromVariant};

// used to get enum data.
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tw), supports(enum_unit))]
pub struct TwVariantContainer {
    pub ident: syn::Ident,
    pub data: ast::Data<TwVariantOption, ()>,
    /// The base tailwind class for the variant.
    pub class: Option<String>,
}

// For each enum option
#[derive(Debug, FromVariant)]
#[darling(supports(unit), attributes(tw, default))]
pub struct TwVariantOption {
    pub ident: syn::Ident,
    pub class: String,
    pub default: Flag,
}
