use std::fmt::Display;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse_macro_input, Error, Fields, FieldsNamed, ItemEnum, Variant};

#[proc_macro_attribute]
pub fn enum_expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemEnum);

    let mut variants = Punctuated::<Variant, Comma>::new();
    let mut common_field = None;

    for variant in input.variants {
        if is_common_part(&variant) {
            common_field = match common_field {
                Some(_) => {
                    return Error::new(variant.span(), ExpandError::DuplicateTarget)
                        .to_compile_error()
                        .into();
                }
                None => {
                    let Fields::Named(named) = variant.fields else {
                        return Error::new(variant.span(), ExpandError::NotNamedVariant)
                            .to_compile_error()
                            .into();
                    };
                    Some(named)
                }
            }
        } else {
            variants.push(variant);
        }
    }

    if let Some(common_field) = common_field {
        for variant in variants.iter_mut() {
            insert_field(&common_field, &mut variant.fields);
        }
    }

    input.variants = variants;
    quote! {
        #input

    }
    .into()
}

fn is_common_part(variant: &Variant) -> bool {
    variant
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("enum_expand"))
}

fn insert_field(src: &FieldsNamed, dst: &mut Fields) {
    match dst {
        Fields::Named(fields_named) => {
            for named in &src.named {
                fields_named.named.push(named.clone());
            }
        }
        Fields::Unnamed(_) => {}
        Fields::Unit => {
            *dst = Fields::Named(src.clone());
        }
    }
}

enum ExpandError {
    DuplicateTarget,
    NotNamedVariant,
}

impl Display for ExpandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpandError::DuplicateTarget => write!(f, "duplicate expand common target"),
            ExpandError::NotNamedVariant => {
                write!(f, "expand common target is not a named variant")
            }
        }
    }
}
