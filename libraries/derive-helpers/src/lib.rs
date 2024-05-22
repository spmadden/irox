// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//!
//!

#![forbid(unsafe_code)]

extern crate proc_macro;
use irox_tools::iterators::Itertools;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

pub trait DeriveMethods: Extend<TokenStream> + Extend<TokenTree> {
    fn add_ident(&mut self, name: &str) {
        self.extend(Self::create_ident(name))
    }
    fn create_ident(name: &str) -> TokenStream {
        TokenStream::from_iter([TokenTree::Ident(Ident::new(name, Span::call_site()))])
    }
    fn add_punc(&mut self, ch: char) {
        self.extend([TokenTree::Punct(Punct::new(ch, Spacing::Alone))])
    }
    fn add_punc2(&mut self, ch: char, ch2: char) {
        self.extend([
            TokenTree::Punct(Punct::new(ch, Spacing::Joint)),
            TokenTree::Punct(Punct::new(ch2, Spacing::Alone)),
        ])
    }
    fn add_comma(&mut self) {
        self.add_punc(',')
    }
    fn wrap_generics(&mut self, inner: TokenStream) {
        self.add_punc('<');
        self.extend([inner]);
        self.add_punc('>');
    }

    fn add_fn(&mut self, name: &str) {
        self.add_ident("fn");
        self.add_ident(name);
    }
    fn add_generics(&mut self, id: &str, generics: TokenStream) {
        self.add_punc('<');
        self.add_ident(id);
        self.add_punc(':');
        self.extend(generics);
        self.add_punc('>');
    }
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
    fn add_path(&mut self, elems: &[&str]) {
        self.extend(Self::create_path(elems))
    }
    fn add_parens(&mut self, inside_parens: TokenStream) {
        self.extend([TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            inside_parens,
        ))]);
    }
    fn add_single_arrow(&mut self) {
        self.add_punc2('-', '>')
    }
    fn return_result(&mut self, ok: TokenStream, err: TokenStream) {
        self.add_single_arrow();
        self.add_ident("Result");
        self.add_punc('<');
        self.extend(ok);
        self.add_punc(',');
        self.extend(err);
        self.add_punc('>');
    }

    fn create_empty_type() -> TokenStream {
        TokenStream::from_iter([TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::new(),
        ))])
    }

    fn wrap_braces(&mut self, inner: TokenStream) {
        self.extend([TokenTree::Group(Group::new(Delimiter::Brace, inner))])
    }

    fn wrap_brackets(&mut self, inner: TokenStream) {
        self.extend([TokenTree::Group(Group::new(Delimiter::Bracket, inner))])
    }

    fn add_literal(&mut self, literal: Literal) {
        self.extend([TokenTree::Literal(literal)])
    }
}

impl DeriveMethods for TokenStream {}
