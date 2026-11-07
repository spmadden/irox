// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::fields::{parse_fields_in_struct, ParseFieldsResult};
use crate::{compile_error, get_table_attr_value};
use irox_derive_helpers::syn::{parse_macro_input, Data, DeriveInput, Fields};
use irox_derive_helpers::DeriveMethods;
use proc_macro::{Literal, TokenStream};

pub fn run_update(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let table_name: Option<String> = get_table_attr_value(&input, "table_name");
    let Some(table_name) = table_name else {
        return compile_error(&input, "table_name attribute must be present");
    };
    let Data::Struct(s) = &input.data else {
        return compile_error(&input, "Unsupported non-struct type found");
    };
    let Fields::Named(named) = &s.fields else {
        return compile_error(&input, "can only derived on named fields");
    };
    let ParseFieldsResult {
        all_fields,
        all_not_unique,
        uniques,
    } = match parse_fields_in_struct(named) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let enum_name = input.ident;
    let mut out = TokenStream::new();
    out.add_ident("impl");
    out.add_ident(&enum_name.to_string());
    out.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("pub");
        ts.add_ident("async");
        ts.add_ident("fn");
        ts.add_ident("update");
        ts.add_parens({
            let mut ts = TokenStream::new();
            ts.extend(TokenStream::create_ref_ident("self"));
            ts.add_punc(',');
            ts.extend(TokenStream::create_ident("conn"));
            ts.add_punc(':');
            ts.add_punc('&');
            ts.extend(TokenStream::create_path(&["crate", "connection", "DbConn"]));
            ts
        });
        ts.return_result(TokenStream::create_empty_type(), {
            let mut ts = TokenStream::new();
            ts.add_path(&["crate", "error", "Error"]);
            ts
        });
        // ts.extend(TokenStream::create_empty_type());
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("let");
            ts.add_ident("res");
            ts.add_punc('=');
            ts.extend(TokenStream::create_callchain(&["conn", "client", "query"]));
            ts.add_parens({
                // .query("query", &[args])
                let mut ts = TokenStream::new();
                let mut query = format!("insert into {table_name} (");
                query += all_fields
                    .iter()
                    .map(|v| v.column_name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
                    .as_str();
                query += ") VALUES (";
                query += all_fields
                    .iter()
                    .enumerate()
                    .map(|v| format!("${}", v.0 + 1))
                    .collect::<Vec<String>>()
                    .join(",")
                    .as_str();
                query += ")";
                if !uniques.is_empty() {
                    query += " ON CONFLICT (";
                    query += uniques
                        .iter()
                        .map(|v| v.column_name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                        .as_str();
                    query += ") DO UPDATE SET ";
                    query += all_not_unique
                        .iter()
                        .map(|v| {
                            if v.is_increment_on_update {
                                format!("{} = EXCLUDED.{} + 1", v.column_name, v.column_name)
                            } else {
                                format!("{} = EXCLUDED.{}", v.column_name, v.column_name)
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(", ")
                        .as_str();
                }
                query += ";";

                ts.add_literal(Literal::string(&query));
                ts.add_comma();
                ts.add_punc('&');
                ts.wrap_brackets({
                    let mut ts = TokenStream::new();
                    for field in &all_fields {
                        ts.add_punc('&');
                        ts.extend(TokenStream::create_callchain(&["self", &field.name]));
                        ts.add_punc(',');
                    }

                    ts
                });

                ts
            });
            ts.add_punc('.');
            ts.add_ident("await");
            ts.add_punc('?');
            ts.add_punc(';');

            ts.add_ident("Ok");
            ts.add_parens(TokenStream::create_empty_type());
            ts
        });
        ts
    });
    out
}
