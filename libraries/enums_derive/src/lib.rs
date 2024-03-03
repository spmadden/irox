// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![no_std]

extern crate alloc;

use alloc::string::ToString;
use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error};

fn compile_error<T: Spanned>(span: &T, msg: &'static str) -> TokenStream {
    Error::new(span.span(), msg).into_compile_error().into()
}

#[proc_macro_derive(EnumName)]
pub fn enumname_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(s) = input.data else {
        return compile_error(&input, "Unsupported non-struct type found");
    };
    let enum_name = input.ident;

    let mut match_elements = alloc::vec::Vec::new();
    let mut names = alloc::vec::Vec::new();

    for field in s.variants {
        // println!("{}: {:?}", field.ident, field);
        let field_ident = field.ident;
        let field_name = field_ident.to_string();

        let inner_fields = match field.fields.len() {
            0 => quote! {},
            _ => quote! {(..)},
        };

        names.push(quote_spanned! {field_ident.span() =>
           #field_name
        });
        match_elements.push(quote_spanned! {field_ident.span() =>
            Self::#field_ident #inner_fields => #field_name,
        });
    }
    let res = quote! {
        impl irox_enums::EnumName for #enum_name {
            #[must_use]
            fn name(&self) -> &'static str {
                match self {
                    #( #match_elements )*
                }
            }
        }

        impl #enum_name {
            pub fn iter_names() -> impl Iterator<Item=&'static str>{
                extern crate alloc;
                let names = alloc::vec![#(#names),*];
                names.into_iter()
            }
        }
    };
    // println!("{}", res);
    proc_macro::TokenStream::from(res)
}

#[proc_macro_derive(EnumIterItem)]
pub fn enumitemiter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(s) = input.data else {
        return compile_error(&input, "Required enum type");
    };
    let enum_name = input.ident;

    let mut items = alloc::vec::Vec::new();

    for field in s.variants {
        if !field.fields.is_empty() {
            return compile_error(&field.span(), "Field cannot have fields");
        }

        let field_ident = field.ident;
        items.push(quote_spanned! {field_ident.span() =>
            Self::#field_ident
        });
    }
    let res = quote! {
        impl irox_enums::EnumIterItem for #enum_name {
            type Item = #enum_name;
            fn iter_items() -> irox_enums::IntoIter<Self::Item> {
                extern crate alloc;
                let items = alloc::vec![#(#items),*];
                items.into_iter()
            }
        }
    };
    // println!("{}", res);
    proc_macro::TokenStream::from(res)
}

#[proc_macro_derive(EnumTryFromStr)]
pub fn tryfromstr_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(s) = input.data else {
        return compile_error(&input, "Required enum type");
    };
    let enum_name = input.ident;

    let mut match_elements = alloc::vec::Vec::new();

    for field in s.variants {
        if !field.fields.is_empty() {
            return compile_error(&field.span(), "Field cannot have fields");
        }

        let field_ident = field.ident;
        let field_name = field_ident.to_string();

        match_elements.push(quote_spanned! {field_ident.span() =>
            #field_name => Self::#field_ident,
        });
    }

    let res = quote! {
        impl TryFrom<&str> for #enum_name {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Ok(match value {
                    #( #match_elements )*
                    _ => {return Err(())}
                })
            }
        }
    };
    // println!("{}", res);
    proc_macro::TokenStream::from(res)
}
