// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Helper traits & functions for the [`proc_macro`] crate to aid in writing less complex derive macros.
//!

#![forbid(unsafe_code)]

extern crate proc_macro;

use irox_tools::iterators::Itertools;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

///
/// Adds a bunch of helper methods to the [`TokenStream`] and [`TokenTree`] types.
pub trait DeriveMethods: Extend<TokenStream> + Extend<TokenTree> {
    /// Appends the specified name as an [`Ident`]
    fn add_ident(&mut self, name: &str) {
        self.extend(Self::create_ident(name))
    }
    /// Creates a literal [`Punct`] type using the provided character
    fn create_punct(ch: char) -> TokenStream {
        TokenStream::from_iter([TokenTree::Punct(Punct::new(ch, Spacing::Alone))])
    }
    /// Creates two literal adjoining [`Punct`] types using the provided characters
    fn create_punct2(ch: char, ch2: char) -> TokenStream {
        TokenStream::from_iter([
            TokenTree::Punct(Punct::new(ch, Spacing::Joint)),
            TokenTree::Punct(Punct::new(ch2, Spacing::Alone)),
        ])
    }
    /// Creates a [`Literal`] using the provided string
    fn create_literal(val: &str) -> TokenStream {
        TokenStream::from_iter([TokenTree::Literal(Literal::string(val))])
    }
    /// Creates a [`Literal`]
    fn from_literal(val: Literal) -> TokenStream {
        TokenStream::from_iter([TokenTree::Literal(val)])
    }
    /// Creates a [`Ident`] using the provided name
    fn create_ident(name: &str) -> TokenStream {
        TokenStream::from_iter([TokenTree::Ident(Ident::new(name, Span::call_site()))])
    }
    /// Creates a `&[name]` token stream.
    fn create_ref_ident(name: &str) -> TokenStream {
        TokenStream::from_iter([Self::create_punct('&'), Self::create_ident(name)])
    }
    /// Creates a `&'lifetime [name]` token stream.
    fn create_ref_ident_lifetime(name: &str, lifetime: &str) -> TokenStream {
        TokenStream::from_iter([
            TokenStream::from_iter([
                TokenTree::Punct(Punct::new('&', Spacing::Alone)),
                TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                TokenTree::Ident(Ident::new(lifetime, Span::call_site())),
            ]),
            Self::create_ident(name),
        ])
    }
    /// Creates a `&'static [name]` token stream.
    fn create_ref_ident_static(name: &str) -> TokenStream {
        Self::create_ref_ident_lifetime(name, "static")
    }
    /// Creates a `&mut [name]` token stream;
    fn create_mut_ref_ident(name: &str) -> TokenStream {
        TokenStream::from_iter([
            Self::create_punct('&'),
            Self::create_ident("mut"),
            Self::create_ident(name),
        ])
    }
    /// Creates a `&'lifetime mut [name]` token stream;
    fn create_mut_ref_ident_lifetime(name: &str, lifetime: &str) -> TokenStream {
        TokenStream::from_iter([
            TokenStream::from_iter([
                TokenTree::Punct(Punct::new('&', Spacing::Alone)),
                TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                TokenTree::Ident(Ident::new(lifetime, Span::call_site())),
            ]),
            Self::create_ident("mut"),
            Self::create_ident(name),
        ])
    }
    /// Wraps the provided token stream with braces: `{ [inner] }`
    fn create_wrapped_braces(inner: TokenStream) -> TokenStream {
        TokenStream::from_iter([TokenTree::Group(Group::new(Delimiter::Brace, inner))])
    }
    /// Appends the specified character as a [`Punct`] type.
    fn add_punc(&mut self, ch: char) {
        self.extend(Self::create_punct(ch))
    }
    /// Appends 2 characters as sequential [`Punct`] types
    fn add_punc2(&mut self, ch: char, ch2: char) {
        self.extend([
            TokenTree::Punct(Punct::new(ch, Spacing::Joint)),
            TokenTree::Punct(Punct::new(ch2, Spacing::Alone)),
        ])
    }
    /// Appends a comma (`,`) as a [`Punct`]
    fn add_comma(&mut self) {
        self.add_punc(',')
    }
    /// Wraps the provided token stream in generics (`<`...`>`)
    fn wrap_generics(&mut self, inner: TokenStream) {
        self.add_punc('<');
        self.extend([inner]);
        self.add_punc('>');
    }

