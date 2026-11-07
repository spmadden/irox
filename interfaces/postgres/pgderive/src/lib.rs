// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

mod fields;
mod findby;
mod get;
mod list;
mod update;

use irox_derive_helpers::syn::spanned::Spanned;
use irox_derive_helpers::syn::{DeriveInput, Error, Expr, Lit, Type};
use proc_macro::{Ident, Punct, Spacing, Span, TokenStream, TokenTree};

fn compile_error<T: Spanned>(span: &T, msg: &'static str) -> TokenStream {
    Error::new(span.span(), msg).into_compile_error().into()
}

#[proc_macro_derive(
    PostgresGetByPrimaryKey,
    attributes(auto_pk, pk, column_name, table_name, query)
)]
pub fn run_get(input: TokenStream) -> TokenStream {
    get::run_get(input)
}
#[proc_macro_derive(PostgresList, attributes(column_name, table_name, query))]
pub fn run_list(input: TokenStream) -> TokenStream {
    list::run_list(input)
}

#[proc_macro_derive(
    PostgresImplUpdate,
    attributes(
        unique,
        increment_on_update,
        pg_ignore,
        auto_pk,
        pk,
        table_name,
        column_name
    )
)]
pub fn run_update(input: TokenStream) -> TokenStream {
    update::run_update(input)
}

#[proc_macro_derive(PostgresImplFindBy, attributes(unique, table_name))]
pub fn run_findby(input: TokenStream) -> TokenStream {
    findby::run_findby(input)
}

pub(crate) fn path_to_stream<T: Spanned>(span: &T, fty: Type) -> Result<TokenStream, TokenStream> {
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

pub(crate) fn get_table_attr_value(input: &DeriveInput, name: &str) -> Option<String> {
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
        if seg.ident.to_string().as_str() == name {
            return Some(s.value());
        }
    }
    None
}
