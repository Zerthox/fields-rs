use crate::Args;
use attribute_derive::FromAttr;
use heck::ToUpperCamelCase;
use quote::format_ident;
use syn::{parse::ParseStream, DeriveInput, Field, Generics, Ident, Member, Type, Visibility};

pub struct Input {
    pub args: Args,
    pub vis: Visibility,
    pub parent: Ident,
    pub generics: Generics,
    pub fields: Vec<(Member, Ident, Type)>,
}

impl syn::parse::Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Input> {
        let DeriveInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        } = input.parse()?;

        let fields = match data {
            syn::Data::Struct(data) => data.fields,
            syn::Data::Enum(data) => {
                return Err(syn::Error::new_spanned(
                    &data.enum_token,
                    "enum not supported by fields derive",
                ))
            }
            syn::Data::Union(data) => {
                return Err(syn::Error::new_spanned(
                    &data.union_token,
                    "union not supported by fields derive",
                ))
            }
        };

        let args = Args::from_attributes(&attrs)?;

        let fields = fields
            .iter()
            .enumerate()
            .filter(|(_, field)| args.filter(field))
            .map(|(i, Field { ident, ty, .. })| {
                let variant = match ident {
                    Some(ident) => {
                        Ident::new(&ident.to_string().to_upper_camel_case(), ident.span())
                    }
                    None => format_ident!("Field{i}"),
                };
                let field = match ident {
                    Some(ident) => Member::Named(ident.clone()),
                    None => Member::Unnamed(i.into()),
                };
                (field, variant, ty.clone())
            })
            .collect::<Vec<_>>();

        Ok(Input {
            args,
            vis,
            parent: ident,
            generics,
            fields,
        })
    }
}
