// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![no_std]

extern crate alloc;
extern crate core;

use alloc::format;
use alloc::string::ToString;
use core::str::FromStr;
use proc_macro::{Literal, TokenStream};

use irox_derive_helpers::DeriveMethods;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Expr, Lit};

fn compile_error<T: Spanned>(span: &T, msg: &'static str) -> TokenStream {
    Error::new(span.span(), msg).into_compile_error().into()
}

#[proc_macro_derive(EnumName)]
pub fn enumname_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(s) = input.data else {
        return compile_error(&input, "Unsupported non-struct type found");
    };
    let mut match_elements = alloc::vec::Vec::<TokenStream>::new();
    let mut name_literals = alloc::vec::Vec::<TokenStream>::new();

    for field in s.variants {
        // println!("{}: {:?}", field.ident, field);
        let field_ident = field.ident;
        let field_name = field_ident.to_string();

        name_literals.push({
            let mut ts = TokenStream::create_literal(&field_name);
            ts.add_punc(',');
            ts
        });
        match_elements.push(if field.fields.is_empty() {
            let mut ts = TokenStream::new();
            ts.append_match_item(
                TokenStream::create_path(&["Self", &field_name]),
                TokenStream::create_literal(&field_name),
            );
            ts
        } else {
            let mut ts = TokenStream::new();
            ts.append_match_item(
                {
                    let mut ts = TokenStream::create_path(&["Self", &field_name]);
                    ts.add_parens(TokenStream::create_punct2('.', '.'));
                    ts
                },
                TokenStream::create_literal(&field_name),
            );
            ts
        });
    }

    let enum_name = input.ident;
    let mut out = TokenStream::new();
    out.add_ident("impl");
    out.add_ident(&enum_name.to_string());
    out.wrap_braces({
        let mut ts = TokenStream::new();
        //#[must_use]
        //fn name(&self) -> &'static str {
        //    match self {
        //        Self::#field_ident => #field_literal,
        //        Self::#field_ident(..) => #field_literal,
        //    }
        //}
        ts.add_must_use();
        ts.add_getter("name", TokenStream::create_ref_ident_static("str"));
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("match");
            ts.add_ident("self");
            ts.wrap_braces({
                let mut ts = TokenStream::new();
                ts.extend(match_elements);
                ts
            });
            ts
        });

        //pub fn iter_names() -> impl Iterator<Item=&'static str>{
        //    extern crate alloc;
        //    let names = alloc::vec![#(#names),*];
        //    names.into_iter()
        //}
        ts.add_ident("pub");
        ts.add_ident("fn");
        ts.add_ident("iter_names");
        ts.extend(TokenStream::create_empty_type());
        ts.add_single_arrow();
        ts.add_ident("impl");
        ts.add_ident("Iterator");
        ts.wrap_generics({
            let mut ts = TokenStream::new();
            ts.add_ident("Item");
            ts.add_punc('=');
            ts.extend(TokenStream::create_ref_ident_static("str"));
            ts
        });
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("extern");
            ts.add_ident("crate");
            ts.add_ident("alloc");
            ts.add_punc(';');
            ts.add_ident("let");
            ts.add_ident("names");
            ts.add_punc('=');
            ts.add_path(&["alloc", "vec"]);
            ts.add_punc('!');
            ts.wrap_brackets({
                let mut ts = TokenStream::new();
                ts.extend(name_literals);
                ts
            });
            ts.add_punc(';');
            ts.add_ident("names");
            ts.add_punc('.');
            ts.add_ident("into_iter");
            ts.extend(TokenStream::create_empty_type());
            ts
        });

        ts
    });

    out
}

#[proc_macro_derive(EnumIterItem, attributes(skip))]
pub fn enumitemiter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(s) = input.data else {
        return compile_error(&input, "Required enum type");
    };
    let enum_name = input.ident;

    let mut items = alloc::vec::Vec::<TokenStream>::new();

    for field in s.variants {
        if field
            .attrs
            .iter()
            .any(|attr| attr.meta.path().is_ident("skip"))
        {
            continue;
        }
        if !field.fields.is_empty() {
            return compile_error(&field.span(), "Field cannot have fields");
        }

        let field_ident = field.ident;
        items.push({
            let mut ts = TokenStream::create_path(&["Self", &field_ident.to_string()]);
            ts.add_punc(',');
            ts
        });
    }
    let mut out = TokenStream::new();
    out.add_ident("impl");
    out.add_path(&["irox_enums", "EnumIterItem"]);
    out.add_ident("for");
    out.add_ident(&enum_name.to_string());
    out.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("type");
        ts.add_ident("Item");
        ts.add_punc('=');
        ts.add_ident(&enum_name.to_string());
        ts.add_punc(';');

        ts.add_fn("iter_items");
        ts.extend([TokenStream::create_empty_type()]);
        ts.add_single_arrow();
        ts.add_path(&["irox_enums", "IntoIter"]);
        ts.wrap_generics(TokenStream::create_path(&["Self", "Item"]));
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("extern");
            ts.add_ident("crate");
            ts.add_ident("alloc");
            ts.add_punc(';');

            ts.add_ident("let");
            ts.add_ident("items");
            ts.add_punc('=');
            ts.add_path(&["alloc", "vec"]);
            ts.add_punc('!');
            ts.wrap_brackets({
                let mut ts = TokenStream::new();
                ts.extend(items);
                ts
            });
            ts.add_punc(';');

            ts.add_ident("items");
            ts.add_punc('.');
            ts.add_ident("into_iter");
            ts.extend(TokenStream::create_empty_type());
            ts
        });
        ts
    });

    out
}

