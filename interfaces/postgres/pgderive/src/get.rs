// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::fields::{parse_fields_in_struct, FieldOpts, ParseFieldsResult};
use crate::{compile_error, get_table_attr_value};
use irox_derive_helpers::syn::{parse_macro_input, Data, DeriveInput, Fields};
use irox_derive_helpers::DeriveMethods;
use proc_macro::{Literal, TokenStream};

pub fn get_by_pubkey(
    pk: &FieldOpts,
    enum_name: &str,
    table_name: &str,
    pfr: &ParseFieldsResult,
    ts: &mut TokenStream,
) {
    ts.add_ident("impl");
    ts.extend(TokenStream::create_path(&[
        "irox_postgres",
        "PostgresGetByPrimaryKey",
    ]));
    ts.add_ident("for");
    ts.add_ident(enum_name);

    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("type");
        ts.add_ident("Item");
        ts.add_punc('=');
        ts.add_ident(enum_name);
        ts.add_punc(';');
        ts.add_ident("type");
        ts.add_ident("PkType");
        ts.add_punc('=');
        ts.extend(pk.ty.clone());
        ts.add_punc(';');

        ts.add_ident("async");
        ts.add_ident("fn");
        ts.add_ident("get_by_pk");
        ts.add_parens({
            let mut ts = TokenStream::new();
            ts.add_ident("conn");
            ts.add_punc(':');
            ts.extend(TokenStream::create_mut_ref_ident("DbConn"));
            ts.add_punc(',');
            ts.add_ident("id");
            ts.add_punc(':');
            ts.extend(pk.ty.clone());
            ts
        });
        ts.return_result(
            {
                let mut ts = TokenStream::new();
                ts.add_ident("Option");
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
                let query = format!(
                    "select * from \"{table_name}\" where \"{}\" = $1 limit 1;",
                    pk.column_name
                );

                ts.add_literal(Literal::string(&query));
                ts.add_comma();
                ts.add_punc('&');
                ts.wrap_brackets({
                    let mut ts = TokenStream::new();
                    for field in &pfr.uniques {
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
                            for field in &pfr.all_fields {
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
}

pub fn run_get(input: TokenStream) -> TokenStream {
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

    let pfr = match parse_fields_in_struct(named) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let pk = pfr.all_fields.iter().find(|f| f.is_primary_key);

    let enum_name = input.ident.to_string();
    let mut out = TokenStream::new();

    if let Some(pk) = pk {
        get_by_pubkey(pk, &enum_name, &table_name, &pfr, &mut out);
    }
    out
}
