// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use proc_macro::{Literal, TokenStream};

use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

use irox_types::{PrimitiveType, Primitives, VariableType};

use irox_derive_helpers::DeriveMethods;

const TYPES_STRICT_SIZING_INCOMPATIBLE: [Primitives; 1] = [Primitives::null];

struct Config {
    strict_sizing: bool,
    big_endian: bool,
}

fn get_endian_method_for_prim(ty: Primitives, read: bool, big_endian: bool) -> String {
    let rw = if read { "read" } else { "write" };
    let be = if big_endian { "be" } else { "le" };
    let base = match ty {
        Primitives::u8 | Primitives::bool | Primitives::char => "u8".to_string(),
        Primitives::i8 => "i8".to_string(),

        _ => {
            format!("{be}_{ty:?}")
        }
    };
    format!("{rw}_{base}")
}

fn get_endian_method_for_varbl(ty: VariableType, read: bool, big_endian: bool) -> String {
    let rw = if read { "read" } else { "write" };
    let be = if big_endian { "be" } else { "le" };
    let base = match ty {
        VariableType::str => "str_u32_blob".to_string(),
        _ => {
            format!("{be}_{ty:?}")
        }
    };
    format!("{rw}_{base}")
}

fn create_write_to_fn(n: &FieldsNamed, config: &Config, sizing: &mut StructSizing) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend::<TokenStream>(
        quote!(
            fn write_to<T: irox_structs::MutBits>(&self, out: &mut T) -> Result<(), irox_structs::Error>
        )
        .into(),
    );

    let mut method = TokenStream::new();

    for x in &n.named {
        let Some(ident) = &x.ident else {
            return irox_derive_helpers::compile_error(&x, "No ident");
        };
        match PrimitiveType::try_from(x) {
            Ok(field) => {
                if let Some(size) = field.bytes_length() {
                    sizing.size += size;
                }
                match field {
                    PrimitiveType::Primitive(input) => {
                        if config.strict_sizing && TYPES_STRICT_SIZING_INCOMPATIBLE.contains(&input)
                        {
                            return irox_derive_helpers::compile_error(
                                &x,
                                "Type is not compatible with strict sizing",
                            );
                        };
                        method.add_ident("out");
                        method.add_punc('.');
                        method.add_ident(&get_endian_method_for_prim(
                            input,
                            false,
                            config.big_endian,
                        ));
                        method.add_parens({
                            let mut ts = TokenStream::new();
                            ts.add_ident("self");
                            ts.add_punc('.');
                            ts.add_ident(&ident.to_string());
                            ts
                        });
                        method.add_punc('?');
                        method.add_punc(';');
                    }
                    PrimitiveType::Array(input, len) => {
                        if config.strict_sizing && TYPES_STRICT_SIZING_INCOMPATIBLE.contains(&input)
                        {
                            return irox_derive_helpers::compile_error(
                                &x,
                                "Type is not compatible with strict sizing",
                            );
                        };
                        method.add_ident("for");
                        method.add_ident("elem");
                        method.add_ident("in");
                        method.add_ident("self");
                        method.add_punc('.');
                        method.add_ident(&ident.to_string());
                        method.wrap_braces({
                            let mut ts = TokenStream::new();
                            for _ in 0..len {
                                ts.add_ident("out");
                                ts.add_punc('.');
                                ts.add_ident(&get_endian_method_for_prim(
                                    input,
                                    false,
                                    config.big_endian,
                                ));
                                ts.add_parens(TokenStream::create_ident("elem"));
                                ts.add_punc('?');
                                ts.add_punc(';');
                            }
                            ts
                        })
                    }
                    PrimitiveType::DynamicallySized(dy) => {
                        if config.strict_sizing {
                            return irox_derive_helpers::compile_error(
                                &x,
                                "Type is not compatible with strict sizing",
                            );
                        };
                        method.add_ident("out");
                        method.add_punc('.');
                        method.add_ident(&get_endian_method_for_varbl(
                            dy,
                            false,
                            config.big_endian,
                        ));
                        method.add_parens({
                            let mut ts = TokenStream::new();
                            ts.add_punc('&');
                            ts.add_ident("self");
                            ts.add_punc('.');
                            ts.add_ident(&ident.to_string());
                            ts
                        });
                        method.add_punc('?');
                        method.add_punc(';');
                    }
                }
            }
            Err(_e) => {
                // <ty as irox_structs::Struct>::write_to(&self.varbl, out)?;
                let mut ts = TokenStream::new();
                ts.wrap_generics({
                    let mut ts = TokenStream::new();
                    ts.extend::<TokenStream>(x.ty.to_token_stream().into());
                    ts.add_ident("as");
                    ts.add_ident("irox_structs");
                    ts.add_punc2(':', ':');
                    ts.add_ident("Struct");
                    ts
                });
                ts.add_punc2(':', ':');
                ts.add_ident("write_to");
                ts.add_parens({
                    let mut ts = TokenStream::new();
                    ts.add_punc('&');
                    ts.add_ident("self");
                    ts.add_punc('.');
                    ts.add_ident(&ident.to_string());
                    ts.add_punc(',');
                    ts.add_ident("out");
                    ts
                });
                ts.add_punc('?');
                ts.add_punc(';');
                method.extend(ts);
            }
        }
    }
    method.add_ident("Ok");
    method.add_parens(TokenStream::create_empty_type());
    ts.wrap_braces(method);
    ts
}

