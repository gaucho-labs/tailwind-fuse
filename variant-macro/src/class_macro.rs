use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::model::{TwClassContainer, TwClassField};

pub fn class_impl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let container = match TwClassContainer::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let struct_ident = &container.ident;
    let builder_ident = format_ident!("{struct_ident}Builder");

    let fields = container
        .data
        .take_struct()
        .expect("Expected struct fields");

    let base_class = container.class.unwrap_or_default();

    let merger = {
        if let Some(merger) = container.merger {
            let ident = merger.as_ident();
            quote! {#ident}
        } else {
            quote! {TailwindMerge}
        }
    };

    let builder_struct = {
        let builder_fields = fields.iter().map(|field| {
            let TwClassField { ident, ty, .. } = field;
            quote! { #ident: Option<#ty> }
        });

        quote! {
            pub struct #builder_ident {
                #(#builder_fields,)*
            }
        }
    };

    let field_idents = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone());

    let builder_impl = {
        let field_idents = field_idents.clone();

        let builder_set_methods = fields.iter().map(|field| {
            let TwClassField { ident, ty, .. } = field;
            quote! {
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = Some(value);
                    self
                }
            }
        });

        quote! {
            impl #builder_ident {
                #(#builder_set_methods)*

                pub fn build(self) -> #struct_ident {
                    #struct_ident {
                        #(#field_idents: self.#field_idents.unwrap_or_default(),)*
                    }
                }
            }
        }
    };

    let field_class_strings: Vec<(&Option<syn::Ident>, syn::Ident)> = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let field_name = &field.ident;
            let var_name =
                syn::Ident::new(&format!("class_str_{}", i), proc_macro2::Span::call_site());

            (field_name, var_name)
        })
        .collect::<Vec<_>>();

    let builder_to_tailwind = {
        let builder_variables = field_class_strings.iter().map(|(field_name, var_name)| {
            quote! {
                let #var_name = self.#field_name.unwrap_or_default();
            }
        });

        let builder_refs = field_class_strings.iter().map(|(_, var_name)| {
            quote! {
                #var_name.as_tailwind_class(),
            }
        });

        quote! {
            impl IntoTailwindClass for #builder_ident {
                fn to_class(&self) -> String {
                    self.with_class("")
                }

                fn with_class(&self, class: impl AsRef<str>) -> String {
                    #( #builder_variables )*
                    let classes = [
                        #base_class,
                        #( #builder_refs )*
                        class.as_ref(),
                    ];
                    #merger.fuse_classes(&classes)
                }
            }
        }
    };
    // We have code duplication here because we can't ensure that clone is implemented for all types.
    // First need to save the String to a local variable.

    let struct_to_tailwind = {
        let field_vars = field_class_strings.iter().map(|(field_name, var_name)| {
            quote! {
                let #var_name = self.#field_name.as_tailwind_class();
            }
        });

        let field_refs = field_class_strings.iter().map(|(_, var_name)| {
            quote! {
                #var_name,
            }
        });

        quote! {
            impl IntoTailwindClass for #struct_ident {
                fn to_class(&self) -> String {
                    self.with_class("")
                }

                fn with_class(&self, class: impl AsRef<str>) -> String {
                    #( #field_vars )*
                    let classes = [
                        #base_class,
                        #( #field_refs )*
                        class.as_ref(),
                    ];
                    #merger.fuse_classes(&classes)
                }
            }
        }
    };

    let builder_default = {
        let field_idents = field_idents.clone();

        quote! {
            impl Default for #builder_ident {
                fn default() -> Self {
                    #builder_ident {
                        #(#field_idents: None,)*
                    }
                }
            }
        }
    };

    let gen = quote! {
        impl IntoBuilder for #struct_ident {
            type Builder = #builder_ident;

            fn builder() -> Self::Builder {
                Default::default()
            }
            fn into_builder(self) -> Self::Builder {
                self.into()
            }
        }

        impl From<#struct_ident> for #builder_ident {
            fn from(value: #struct_ident) -> Self {
                #builder_ident {
                    #(#field_idents: Some(value.#field_idents),)*
                }
            }
        }

        impl From<#builder_ident> for #struct_ident {
            fn from(value: #builder_ident) -> Self {
                value.build()
            }
        }

        #builder_default

        #builder_struct

        #builder_impl

        #builder_to_tailwind

        #struct_to_tailwind
    };

    gen.into()
}