#[proc_macro_derive(EnumTryFromStr)]
pub fn tryfromstr_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(s) = input.data else {
        return compile_error(&input, "Required enum type");
    };
    let enum_name = input.ident;
    let mut out = TokenStream::new();
    out.add_ident("impl");
    out.add_ident("TryFrom");
    out.wrap_generics(TokenStream::create_ref_ident("str"));
    out.add_ident("for");
    out.add_ident(&format!("{enum_name}"));
    out.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("type");
        ts.add_ident("Error");
        ts.add_punc('=');
        ts.extend([TokenStream::create_empty_type()]);
        ts.add_punc(';');

        ts.add_fn("try_from");
        ts.add_parens({
            let mut ts1 = TokenStream::new();
            ts1.add_ident("value");
            ts1.add_punc(':');
            ts1.extend([TokenStream::create_ref_ident("str")]);
            ts1
        });
        ts.return_result(
            TokenStream::create_ident("Self"),
            TokenStream::create_path(&["Self", "Error"]),
        );

        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("Ok");
            ts.add_parens({
                let mut ts = TokenStream::new();
                ts.add_ident("match");
                ts.add_ident("value");
                ts.wrap_braces({
                    let mut ts = TokenStream::new();
                    for field in s.variants {
                        if !field.fields.is_empty() {
                            return compile_error(&field.span(), "Field cannot have fields");
                        }

                        let field_ident = field.ident;
                        let field_name = field_ident.to_string();
                        ts.append_match_item(
                            TokenStream::create_literal(&field_name),
                            TokenStream::create_path(&["Self", &field_name]),
                        );
                    }
                    ts.add_ident("_");
                    ts.add_punc2('=', '>');
                    ts.wrap_braces({
                        let mut ts = TokenStream::new();
                        ts.add_ident("return");
                        ts.add_ident("Err");
                        ts.add_parens(TokenStream::create_empty_type());
                        ts
                    });
                    ts
                });
                ts
            });
            ts
        });
        ts
    });
    out
}

#[proc_macro_derive(EnumTryFromRepr)]
pub fn tryfromrepr_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Enum(s) = &input.data else {
        return compile_error(&input, "Required enum type");
    };
    let mut repr = None;
    for att in &input.attrs {
        if att.path().is_ident("repr") {
            let Ok(lst) = att.meta.require_list() else {
                return compile_error(&att.span(), "Invalid repr attribute");
            };
            repr = Some(lst.tokens.to_string());
            break;
        }
    }
    let Some(repr) = repr else {
        return compile_error(&input, "Required repr attribute");
    };
    let enum_name = input.ident;
    let mut out = TokenStream::new();
    out.add_ident("impl");
    out.add_ident("TryFrom");
    out.wrap_generics(TokenStream::create_ident(&repr));
    out.add_ident("for");
    out.add_ident(&format!("{enum_name}"));
    out.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("type");
        ts.add_ident("Error");
        ts.add_punc('=');
        ts.extend([TokenStream::create_empty_type()]);
        ts.add_punc(';');

        ts.add_fn("try_from");
        ts.add_parens({
            let mut ts1 = TokenStream::new();
            ts1.add_ident("value");
            ts1.add_punc(':');
            ts1.extend([TokenStream::create_ident(&repr)]);
            ts1
        });
        ts.return_result(
            TokenStream::create_ident("Self"),
            TokenStream::create_path(&["Self", "Error"]),
        );

        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("Ok");
            ts.add_parens({
                let mut ts = TokenStream::new();
                ts.add_ident("match");
                ts.add_ident("value");
                ts.wrap_braces({
                    let mut ts = TokenStream::new();
                    for field in &s.variants {
                        if !field.fields.is_empty() {
                            return compile_error(&field.span(), "Field cannot have fields");
                        }
                        let Some((_, desc)) = field.discriminant.as_ref() else {
                            return compile_error(&field.span(), "Field must have a discriminant");
                        };
                        let Expr::Lit(lit) = desc else {
                            return compile_error(
                                &field.span(),
                                "Field discriminant must be a literal",
                            );
                        };
                        let Lit::Int(lit) = &lit.lit else {
                            return compile_error(
                                &field.span(),
                                "Field discriminant must be an integer",
                            );
                        };

                        let field_ident = &field.ident;
                        let field_name = field_ident.to_string();
                        let token = lit.token();
                        let Ok(token) = Literal::from_str(&token.to_string()) else {
                            return compile_error(
                                &field.span(),
                                "Field discriminant must be an integer",
                            );
                        };
                        ts.append_match_item(
                            TokenStream::from_literal(token),
                            TokenStream::create_path(&["Self", &field_name]),
                        );
                    }
                    ts.add_ident("_");
                    ts.add_punc2('=', '>');
                    ts.wrap_braces({
                        let mut ts = TokenStream::new();
                        ts.add_ident("return");
                        ts.add_ident("Err");
                        ts.add_parens(TokenStream::create_empty_type());
                        ts
                    });
                    ts
                });
                ts
            });
            ts
        });
        ts
    });
    out
}
