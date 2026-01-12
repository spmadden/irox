// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_derive_helpers::DeriveMethods;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parenthesized, parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, LitStr};

struct Config {
    name: String,
}

pub fn shared_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = format!("Shared{}", input.ident);
    let mut config = Config { name };
    let struct_name = &input.ident;
    for attr in &input.attrs {
        let Ok(ident) = attr.meta.path().require_ident() else {
            return irox_derive_helpers::compile_error(
                &attr,
                "This attribute is unnamed.".to_string(),
            );
        };
        if ident.eq("shared") {
            if let Err(e) = attr.parse_nested_meta(|m| {
                if m.path.is_ident("name") {
                    let content;
                    parenthesized!(content in m.input);
                    let n: LitStr = content.parse()?;
                    config.name = n.value();
                    return Ok(());
                }
                let ident = m
                    .path
                    .get_ident()
                    .map(ToString::to_string)
                    .unwrap_or_default();
                Err(m.error(format!("Unexpected attribute {ident}")))
            }) {
                return irox_derive_helpers::compile_error(&attr, e.to_string());
            }
        }
    }
    let generics = input.generics.to_token_stream();
    let mut ts = TokenStream::new();
    ts.add_punc('#');
    ts.wrap_brackets({
        let mut ts = TokenStream::new();
        ts.add_ident("derive");
        ts.add_parens(TokenStream::create_ident("Debug"));
        ts
    });
    ts.add_ident("pub");
    ts.add_ident("struct");
    ts.add_ident(&config.name);
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("inner");
        ts.add_punc(':');
        ts.add_path(&["alloc", "sync", "Arc"]);
        ts.wrap_generics({
            let mut ts = TokenStream::new();
            ts.add_path(&["std", "sync", "RwLock"]);
            ts.wrap_generics({
                let mut ts = TokenStream::new();
                ts.add_ident(&struct_name.to_string());
                ts.extend(TokenStream::from(generics.clone()));
                ts
            });
            ts
        });
        ts
    });
    let Data::Struct(s) = input.data else {
        return irox_derive_helpers::compile_error(&input, "Can only derive on struct type");
    };
    let Fields::Named(n) = s.fields else {
        return irox_derive_helpers::compile_error(&s.fields, "Can only derive on named fields.");
    };

    ts.add_ident("impl");
    ts.wrap_generics(TokenStream::from(generics.clone()));
    ts.add_ident(&config.name);
    ts.extend(TokenStream::from(generics.clone()));
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.extend(write_fields(&n, &config));
        ts
    });

    ts.add_ident("impl");
    ts.add_path(&["core", "clone", "Clone"]);
    ts.add_ident("for");
    ts.add_ident(&config.name);
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("fn");
        ts.add_ident("clone");
        ts.add_parens(TokenStream::create_ref_ident("self"));
        ts.add_single_arrow();
        ts.add_ident("Self");
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("Self");
            ts.wrap_braces({
                let mut ts = TokenStream::new();
                ts.add_ident("inner");
                ts.add_punc(':');
                ts.extend(TokenStream::create_callchain(&["self", "inner", "clone"]));
                ts.add_parens(TokenStream::default());
                ts
            });
            ts
        });
        ts
    });

    ts.add_ident("impl");
    ts.add_ident("From");
    ts.wrap_generics({
        let mut ts = TokenStream::new();
        ts.add_ident(&struct_name.to_string());
        ts.extend(TokenStream::from(generics.clone()));
        ts
    });
    ts.add_ident("for");
    ts.add_ident(&config.name);
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("fn");
        ts.add_ident("from");
        ts.add_parens({
            let mut ts = TokenStream::new();
            ts.add_ident("value");
            ts.add_punc(':');
            ts.add_ident(&struct_name.to_string());
            ts.extend(TokenStream::from(generics.clone()));
            ts
        });
        ts.add_single_arrow();
        ts.add_ident("Self");
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("Self");
            ts.wrap_braces({
                let mut ts = TokenStream::new();
                ts.add_ident("inner");
                ts.add_punc(':');
                ts.add_path(&["alloc", "sync", "Arc", "new"]);
                ts.add_parens({
                    let mut ts = TokenStream::new();
                    ts.add_path(&["std", "sync", "RwLock", "new"]);
                    ts.add_parens(TokenStream::create_ident("value"));
                    ts
                });
                ts
            });
            ts
        });
        ts
    });

    ts
}

