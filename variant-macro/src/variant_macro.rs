use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::model::TwVariantContainer;

pub fn variant_impl(input: TokenStream) -> TokenStream {
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

    let defaults = variants
        .iter()
        .filter(|v| v.default.is_present())
        .collect::<Vec<_>>();

    if defaults.is_empty() {
        return syn::Error::new_spanned(
            input,
            "No default variant specified. Please mark one variant with `#[tw(default)]`",
        )
        .to_compile_error()
        .into();
    }

    if defaults.len() > 1 {
        let error = format!(
            "Only one variant can be marked as default: {:?}",
            defaults
                .iter()
                .map(|v| v.ident.to_string())
                .collect::<Vec<_>>()
        );
        return syn::Error::new_spanned(input, error)
            .to_compile_error()
            .into();
    }

    let default_variant = defaults.into_iter().next().map(|v| {
        let variant_ident = &v.ident;
        quote! {
            impl Default for #enum_ident {
                fn default() -> Self {
                    #enum_ident::#variant_ident
                }
            }
        }
    });

    let gen = quote! {
        impl IntoTailwindClass for #enum_ident {
            fn to_class(&self) -> String {
                self.with_class("")
            }
            fn with_class(&self, class: impl AsRef<str>) -> String {
                let variant_class = match self {
                    #( #to_class_cases )*
                };
                tw_join!(#base_class, variant_class, class.as_ref())
            }
        }

        #default_variant
    };

    gen.into()
}
