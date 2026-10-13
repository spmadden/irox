// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use irox_derive_helpers::syn::spanned::Spanned;
use irox_derive_helpers::syn::{
    parse_macro_input, Data, DeriveInput, Error, Expr, Fields, Lit, Meta, Type,
};
use irox_derive_helpers::DeriveMethods;
use irox_tools::map::OrderedHashMap;
use proc_macro::{Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

fn compile_error<T: Spanned>(span: &T, msg: &'static str) -> TokenStream {
    Error::new(span.span(), msg).into_compile_error().into()
}
#[proc_macro_derive(PostgresImplGet, attributes(column_name, table_name, query))]
pub fn run_get(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut table_name: Option<String> = None;
    let mut query: Option<String> = None;
    for attr in &input.attrs {
        let Ok(attr) = attr.meta.require_name_value() else {
            continue;
        };
        let Some(seg) = attr.path.segments.first() else {
            continue;
        };
        let Expr::Lit(lit) = &attr.value else {
            continue;
        };

        let Lit::Str(s) = &lit.lit else {
            continue;
        };
        match seg.ident.to_string().as_str() {
            "table_name" => {
                table_name = Some(s.value());
            }
            "query" => {
                query = Some(s.value());
            }
            _ => {
                continue;
            }
        }
    }

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
    let mut map = OrderedHashMap::new();
    for field in &named.named {
        let Some(id) = &field.ident else {
            return compile_error(&field, "Field must be named");
        };
        let id = id.to_string();
        for attr in &field.attrs {
            let Ok(attr) = attr.meta.require_name_value() else {
                continue;
            };
            let Expr::Lit(lit) = &attr.value else {
                return compile_error(&attr, "attr must be literal");
            };

            let Lit::Str(s) = &lit.lit else {
                return compile_error(&attr, "attr literal must be str");
            };
            let Some(seg) = attr.path.segments.first() else {
                return compile_error(&attr, "attr must have a path");
            };
            if seg.ident != "column_name" {
                continue;
            }
            let val = s.value();
            map.insert(id.clone(), val);
            break;
        }
        if !map.contains(&id) {
            map.insert(id.clone(), id);
        }
    }
    let enum_name = input.ident;
    let mut out = TokenStream::new();
    out.add_ident("impl");
    out.add_ident(&enum_name.to_string());
    out.wrap_braces({
        let mut ts = TokenStream::new();

        ts.add_ident("pub");
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
                ts.wrap_generics(TokenStream::create_ident("Self"));
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
                ts.extend(TokenStream::create_literal(&query));
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
                    ts.add_ident(&enum_name.to_string());
                    ts.wrap_braces({
                        let mut ts = TokenStream::new();
                        for (k, item) in &map {
                            ts.add_ident(k);
                            ts.add_punc(':');
                            ts.extend(TokenStream::create_callchain(&["row", "get"]));
                            ts.add_parens(TokenStream::create_literal(item));
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

    out
}
#[derive(Debug, Clone)]
struct FieldOpts {
    name: String,
    _ty: TokenStream,
    is_unique: bool,
    is_primary_key: bool,
    is_increment_on_update: bool,
    column_name: String,
}
#[proc_macro_derive(
    PostgresImplUpdate,
    attributes(
        unique,
        increment_on_update,
        pg_ignore,
        auto_pk,
        table_name,
        column_name
    )
)]
pub fn run_update(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut table_name: Option<String> = None;
    for attr in &input.attrs {
        let Ok(attr) = attr.meta.require_name_value() else {
            continue;
        };
        let Some(seg) = attr.path.segments.first() else {
            continue;
        };
        let Expr::Lit(lit) = &attr.value else {
            continue;
        };

        let Lit::Str(s) = &lit.lit else {
            continue;
        };
        match seg.ident.to_string().as_str() {
            "table_name" => {
                table_name = Some(s.value());
            }
            _ => {
                continue;
            }
        }
    }
    let Some(table_name) = table_name else {
        return compile_error(&input, "table_name attribute must be present");
    };
    let Data::Struct(s) = &input.data else {
        return compile_error(&input, "Unsupported non-struct type found");
    };
    let Fields::Named(named) = &s.fields else {
        return compile_error(&input, "can only derived on named fields");
    };
    let mut all_fields = Vec::<FieldOpts>::new();
    let mut all_not_unique = Vec::<FieldOpts>::new();
    let mut uniques = Vec::<FieldOpts>::new();
    for field in &named.named {
        let Some(id) = &field.ident else {
            return compile_error(&field, "Field must be named");
        };
        let id = id.to_string();
        let fty = field.ty.clone();
        let field_name_ty = match path_to_stream(&field, fty) {
            Ok(ts) => (id.clone(), ts),
            Err(ts) => {
                return ts;
            }
        };
        let mut field_opts = FieldOpts {
            name: field_name_ty.0,
            _ty: field_name_ty.1,
            is_unique: false,
            is_primary_key: false,
            is_increment_on_update: false,
            column_name: id.clone(),
        };
        let mut skip_field = false;
        for attr in &field.attrs {
            let mut value: Option<String> = None;
            let attrname = match &attr.meta {
                Meta::NameValue(attr) => {
                    let Expr::Lit(attrval) = &attr.value else {
                        return compile_error(&attr, "attr must be literal");
                    };

                    let Lit::Str(attrval) = &attrval.lit else {
                        return compile_error(&attr, "attr literal must be str");
                    };
                    let Some(name) = attr.path.segments.first() else {
                        return compile_error(&attr, "attr must have a path");
                    };
                    value = Some(attrval.value());
                    name.ident.to_string()
                }
                Meta::Path(attrname) => {
                    let Some(name) = attrname.segments.first() else {
                        return compile_error(&attr, "attr must have a path");
                    };
                    name.ident.to_string()
                }
                _ => {
                    continue;
                }
            };
            match attrname.as_str() {
                "column_name" => {
                    if let Some(value) = value {
                        field_opts.column_name = value;
                    }
                }
                "unique" => {
                    field_opts.is_unique = true;
                }
                "increment_on_update" => {
                    field_opts.is_increment_on_update = true;
                }
                "auto_pk" | "ignore" | "pg_ignore" => {
                    field_opts.is_primary_key = true;
                    skip_field = true;
                    break;
                }
                _ => {}
            }
        }
        if skip_field {
            continue;
        }
        all_fields.push(field_opts.clone());
        if field_opts.is_unique {
            uniques.push(field_opts);
        } else {
            all_not_unique.push(field_opts);
        }
    }
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

#[proc_macro_derive(PostgresImplFindBy, attributes(unique, table_name))]
pub fn run_findby(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut table_name: Option<String> = None;
    for attr in &input.attrs {
        let Ok(attr) = attr.meta.require_name_value() else {
            continue;
        };
        let Some(seg) = attr.path.segments.first() else {
            continue;
        };
        let Expr::Lit(lit) = &attr.value else {
            continue;
        };

        let Lit::Str(s) = &lit.lit else {
            continue;
        };
        match seg.ident.to_string().as_str() {
            "table_name" => {
                table_name = Some(s.value());
            }
            _ => {
                continue;
            }
        }
    }
    let Some(table_name) = table_name else {
        return compile_error(&input, "table_name attribute must be present");
    };
    let Data::Struct(s) = &input.data else {
        return compile_error(&input, "Unsupported non-struct type found");
    };
    let Fields::Named(named) = &s.fields else {
        return compile_error(&input, "can only derived on named fields");
    };
    let mut all_fields = Vec::<(String, TokenStream)>::new();
    let mut uniques = Vec::<(String, TokenStream)>::new();
    for field in &named.named {
        let Some(id) = &field.ident else {
            return compile_error(&field, "Field must be named");
        };
        let id = id.to_string();
        let fty = field.ty.clone();
        let field_name_ty = match path_to_stream(&field, fty) {
            Ok(ts) => (id, ts),
            Err(ts) => {
                return ts;
            }
        };

        for attr in &field.attrs {
            let Ok(attr) = attr.meta.require_path_only() else {
                continue;
            };
            let Ok(ident) = attr.require_ident() else {
                continue;
            };
            let ident = ident.to_string();
            if ident != "unique" {
                continue;
            }

            uniques.push(field_name_ty.clone());
            break;
        }
        all_fields.push(field_name_ty);
    }
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

            for (arg, ty) in &uniques {
                ts.add_comma();
                ts.extend(TokenStream::create_ident(arg));
                ts.add_punc(':');
                ts.add_ident("impl");
                ts.add_ident("Into");
                ts.wrap_generics(ty.clone());
            }

            ts
        });
        use std::fmt::Write;
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
                for (idx, (arg, _ty)) in uniques.iter().enumerate() {
                    if need_and {
                        query += " and ";
                    }
                    let _ = write!(&mut query, "\"{arg}\" = ${}", idx + 1);
                    need_and = true;
                }
                query += " limit 1;";

                ts.add_literal(Literal::string(&query));
                ts.add_comma();
                ts.add_punc('&');
                ts.wrap_brackets({
                    let mut ts = TokenStream::new();
                    for (arg, _ty) in &uniques {
                        ts.add_punc('&');
                        ts.extend(TokenStream::create_callchain(&[arg, "into"]));
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
                            for (arg, _ty) in &all_fields {
                                ts.add_ident(arg);
                                ts.add_punc(':');
                                ts.extend(TokenStream::create_callchain(&["res", "get"]));
                                ts.add_parens(TokenStream::create_literal(arg));
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

fn path_to_stream<T: Spanned>(span: &T, fty: Type) -> Result<TokenStream, TokenStream> {
    match fty {
        Type::Path(path) => {
            let mut out = TokenStream::new();

            let mut need_prefix = false;
            for seg in path.path.segments {
                if need_prefix {
                    out.extend([
                        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    ]);
                }
                need_prefix = true;
                let id = Ident::new(&seg.ident.to_string(), Span::call_site());
                out.extend([TokenTree::Ident(id)])
            }
            Ok(out)
        }
        _ => Err(compile_error(span, "attr must have a path")),
    }
}
