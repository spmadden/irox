// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::fields::{parse_fields_in_struct, ParseFieldsResult};
use crate::{compile_error, get_table_attr_value};
use irox_derive_helpers::syn::{parse_macro_input, Data, DeriveInput, Fields};
use irox_derive_helpers::DeriveMethods;
use proc_macro::{Literal, TokenStream};
use std::fmt::Write;

pub fn run_findby(input: TokenStream) -> TokenStream {
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
        all_not_unique: _all_not_unique,
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
        ts.add_ident("findby");
        ts.add_parens({
            let mut ts = TokenStream::new();
            ts.extend(TokenStream::create_ident("conn"));
            ts.add_punc(':');
            ts.add_punc('&');
            ts.extend(TokenStream::create_path(&["crate", "connection", "DbConn"]));

            for field in &uniques {
                ts.add_comma();
                ts.extend(TokenStream::create_ident(&field.name));
                ts.add_punc(':');
                ts.add_ident("impl");
                ts.add_ident("Into");
                ts.wrap_generics(field.ty.clone());
            }

            ts
        });
        ts.return_result(
            {
                let mut ts = TokenStream::new();
                ts.add_ident("Option");
                ts.wrap_generics(TokenStream::create_ident("Self"));
                ts
            },
            {
                let mut ts = TokenStream::new();
                ts.add_path(&["crate", "error", "Error"]);
                ts
            },
        );
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("let");
            ts.add_ident("res");
            ts.add_punc('=');
            ts.extend(TokenStream::create_callchain(&["conn", "client", "query"]));
            ts.add_parens({
                // .query("query", &[args])
                let mut ts = TokenStream::new();
                let mut query = format!("select * from {table_name} where ");
                let mut need_and = false;
                for (idx, field) in uniques.iter().enumerate() {
                    if need_and {
                        query += " and ";
                    }
                    let _ = write!(&mut query, "\"{}\" = ${}", field.column_name, idx + 1);
                    need_and = true;
                }
                query += " limit 1;";

                ts.add_literal(Literal::string(&query));
                ts.add_comma();
                ts.add_punc('&');
                ts.wrap_brackets({
                    let mut ts = TokenStream::new();
                    for field in &uniques {
                        ts.add_punc('&');
                        ts.extend(TokenStream::create_callchain(&[&field.name, "into"]));
                        ts.extend(TokenStream::create_empty_type());
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

            ts.add_ident("if");
            ts.add_ident("let");
            ts.add_ident("Some");
            ts.add_parens(TokenStream::create_ident("res"));
            ts.add_punc('=');
            ts.extend(TokenStream::create_callchain(&["res", "first"]));
            ts.extend(TokenStream::create_empty_type());
            ts.wrap_braces({
                let mut ts = TokenStream::new();
                ts.add_ident("Ok");
                ts.add_parens({
                    let mut ts = TokenStream::new();
                    ts.add_ident("Some");
                    ts.add_parens({
                        let mut ts = TokenStream::new();
                        ts.add_ident("Self");
                        ts.wrap_braces({
                            let mut ts = TokenStream::new();
                            for field in &all_fields {
                                ts.add_ident(&field.name);
                                ts.add_punc(':');
                                ts.extend(TokenStream::create_callchain(&["res", "get"]));
                                ts.add_parens(TokenStream::create_literal(&field.column_name));
                                ts.add_punc(',')
                            }
                            ts
                        });
                        ts
                    });
                    ts
                });
                ts
            });
            ts.add_ident("else");
            ts.wrap_braces({
                let mut ts = TokenStream::new();
                ts.add_ident("Ok");
                ts.add_parens(TokenStream::create_ident("None"));
                ts
            });

            ts
        });
        ts
    });
    out
}
