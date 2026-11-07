// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::fields::{parse_fields_in_struct, FieldOpts, ParseFieldsResult};
use crate::{compile_error, get_table_attr_value};
use irox_derive_helpers::syn::{parse_macro_input, Data, DeriveInput, Fields};
use irox_derive_helpers::DeriveMethods;
use proc_macro::TokenStream;

pub fn list(ts: &mut TokenStream, query: &str, enum_name: &str, all_fields: &[FieldOpts]) {
    ts.add_ident("impl");
    ts.extend(TokenStream::create_path(&["irox_postgres", "PostgresList"]));
    ts.add_ident("for");
    ts.add_ident(enum_name);
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("type");
        ts.add_ident("Item");
        ts.add_punc('=');
        ts.add_ident(enum_name);
        ts.add_punc(';');
        // ts.add_ident("pub");
        ts.add_ident("async");
        ts.add_ident("fn");
        ts.add_ident("list");
        ts.add_parens({
            let mut ts = TokenStream::new();
            ts.add_ident("conn");
            ts.add_punc(':');
            ts.extend(TokenStream::create_mut_ref_ident("DbConn"));
            ts
        });
        ts.return_result(
            {
                let mut ts = TokenStream::new();
                ts.add_path(&["std", "vec", "Vec"]);
                ts.wrap_generics(TokenStream::create_path(&["Self", "Item"]));
                ts
            },
            TokenStream::create_path(&["crate", "error", "Error"]),
        );
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("let");
            ts.add_ident("res");
            ts.add_punc('=');
            ts.extend(TokenStream::create_callchain(&["conn", "client", "query"]));
            ts.add_parens({
                let mut ts = TokenStream::new();
                ts.extend(TokenStream::create_literal(query));
                ts.add_comma();
                ts.add_punc('&');
                ts.wrap_brackets(TokenStream::new());
                ts
            });
            ts.add_punc('.');
            ts.add_ident("await");
            ts.add_punc('?');
            ts.add_punc(';');

            ts.add_ident("let");
            ts.add_ident("mut");
            ts.add_ident("out");
            ts.add_punc('=');
            ts.add_path(&["std", "vec", "Vec", "new"]);
            ts.extend(TokenStream::create_empty_type());
            ts.add_punc(';');

            ts.add_ident("for");
            ts.add_ident("row");
            ts.add_ident("in");
            ts.add_ident("res");
            ts.wrap_braces({
                let mut ts = TokenStream::new();
                ts.extend(TokenStream::create_callchain(&["out", "push"]));
                ts.add_parens({
                    let mut ts = TokenStream::new();
                    ts.add_ident(enum_name);
                    ts.wrap_braces({
                        let mut ts = TokenStream::new();
                        for field in all_fields {
                            ts.add_ident(&field.name);
                            ts.add_punc(':');
                            ts.extend(TokenStream::create_callchain(&["row", "get"]));
                            ts.add_parens(TokenStream::create_literal(&field.column_name));
                            ts.add_comma();
                        }
                        ts
                    });
                    ts
                });
                ts
            });

            ts.add_ident("Ok");
            ts.add_parens(TokenStream::create_ident("out"));
            ts
        });
        ts
    });
}
pub fn run_list(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let table_name: Option<String> = get_table_attr_value(&input, "table_name");
    let query: Option<String> = get_table_attr_value(&input, "query");

    let query = if let Some(q) = query {
        q
    } else {
        let Some(table_name) = table_name else {
            return compile_error(&input, "table_name attribute must be present");
        };
        format!("select * from \"{table_name}\"")
    };
    let Data::Struct(s) = &input.data else {
        return compile_error(&input, "Unsupported non-struct type found");
    };

    let Fields::Named(named) = &s.fields else {
        return compile_error(&input, "can only derived on named fields");
    };

    let ParseFieldsResult {
        all_fields,
        all_not_unique: _all_not_unique,
        uniques: _uniques,
    } = match parse_fields_in_struct(named) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let enum_name = input.ident;
    let mut out = TokenStream::new();

    list(&mut out, &query, &enum_name.to_string(), &all_fields);
    out
}
