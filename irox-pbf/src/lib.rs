extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttrStyle, Data, DeriveInput, Fields, Meta, Lit, Attribute};

struct FieldInfo {
    field_id : u32,
    field_name : String,
    is_optional : bool,
    
}

fn try_find_fieldid(field_name: String, attrs : &Vec<Attribute>) -> Result<u32, String> {
    for attr in attrs {
        let meta = attr.parse_meta().unwrap();
        if let Meta::NameValue(mnv) = meta {
            if mnv.path.is_ident("FieldID") {
                if let Lit::Int(int) = mnv.lit {
                    match int.base10_parse::<u32>() {
                        Ok(v) => return Ok(v),
                        Err(e) => return Err(format!("Unable to parse FieldID annotation on {} found error {}", field_name, e.to_string()))
                    }
                }
                return Err(format!("FieldID annotation on {} requires an int value, found {:?}", field_name, mnv.lit));
            }
        }
    }
    Err(format!("Field {} is missing FieldID attribute", field_name))
}

#[proc_macro_derive(PBF, attributes(WireFormat, FieldID))]
pub fn derive_pbf(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let struct_name = ast.ident.to_string();

    let mut out = TokenStream::new();

    let Data::Struct(ds) = ast.data else {
        panic!("Derive PBF is only supported on structs in {}", struct_name);
    };

    let fields = ds.fields;
    let Fields::Named(named) = fields else {
        panic!("Derive PBF is only supported on named fields in {}", struct_name);
    };

    let mut fields : Vec<FieldInfo> = Vec::new();

    for item in named.named.into_iter() {
        let Some(ident) = item.ident else {
            panic!("Derive PBF requires named fields in {}", struct_name);
        };

        let field_name = ident.to_string();
        let dbg_field_name = format!("{}.{}", struct_name, field_name);

        println!("{} : ({:?})", dbg_field_name, item.ty);
        match item.ty {
            syn::Type::Array(_) => todo!(),
            syn::Type::BareFn(_) => todo!(),
            syn::Type::Group(_) => todo!(),
            syn::Type::ImplTrait(_) => todo!(),
            syn::Type::Infer(_) => todo!(),
            syn::Type::Macro(_) => todo!(),
            syn::Type::Never(_) => todo!(),
            syn::Type::Paren(_) => todo!(),
            syn::Type::Path(path) => {
                let ident = path.path.get_ident();
                println!("{} : {:?},", dbg_field_name, ident);
                let toks = path.path.to_token_stream().to_string();
                println!("{} : ... {}", dbg_field_name, toks);
            },
            syn::Type::Ptr(_) => todo!(),
            syn::Type::Reference(_) => todo!(),
            syn::Type::Slice(_) => todo!(),
            syn::Type::TraitObject(_) => todo!(),
            syn::Type::Tuple(_) => todo!(),
            syn::Type::Verbatim(_) => todo!(),
            _ => todo!(),
        }

        match try_find_fieldid(dbg_field_name, &item.attrs) {
            Ok(field_id) => {
                
            },
            Err(e) => panic!("{}", e),
        }

    }

    TokenStream::from(out)
}