fn create_parse_from_fn(n: &FieldsNamed, config: &Config) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend::<TokenStream>(quote!(
        fn parse_from<T: irox_structs::Bits>(input: &mut T) -> Result<Self::ImplType, irox_structs::Error>
    ).into());

    let mut inits = TokenStream::new();

    for x in &n.named {
        let Some(ident) = &x.ident else {
            return irox_derive_helpers::compile_error(&x, "No ident");
        };

        match PrimitiveType::try_from(x) {
            Ok(field) => match field {
                PrimitiveType::Primitive(input) => {
                    if config.strict_sizing && TYPES_STRICT_SIZING_INCOMPATIBLE.contains(&input) {
                        return irox_derive_helpers::compile_error(
                            &x,
                            "Type is not compatible with strict sizing",
                        );
                    };
                    inits.add_ident(&ident.to_string());
                    inits.add_punc(':');
                    inits.add_ident("input");
                    inits.add_punc('.');
                    inits.add_ident(&get_endian_method_for_prim(input, true, config.big_endian));
                    inits.add_parens(TokenStream::new());
                    inits.add_punc('?');
                    inits.add_punc(',');
                }
                PrimitiveType::Array(input, len) => {
                    if config.strict_sizing && TYPES_STRICT_SIZING_INCOMPATIBLE.contains(&input) {
                        return irox_derive_helpers::compile_error(
                            &x,
                            "Type is not compatible with strict sizing",
                        );
                    };
                    inits.add_ident(&ident.to_string());
                    inits.add_punc(':');
                    inits.wrap_brackets({
                        let mut ts = TokenStream::new();
                        for _ in 0..len {
                            ts.add_ident("input");
                            ts.add_punc('.');
                            ts.add_ident(&get_endian_method_for_prim(
                                input,
                                true,
                                config.big_endian,
                            ));
                            ts.add_parens(TokenStream::new());
                            ts.add_punc('?');
                            ts.add_punc(',');
                        }
                        ts
                    });
                    inits.add_punc('.');
                    inits.add_ident("into");
                    inits.add_parens(TokenStream::new());
                    inits.add_punc(',');
                }
                PrimitiveType::DynamicallySized(ds) => {
                    if config.strict_sizing {
                        return irox_derive_helpers::compile_error(
                            &x,
                            "Type is not compatible with strict sizing",
                        );
                    };

                    inits.add_ident(&ident.to_string());
                    inits.add_punc(':');
                    inits.add_ident("input");
                    inits.add_punc('.');
                    inits.add_ident(&get_endian_method_for_varbl(ds, true, config.big_endian));
                    inits.add_parens(TokenStream::new());
                    inits.add_punc('?');
                    inits.add_punc(',');
                }
            },
            Err(_e) => {
                // <ty as irox_structs::Struct>::parse_from(input)?;
                let ty = x.ty.to_token_stream();
                inits.add_ident(&ident.to_string());
                inits.add_punc(':');
                inits.extend::<TokenStream>(
                    quote! {
                        <#ty as irox_structs::Struct>::parse_from(input)?,
                    }
                    .into(),
                );
            }
        }
    }

    let mut method = TokenStream::new();
    method.add_ident("Ok");
    method.add_parens({
        let mut ts = TokenStream::new();
        ts.add_ident("Self");
        ts.wrap_braces(inits);
        ts
    });
    ts.wrap_braces(method);
    ts
}

#[derive(Default)]
struct StructSizing {
    size: usize,
}

#[proc_macro_derive(Struct, attributes(little_endian, big_endian, strict_sizing))]
pub fn struct_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut config = Config {
        big_endian: true,
        strict_sizing: false,
    };

    for attr in &input.attrs {
        let Ok(ident) = attr.meta.path().require_ident() else {
            return irox_derive_helpers::compile_error(
                &attr,
                "This attribute is unnamed.".to_string(),
            );
        };
        if ident.eq("little_endian") {
            config.big_endian = false;
        } else if ident.eq("big_endian") {
            config.big_endian = true;
        } else if ident.eq("strict_sizing") {
            config.strict_sizing = true;
        }
    }

    let struct_name = &input.ident;

    let Data::Struct(s) = input.data else {
        return irox_derive_helpers::compile_error(&input, "Can only derive on struct type");
    };
    let Fields::Named(n) = s.fields else {
        return irox_derive_helpers::compile_error(&s.fields, "Can only derive on named fields.");
    };

    let mut ts = TokenStream::new();
    let mut sizing = StructSizing::default();
    ts.add_ident("impl");
    ts.add_path(&["irox_structs", "Struct"]);
    ts.add_ident("for");
    ts.add_ident(&struct_name.to_string());
    ts.wrap_braces({
        let mut ts = TokenStream::new();
        ts.add_ident("type");
        ts.add_ident("ImplType");
        ts.add_punc('=');
        ts.add_ident(&struct_name.to_string());
        ts.add_punc(';');

        ts.extend(create_write_to_fn(&n, &config, &mut sizing));
        ts.extend(create_parse_from_fn(&n, &config));
        ts
    });
    if config.strict_sizing {
        ts.add_ident("impl");
        ts.add_ident(&struct_name.to_string());
        ts.wrap_braces({
            let mut ts = TokenStream::new();
            ts.add_ident("pub");
            ts.add_ident("const");
            ts.add_ident("STRUCT_SIZE");
            ts.add_punc(':');
            ts.add_ident("usize");
            ts.add_punc('=');
            ts.add_literal(Literal::usize_suffixed(sizing.size));
            ts.add_punc(';');
            ts
        });
    }
    ts
}
