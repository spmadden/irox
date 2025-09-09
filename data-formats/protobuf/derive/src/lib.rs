// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Derive helpers for the protobuf module
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use irox_derive_helpers::{compile_error, DeriveMethods};
use proc_macro::{Literal, TokenStream};
use syn::__private::ToTokens;
use syn::{Data, Fields};

#[derive(Debug)]
struct FieldInfo {
    name: String,
    id: u64,
}

fn create_binwrite_fn(fields: &[FieldInfo]) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.add_ident("fn");
    ts.add_ident("write_to");
    ts.add_generics(
        "T",
        TokenStream::create_path(&["irox_protobuf", "irox_bits", "MutBits"]),
    );
    ts.add_parens({
        let mut ts = TokenStream::new();
        ts.extend(TokenStream::create_ref_ident("self"));
        ts.add_comma();
        ts.add_ident("output");
        ts.add_punc(':');
        ts.extend(TokenStream::create_mut_ref_ident("T"));
        ts
    });
    ts.return_result(
        TokenStream::create_path(&["usize"]),
        TokenStream::create_path(&["irox_protobuf", "irox_bits", "BitsError"]),
    );
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("let");
        ts.add_ident("mut");
        ts.add_ident("out");
        ts.add_punc('=');
        ts.add_literal(Literal::usize_suffixed(0));
        ts.add_punc(';');

        ts.add_ident("let");
        ts.add_ident("mut");
        ts.add_ident("msg");
        ts.add_punc('=');
        ts.add_path(&["irox_protobuf", "message", "ProtoMessage", "default"]);
        ts.extend(TokenStream::create_empty_type());
        ts.add_punc(';');

        for field in fields {
            ts.add_ident("msg");
            ts.add_punc('.');
            ts.add_ident("fields");
            ts.add_punc('.');
            ts.add_ident("push");
            ts.add_parens({
                let mut ts = TokenStream::new();
                ts.add_path(&[
                    "irox_protobuf",
                    "message",
                    "ToProtoFieldData",
                    "to_proto_field",
                ]);
                ts.add_parens({
                    let mut ts = TokenStream::new();
                    ts.add_punc('&');
                    ts.add_ident("self");
                    ts.add_punc('.');
                    ts.add_ident(&field.name);
                    ts.add_punc(',');
                    ts.add_literal(Literal::string(&field.name));
                    ts.add_punc(',');
                    ts.add_literal(Literal::u64_unsuffixed(field.id));
                    ts
                });
                ts
            });
            ts.add_punc(';');
        }
        ts.add_ident("out");
        ts.add_punc2('+', '=');
        ts.add_ident("msg");
        ts.add_punc('.');
        ts.add_ident("write_be_to");
        ts.add_parens(TokenStream::create_ident("output"));
        ts.add_punc2('?', ';');

        ts.add_ident("Ok");
        ts.add_parens(TokenStream::create_ident("out"));
        // ts.add_todo();
        ts
    });

    ts
}
fn create_binread_fn() -> TokenStream {
    let mut ts = TokenStream::new();
    ts.add_ident("fn");
    ts.add_ident("read_from");

    ts.add_generics(
        "T",
        TokenStream::create_path(&["irox_protobuf", "irox_bits", "Bits"]),
    );
    ts.add_parens({
        let mut ts = TokenStream::new();
        ts.add_ident("input");
        ts.add_punc(':');
        ts.extend(TokenStream::create_mut_ref_ident("T"));
        ts
    });
    ts.return_result(
        TokenStream::create_ident("Self"),
        TokenStream::create_path(&["irox_protobuf", "irox_bits", "BitsError"]),
    );
    ts.add_where_self_sized();
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_todo();
        ts
    });

    ts
}
#[proc_macro_derive(ProtobufBinary, attributes(id, ignore))]
pub fn protobinary_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &input.ident;

    let Data::Struct(s) = input.data else {
        return compile_error(&input, "Can only derive on struct type");
    };
    let Fields::Named(n) = s.fields else {
        return compile_error(&s.fields, "Can only derive on named fields.");
    };
    let mut fields = Vec::new();
    for x in &n.named {
        let Some(ident) = &x.ident else {
            return irox_derive_helpers::compile_error(&x, "No ident");
        };
        for attr in &x.attrs {
            let Ok(_ident) = attr.meta.path().require_ident() else {
                return irox_derive_helpers::compile_error(
                    &attr,
                    "This attribute is unnamed.".to_string(),
                );
            };
            let Ok(nameval) = attr.meta.require_name_value() else {
                return irox_derive_helpers::compile_error(
                    &attr,
                    "This attribute is not a name value pair.".to_string(),
                );
            };
            let Ok(name) = nameval.path.require_ident() else {
                return irox_derive_helpers::compile_error(
                    &nameval.path,
                    "This attribute is not a name value pair.".to_string(),
                );
            };
            let val = nameval.value.to_token_stream().to_string();
            let name = name.to_token_stream().to_string();
            if name == "id" {
                let Ok(id) = val.parse::<u64>() else {
                    return irox_derive_helpers::compile_error(
                        &nameval.path,
                        "The value of ID must be an integer".to_string(),
                    );
                };
                fields.push(FieldInfo {
                    name: ident.to_token_stream().to_string(),
                    id,
                });
            }
        }
    }
    let mut ts = TokenStream::new();

    ts.add_ident("impl");
    ts.add_path(&["irox_protobuf", "ProtobufBinary"]);
    ts.add_ident("for");
    ts.add_ident(&struct_name.to_string());
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.extend(create_binwrite_fn(&fields));
        ts.extend(create_binread_fn());
        ts
    });
    ts
}
