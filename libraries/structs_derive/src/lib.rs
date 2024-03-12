// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use proc_macro::TokenStream;
use std::fmt::Display;

use quote::quote;
use syn::__private::Span;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

use irox_types::{PrimitiveType, Primitives};

fn compile_error<T: Spanned, D: Display>(span: &T, msg: D) -> TokenStream {
    Error::new(span.span(), msg).into_compile_error().into()
}

fn do_single_primitive(
    input: Primitives,
    writers: &mut Vec<proc_macro2::TokenStream>,
    readers: &mut Vec<proc_macro2::TokenStream>,
    ident: &proc_macro2::Ident,
    big_endian: bool,
) -> Result<(), Error> {
    match input {
        Primitives::u8 => {
            writers.push(quote! {
                out.write_u8(self.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_u8()?,
            })
        }
        Primitives::i8 => {
            writers.push(quote! {
                out.write_i8(self.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_i8()?,
            });
        }
        Primitives::u16 => {
            if big_endian {
                writers.push(quote! {
                    out.write_be_u16(self.#ident)?;
                });
                    readers.push(quote! {
                    #ident: input.read_be_u16()?,
                });
            } else {
                writers.push(quote! {
                    out.write_le_u16(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_le_u16()?,
                });
            }
        }
        Primitives::i16 => {
            if big_endian {
                writers.push(quote! {
                    out.write_be_i16(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_be_i16()?,
                });
            } else {
                writers.push(quote! {
                    out.write_le_i16(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_le_i16()?,
                });
            }
        }
        Primitives::u32 => {
            if big_endian {
                writers.push(quote! {
                    out.write_be_u32(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_be_u32()?,
                });
            } else {
                writers.push(quote! {
                    out.write_le_u32(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_le_u32()?,
                });
            }
        }
        Primitives::i32 => {
            if big_endian {
                writers.push(quote! {
                    out.write_be_i32(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_be_i32()?,
                });
            } else {
                writers.push(quote! {
                    out.write_le_i32(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_le_i32()?,
                });
            }
        }
        Primitives::f32 => {
            writers.push(quote! {
                out.write_f32(self.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_f32()?,
            });
        }
        Primitives::u64 => {
            if big_endian {
                writers.push(quote! {
                    out.write_be_u64(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_be_u64()?,
                });
            } else {
                writers.push(quote! {
                    out.write_le_u64(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_le_u64()?,
                });
            }
        }
        Primitives::i64 => {
            if big_endian {
                writers.push(quote! {
                    out.write_be_i64(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_be_i64()?,
                });
            } else {
                writers.push(quote! {
                    out.write_le_i64(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_le_i64()?,
                });
            }
        }
        Primitives::f64 => {
            writers.push(quote! {
                out.write_f64(self.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_f64()?,
            });
        }
        Primitives::u128 => {
            if big_endian {
                writers.push(quote! {
                    out.write_be_u128(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_be_u128()?,
                });
            } else {
                writers.push(quote! {
                    out.write_le_u128(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_le_u128()?,
                });
            }
        }
        Primitives::i128 => {
            if big_endian {
                writers.push(quote! {
                    out.write_be_i128(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_be_i128()?,
                });
            } else {
                writers.push(quote! {
                    out.write_le_i128(self.#ident)?;
                });
                readers.push(quote! {
                    #ident: input.read_le_i128()?,
                });
            }
        }
        Primitives::bool => {
            writers.push(quote! {
                out.write_u8(self.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_u8()?,
            });
        }
        Primitives::u8_blob => {
            writers.push(quote! {
                out.write_u8_blob(&@dself.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_u8_blob()?,
            });
        }
        Primitives::u16_blob => {
            writers.push(quote! {
                out.write_u16_blob(&self.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_u16_blob()?,
            });
        }
        Primitives::u32_blob => {
            writers.push(quote! {
                out.write_u32_blob(&self.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_u32_blob()?,
            });
        }
        Primitives::u64_blob => {
            writers.push(quote! {
                out.write_u64_blob(&self.#ident)?;
            });
            readers.push(quote! {
                #ident: input.read_u64_blob()?,
            });
        }
        t => {
            return Err(Error::new(
                Span::call_site(),
                format!("Unsupported type: {t:?}"),
            ));
        }
    };
    Ok(())
}

fn arr_writer_function_be(input: Primitives) -> Result<proc_macro2::TokenStream, Error> {
    Ok(match input {
        Primitives::u8 => {
            quote! {
                out.write_u8(elem)?;
            }
        }
        Primitives::i8 => {
            quote! {
                out.write_i8(elem)?;
            }
        }
        Primitives::u16 => {
            quote! {
                out.write_be_u16(elem)?;
            }
        }
        Primitives::i16 => {
            quote! {
                out.write_be_i16(elem)?;
            }
        }
        Primitives::u32 => {
            quote! {
                out.write_be_u32(elem)?;
            }
        }
        Primitives::i32 => {
            quote! {
                out.write_be_i32(elem)?;
            }
        }
        Primitives::f32 => {
            quote! {
                out.write_f32(elem)?;
            }
        }
        Primitives::u64 => {
            quote! {
                out.write_be_u64(elem)?;
            }
        }
        Primitives::i64 => {
            quote! {
                out.write_be_i64(elem)?;
            }
        }
        Primitives::f64 => {
            quote! {
                out.write_f64(elem)?;
            }
        }
        Primitives::u128 => {
            quote! {
                out.write_be_u128(elem)?;
            }
        }
        Primitives::i128 => {
            quote! {
                out.write_be_i128(elem)?;
            }
        }
        _ => {
            return Err(Error::new(Span::call_site(), "Unsupported"));
        }
    })
}

fn arr_writer_function_le(input: Primitives) -> Result<proc_macro2::TokenStream, Error> {
    Ok(match input {
        Primitives::u8 => {
            quote! {
                out.write_u8(elem)?;
            }
        }
        Primitives::i8 => {
            quote! {
                out.write_i8(elem)?;
            }
        }
        Primitives::u16 => {
            quote! {
                out.write_le_u16(elem)?;
            }
        }
        Primitives::i16 => {
            quote! {
                out.write_le_i16(elem)?;
            }
        }
        Primitives::u32 => {
            quote! {
                out.write_le_u32(elem)?;
            }
        }
        Primitives::i32 => {
            quote! {
                out.write_le_i32(elem)?;
            }
        }
        Primitives::f32 => {
            quote! {
                out.write_f32(elem)?;
            }
        }
        Primitives::u64 => {
            quote! {
                out.write_le_u64(elem)?;
            }
        }
        Primitives::i64 => {
            quote! {
                out.write_le_i64(elem)?;
            }
        }
        Primitives::f64 => {
            quote! {
                out.write_f64(elem)?;
            }
        }
        Primitives::u128 => {
            quote! {
                out.write_le_u128(elem)?;
            }
        }
        Primitives::i128 => {
            quote! {
                out.write_le_i128(elem)?;
            }
        }
        _ => {
            return Err(Error::new(Span::call_site(), "Unsupported"));
        }
    })
}

fn arr_reader_fn_be(input: Primitives) -> Result<proc_macro2::TokenStream, Error> {
    Ok(match input {
        Primitives::u8 => {
            quote! {
                input.read_u8()?,
            }
        }
        Primitives::i8 => {
            quote! {
                input.read_i8()?,
            }
        }
        Primitives::u16 => {
            quote! {
                input.read_be_u16()?,
            }
        }
        Primitives::i16 => {
            quote! {
                input.read_be_i16()?,
            }
        }
        Primitives::u32 => {
            quote! {
                input.read_be_u32()?,
            }
        }
        Primitives::i32 => {
            quote! {
                input.read_be_i32()?,
            }
        }
        Primitives::f32 => {
            quote! {
                input.read_f32()?,
            }
        }
        Primitives::u64 => {
            quote! {
                input.read_be_u64()?,
            }
        }
        Primitives::i64 => {
            quote! {
                input.read_be_i64()?,
            }
        }
        Primitives::f64 => {
            quote! {
                input.read_f64()?,
            }
        }
        Primitives::u128 => {
            quote! {
                input.read_be_u128()?,
            }
        }
        Primitives::i128 => {
            quote! {
                input.read_be_i128()?,
            }
        }
        _ => {
            return Err(Error::new(Span::call_site(), "Unsupported"));
        }
    })
}
fn arr_reader_fn_le(input: Primitives) -> Result<proc_macro2::TokenStream, Error> {
    Ok(match input {
        Primitives::u8 => {
            quote! {
                input.read_u8()?,
            }
        }
        Primitives::i8 => {
            quote! {
                input.read_i8()?,
            }
        }
        Primitives::u16 => {
            quote! {
                input.read_le_u16()?,
            }
        }
        Primitives::i16 => {
            quote! {
                input.read_le_i16()?,
            }
        }
        Primitives::u32 => {
            quote! {
                input.read_le_u32()?,
            }
        }
        Primitives::i32 => {
            quote! {
                input.read_le_i32()?,
            }
        }
        Primitives::f32 => {
            quote! {
                input.read_f32()?,
            }
        }
        Primitives::u64 => {
            quote! {
                input.read_le_u64()?,
            }
        }
        Primitives::i64 => {
            quote! {
                input.read_le_i64()?,
            }
        }
        Primitives::f64 => {
            quote! {
                input.read_f64()?,
            }
        }
        Primitives::u128 => {
            quote! {
                input.read_le_u128()?,
            }
        }
        Primitives::i128 => {
            quote! {
                input.read_le_i128()?,
            }
        }
        _ => {
            return Err(Error::new(Span::call_site(), "Unsupported"));
        }
    })
}

#[proc_macro_derive(Struct, attributes(little_endian, big_endian))]
pub fn struct_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut big_endian = true;
    for attr in &input.attrs {
        let Ok(ident) = attr.meta.path().require_ident() else {
            return compile_error(&attr, format!("This attribute is unnamed."));
        };
        if ident.eq("little_endian") {
            big_endian = false;
        } else if ident.eq("big_endian") {
            big_endian = true;
        }
    }

    let struct_name = &input.ident;

    let Data::Struct(s) = input.data else {
        return compile_error(&input, "Can only derive on struct type");
    };
    let Fields::Named(n) = s.fields else {
        return compile_error(&s.fields, "Can only derive on named fields.");
    };
    let mut writers = Vec::new();
    let mut readers = Vec::new();
    for x in n.named {
        let field = match PrimitiveType::try_from(&x) {
            Ok(f) => f,
            Err(e) => {
                return compile_error(&x, format!("Unable to name this field: {e}"));
            }
        };

        let Some(ident) = x.ident else {
            return compile_error(&x, "No ident");
        };
        match field {
            PrimitiveType::Primitive(input) => {
                if let Err(e) = do_single_primitive(input, &mut writers, &mut readers, &ident, big_endian) {
                    return e.into_compile_error().into();
                }
            }
            PrimitiveType::Array(input, size) => {
                let wrfn = if big_endian {
                    arr_writer_function_be(input)
                } else {
                    arr_writer_function_le(input)
                };
                let wrfn = match wrfn {
                    Ok(e) => e,
                    Err(e) => return e.into_compile_error().into(),
                };
                writers.push(quote! {
                    for elem in self.#ident {
                        #wrfn
                    }
                });
                let mut arr_readers = Vec::new();
                let refn = if big_endian {
                    arr_reader_fn_be(input)
                } else {
                    arr_reader_fn_le(input)
                };
                let refn = match refn {
                    Ok(e) => e,
                    Err(e) => return e.into_compile_error().into(),
                };
                for _ in 0..size {
                    arr_readers.push(quote! {
                        #refn
                    })
                }
                let rdr = quote! {
                    #ident: [
                        #(#arr_readers)*
                    ],
                };
                readers.push(rdr);
            }
        }
    }
    TokenStream::from(quote! {
        impl irox_structs::Struct for #struct_name {
            type ImplType = #struct_name;

            fn write_to<T: irox_tools::bits::MutBits>(&self, out: &mut T) -> Result<(), irox_tools::bits::Error> {
                #(#writers)*
                Ok(())
            }

            fn parse_from<T: irox_tools::bits::Bits>(input: &mut T) -> Result<Self::ImplType, irox_tools::bits::Error> {
                Ok(#struct_name {
                    #(#readers)*
                })
            }
        }
    })
}
