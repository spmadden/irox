// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::{compile_error, path_to_stream};
use irox_derive_helpers::syn::{Expr, FieldsNamed, Lit, Meta};
use proc_macro::TokenStream;

#[derive(Debug, Clone)]
pub struct FieldOpts {
    pub name: String,
    pub ty: TokenStream,
    pub is_unique: bool,
    pub is_primary_key: bool,
    pub is_increment_on_update: bool,
    pub column_name: String,
}

pub struct ParseFieldsResult {
    pub all_fields: Vec<FieldOpts>,
    pub all_not_unique: Vec<FieldOpts>,
    pub uniques: Vec<FieldOpts>,
}
pub fn parse_fields_in_struct(named: &FieldsNamed) -> Result<ParseFieldsResult, TokenStream> {
    let mut all_fields = Vec::<FieldOpts>::new();
    let mut all_not_unique = Vec::<FieldOpts>::new();
    let mut uniques = Vec::<FieldOpts>::new();
    for field in &named.named {
        let Some(id) = &field.ident else {
            return Err(compile_error(&field, "Field must be named"));
        };
        let id = id.to_string();
        let fty = field.ty.clone();
        let field_name_ty = match path_to_stream(&field, fty) {
            Ok(ts) => (id.clone(), ts),
            Err(ts) => {
                return Err(ts);
            }
        };
        let mut field_opts = FieldOpts {
            name: field_name_ty.0,
            ty: field_name_ty.1,
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
                        return Err(compile_error(&attr, "attr must be literal"));
                    };

                    let Lit::Str(attrval) = &attrval.lit else {
                        return Err(compile_error(&attr, "attr literal must be str"));
                    };
                    let Some(name) = attr.path.segments.first() else {
                        return Err(compile_error(&attr, "attr must have a path"));
                    };
                    value = Some(attrval.value());
                    name.ident.to_string()
                }
                Meta::Path(attrname) => {
                    let Some(name) = attrname.segments.first() else {
                        return Err(compile_error(&attr, "attr must have a path"));
                    };
                    name.ident.to_string()
                }
                _ => {
                    continue;
                }
            };
            match attrname.as_str() {
                "column_name" => {
                    field_opts.column_name =
                        value.ok_or(compile_error(&attr, "column name attribute"))?;
                }
                "unique" => {
                    field_opts.is_unique = true;
                }
                "increment_on_update" => {
                    field_opts.is_increment_on_update = true;
                }
                "auto_pk" => {
                    field_opts.is_increment_on_update = true;
                    field_opts.is_primary_key = true;
                }
                "pk" => {
                    field_opts.is_primary_key = true;
                }
                "ignore" | "pg_ignore" => {
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
    Ok(ParseFieldsResult {
        all_fields,
        all_not_unique,
        uniques,
    })
}
