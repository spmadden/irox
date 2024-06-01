// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

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
    /// Creates a [`Ident`] using the provided name
    fn create_ident(name: &str) -> TokenStream {
        TokenStream::from_iter([TokenTree::Ident(Ident::new(name, Span::call_site()))])
    }
    /// Appends the specified character as a [`Punct`] type.
    fn add_punc(&mut self, ch: char) {
        self.extend([TokenTree::Punct(Punct::new(ch, Spacing::Alone))])
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
    /// Appends a `Result< {ok} , {err} >` stream
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
}

impl DeriveMethods for TokenStream {}