    /// Appends two idents: `fn` and `name`
    fn add_fn(&mut self, name: &str) {
        self.add_ident("fn");
        self.add_ident(name);
    }
    /// Appends named generics: `< {id} : {stream..} >`
    fn add_generics(&mut self, id: &str, generics: TokenStream) {
        self.add_punc('<');
        self.add_ident(id);
        self.add_punc(':');
        self.extend(generics);
        self.add_punc('>');
    }
    /// Creates the elements as a path, a series of [`Ident`]s separated by `::`
    fn create_path(elems: &[&str]) -> TokenStream {
        elems
            .iter()
            .map(|e| TokenTree::Ident(Ident::new(e, Span::call_site())))
            .joining_multi(&[
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
            ])
            .collect()
    }
    /// Appends the elements as a path, a series of [`Ident`]s separated by `::`
    fn add_path(&mut self, elems: &[&str]) {
        self.extend(Self::create_path(elems))
    }
    /// Wraps the provided stream with parens: `( {stream} )`
    fn add_parens(&mut self, inside_parens: TokenStream) {
        self.extend([TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            inside_parens,
        ))]);
    }
    /// Appends the single bar arrow: `->`
    fn add_single_arrow(&mut self) {
        self.add_punc2('-', '>')
    }
    /// Appends the double bar arrow: `=>`
    fn add_double_arrow(&mut self) {
        self.add_punc2('=', '>')
    }
    /// Appends a single match row, `[matching] => [result],`
    fn append_match_item(&mut self, matching: TokenStream, result: TokenStream) {
        self.extend([
            matching,
            TokenStream::from_iter([
                TokenTree::Punct(Punct::new('=', Spacing::Joint)),
                TokenTree::Punct(Punct::new('>', Spacing::Alone)),
            ]),
            TokenStream::create_wrapped_braces(result),
            TokenStream::create_punct(','),
        ])
    }
    /// Appends a `-> Result< {ok} , {err} >` stream
    fn return_result(&mut self, ok: TokenStream, err: TokenStream) {
        self.add_single_arrow();
        self.add_ident("Result");
        self.add_punc('<');
        self.extend(ok);
        self.add_punc(',');
        self.extend(err);
        self.add_punc('>');
    }

    /// Appends a `()` empty type.
    fn create_empty_type() -> TokenStream {
        TokenStream::from_iter([TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        ))])
    }

    /// Wraps the provided stream in braces/staches: `{ {inner} }`
    fn wrap_braces(&mut self, inner: TokenStream) {
        self.extend([TokenTree::Group(Group::new(Delimiter::Brace, inner))])
    }
    /// Wraps the provided stream in brackets: `[ {inner} ]`
    fn wrap_brackets(&mut self, inner: TokenStream) {
        self.extend([TokenTree::Group(Group::new(Delimiter::Bracket, inner))])
    }

    /// Appends the provided [`Literal`]
    fn add_literal(&mut self, literal: Literal) {
        self.extend([TokenTree::Literal(literal)])
    }

    /// Appends the provided [`Ident`]
    fn add_ident_type(&mut self, ident: Ident) {
        self.extend([TokenTree::Ident(ident)])
    }
    /// Appends the stream `#[must_use]`
    fn add_must_use(&mut self) {
        self.extend([
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Bracket,
                TokenStream::create_ident("must_use"),
            )),
        ])
    }
    fn add_getter(&mut self, name: &str, output_type: TokenStream) {
        self.add_ident("pub");
        self.add_ident("fn");
        self.add_ident(name);
        self.add_parens(TokenStream::create_ref_ident("self"));
        self.add_single_arrow();
        self.extend([output_type])
    }
    fn add_todo(&mut self) {
        self.add_ident("todo");
        self.add_punc('!');
        self.extend(Self::create_empty_type());
    }
    fn add_where_self_sized(&mut self) {
        self.add_ident("where");
        self.add_ident("Self");
        self.add_punc(':');
        self.add_ident("Sized");
    }
}

impl DeriveMethods for TokenStream {}

#[cfg(feature = "syn")]
mod synhelp {
    use core::fmt::Display;
    use proc_macro::TokenStream;
    use syn::spanned::Spanned;
    use syn::Error;
    pub fn compile_error<T: Spanned, D: Display>(span: &T, msg: D) -> TokenStream {
        Error::new(span.span(), msg).into_compile_error().into()
    }
}
#[cfg(feature = "syn")]
pub use synhelp::*;
#[cfg(feature = "syn")]
pub extern crate syn;
