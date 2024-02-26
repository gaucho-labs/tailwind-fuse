extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Lit, Meta, NestedMeta};

#[proc_macro_derive(TailwindVariant, attributes(class, default))]
pub fn tailwind_variant(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident; // The name of the enum

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

    // Combine everything into the final generated code
    let gen = quote! {
        #default_impl

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
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &input.ident;

    // Ensure we are dealing with a struct
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("TailwindClass can only be applied to structs"),
    };

    // Generate code for each field's class string
    let field_class_calls = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.to_class()
            }
        })
        .collect::<Vec<_>>();

    // Generate the implementation of the `to_class` method using the `tw!` macro
    let gen = quote! {
        impl #name {
            pub fn to_class(&self) -> String {
                use tw_class::IntoTailwindClass;
                tw_class::tw!(#(#field_class_calls),*)
            }
            pub fn with_class(&self, class: &str) -> String {
                use tw_class::IntoTailwindClass;
                tw_class::tw!(#(#field_class_calls),*, class)
            }
        }
    };

    // Return the generated code
    TokenStream::from(gen)
}
