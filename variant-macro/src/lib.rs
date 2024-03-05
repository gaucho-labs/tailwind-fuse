extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::model::TwVariantContainer;

mod model;

#[proc_macro_derive(TwVariant, attributes(tw))]
pub fn variant(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let container = match TwVariantContainer::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let enum_ident = &container.ident;

    let variants = container.data.take_enum().unwrap_or_else(Vec::new);

    let base_class = container.class.unwrap_or_default();

    let to_class_cases = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_class = &variant.class;

        quote! {
            #enum_ident::#variant_ident =>  #variant_class,
        }
    });

    let gen = quote! {
        impl tw_utils::ToTailwindClass for #enum_ident {
            fn to_class(&self) -> String {
                self.with_class("")
            }
            fn with_class(&self, class: impl AsRef<str>) -> String {
                use tw_merge::MaybeToTailwindClass;
                let variant_class = match self {
                    #( #to_class_cases )*
                };
                tw_merge::tw_join!(#base_class, variant_class, class.as_ref())
            }
        }
    };

    gen.into()
}