fn write_fields(n: &FieldsNamed, _config: &Config) -> TokenStream {
    let mut out = TokenStream::new();
    for x in &n.named {
        let Some(ident) = &x.ident else {
            return irox_derive_helpers::compile_error(&x, "No ident");
        };
        let field_type = TokenStream::from(x.ty.to_token_stream());
        let field_name = ident.to_string();
        // getter
        out.extend({
            let mut ts = TokenStream::new();
            ts.add_ident("pub");
            ts.add_ident("fn");
            ts.add_ident(&field_name);
            ts.add_generics("F", {
                let mut ts = TokenStream::new();
                ts.add_ident("FnMut");
                ts.add_parens({
                    let mut ts = TokenStream::new();
                    ts.add_ident("Option");
                    ts.wrap_generics({
                        let mut ts = TokenStream::new();
                        ts.add_punc('&');
                        ts.extend(field_type.clone());
                        ts
                    });
                    ts
                });
                ts.add_single_arrow();
                ts.add_ident("T");
                ts.add_punc(',');
                ts.add_ident("T");
                ts
            });
            ts.add_parens({
                let mut ts = TokenStream::new();
                ts.extend(TokenStream::create_ref_ident("self"));
                ts.add_punc(',');
                ts.add_ident("mut");
                ts.add_ident("f");
                ts.add_punc(':');
                ts.add_ident("F");
                ts
            });
            ts.add_single_arrow();
            ts.add_ident("T");
            ts.wrap_braces({
                let mut ts = TokenStream::new();
                ts.add_ident("if");
                ts.add_ident("let");
                ts.add_ident("Ok");
                ts.add_parens(TokenStream::create_ident("lock"));
                ts.add_punc('=');
                ts.extend(TokenStream::create_callchain(&["self", "inner", "read"]));
                ts.add_parens(TokenStream::default());
                ts.wrap_braces({
                    let mut ts = TokenStream::new();
                    ts.add_ident("f");
                    ts.add_parens({
                        let mut ts = TokenStream::new();
                        ts.add_ident("Some");
                        ts.add_parens({
                            let mut ts = TokenStream::new();
                            ts.add_punc('&');
                            ts.add_ident("lock");
                            ts.add_punc('.');
                            ts.add_ident(&field_name);
                            ts
                        });
                        ts
                    });
                    ts
                });
                ts.add_ident("else");
                ts.wrap_braces({
                    let mut ts = TokenStream::new();
                    ts.add_ident("f");
                    ts.add_parens(TokenStream::create_ident("None"));
                    ts
                });
                ts
            });
            ts
        });

        // mutable
        out.extend({
            let mut ts = TokenStream::new();
            ts.add_ident("pub");
            ts.add_ident("fn");
            ts.add_ident(&format!("{ident}_mut"));
            ts.add_generics("F", {
                let mut ts = TokenStream::new();
                ts.add_ident("FnMut");
                ts.add_parens({
                    let mut ts = TokenStream::new();
                    ts.add_ident("Option");
                    ts.wrap_generics({
                        let mut ts = TokenStream::new();
                        ts.add_punc('&');
                        ts.add_ident("mut");
                        ts.extend(field_type.clone());
                        ts
                    });
                    ts
                });
                ts.add_single_arrow();
                ts.add_ident("T");
                ts.add_punc(',');
                ts.add_ident("T");
                ts
            });
            ts.add_parens({
                let mut ts = TokenStream::new();
                ts.extend(TokenStream::create_mut_ref_ident("self"));
                ts.add_punc(',');
                ts.add_ident("mut");
                ts.add_ident("f");
                ts.add_punc(':');
                ts.add_ident("F");
                ts
            });
            ts.add_single_arrow();
            ts.add_ident("T");
            ts.wrap_braces({
                let mut ts = TokenStream::new();
                ts.add_ident("if");
                ts.add_ident("let");
                ts.add_ident("Ok");
                ts.add_parens(TokenStream::create_mut_ident("lock"));
                ts.add_punc('=');
                ts.extend(TokenStream::create_callchain(&["self", "inner", "write"]));
                ts.add_parens(TokenStream::default());
                ts.wrap_braces({
                    let mut ts = TokenStream::new();
                    ts.add_ident("f");
                    ts.add_parens({
                        let mut ts = TokenStream::new();
                        ts.add_ident("Some");
                        ts.add_parens({
                            let mut ts = TokenStream::new();
                            ts.extend(TokenStream::create_mut_ref_ident("lock"));
                            ts.add_punc('.');
                            ts.add_ident(&field_name);
                            ts
                        });
                        ts
                    });
                    ts
                });
                ts.add_ident("else");
                ts.wrap_braces({
                    let mut ts = TokenStream::new();
                    ts.add_ident("f");
                    ts.add_parens(TokenStream::create_ident("None"));
                    ts
                });
                ts
            });
            ts
        })
    }
    out
}
