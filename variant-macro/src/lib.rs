extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Field, FieldsNamed, Lit, Meta, NestedMeta,
};

#[proc_macro_derive(TailwindVariant, attributes(class, default))]
pub fn tailwind_variant(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // The name of the enum
    let name = &ast.ident;

    let variants = if let Data::Enum(data) = &ast.data {
        &data.variants
    } else {
        panic!("TailwindVariant can only be applied to enums");
    };

    // Extract variant arms for the to_class method and identify the default variant
    let mut default_variant_name = None;
    let variant_arms = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let class_attr = variant
                .attrs
                .iter()
                .find(|attr| attr.path.is_ident("class"))
                .expect("Missing `class` attribute");
            let class_value = extract_class_value(class_attr);

            if variant
                .attrs
                .iter()
                .any(|attr| attr.path.is_ident("default"))
            {
                default_variant_name = Some(variant_name.clone());
            }

            quote! {
                #name::#variant_name => #class_value,
            }
        })
        .collect::<Vec<_>>();

    // Ensure a default variant was specified
    let default_variant_name =
        default_variant_name.expect("No default variant found with #[default] attribute");

    // Generate the Default trait implementation
    let default_impl = quote! {
        impl Default for #name {
            fn default() -> Self {
                #name::#default_variant_name
            }
        }
    };

    // Generate the to_class method
    let to_class_impl = quote! {
        impl #name {
            pub fn to_class(&self) -> &'static str {
                match self {
                    #(#variant_arms)*
                }
            }
        }
    };

    let display_impl = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_class())
            }
        }
    };

    let gen = quote! {
        #default_impl

        #display_impl

        #to_class_impl
    };

    gen.into()
}

fn extract_class_value(attr: &Attribute) -> String {
    if let Ok(Meta::List(meta)) = attr.parse_meta() {
        if let NestedMeta::Lit(Lit::Str(lit)) = &meta.nested[0] {
            return lit.value();
        }
    }
    panic!("Expected string literal for `class` attribute")
}

#[proc_macro_derive(TailwindClass)]
pub fn tailwind_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named,
            _ => panic!("TailwindClass can only be applied to structs with named fields"),
        },
        _ => panic!("TailwindClass can only be applied to structs"),
    };

    let field_names = fields
        .iter()
        .map(|Field { ident, .. }| ident.as_ref().unwrap());

    let builder_fields = fields.iter().map(|field| {
        let Field { ident, ty, .. } = field;
        quote! { #ident: Option<#ty> }
    });

    let builder_set_methods = fields.iter().map(|field| {
        let Field { ident, ty, .. } = field;
        quote! {
            pub fn #ident(mut self, value: #ty) -> Self {
                self.#ident = Some(value);
                self
            }
        }
    });

    let builder_to_tailwind = {
        let optional_builder_fields = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.as_ref().unwrap_or(&Default::default()).to_class()
            }
        });

        quote! {
            impl tw_utils::ToTailwindClass for #builder_name{
                fn to_class(&self) -> String {
                    self.with_class("")
                }

                fn with_class(&self, class: impl AsRef<str>) -> String {
                    use tw_classnames::MaybeToTailwindClass;
                    tw_classnames::tw!(#(#optional_builder_fields),*, class.as_ref())
                }
            }
        }
    };

    let struct_to_tailwind = {
        let field_class_calls = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.to_class()
            }
        });

        quote! {
            impl tw_utils::ToTailwindClass for #struct_name {
                fn to_class(&self) -> String {
                    self.with_class("")
                }

                fn with_class(&self, class: impl AsRef<str>) -> String {
                    use tw_classnames::MaybeToTailwindClass;
                    tw_classnames::tw!(#(#field_class_calls),*, class.as_ref())
                }
            }
        }
    };

    let gen = quote! {
        impl #struct_name {
            pub fn variant() -> #builder_name {
                #builder_name {
                    #(#field_names: None,)*
                }
            }
        }

        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #(#builder_set_methods)*

        }
        #builder_to_tailwind

        #struct_to_tailwind
    };

    TokenStream::from(gen)
}
