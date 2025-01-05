use crate::{FieldArgs, StructArgs};
use attribute_derive::FromAttr;
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use syn::{parse::ParseStream, DeriveInput, Generics, Ident, Member, Type, Visibility};

pub struct Input {
    pub args: StructArgs,
    pub vis: Visibility,
    pub parent: Ident,
    pub generics: Generics,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub flatten: bool,
    pub member: Member,
    pub variant: Ident,
    pub ty: Type,
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

        let args = StructArgs::from_attributes(&attrs)?;

        let fields = fields
            .iter()
            .enumerate()
            .filter(|(_, field)| args.filter(field))
            .map(
                |(
                    i,
                    syn::Field {
                        attrs, ident, ty, ..
                    },
                )| {
                    let FieldArgs { flatten } = FieldArgs::from_attributes(attrs)?;
                    let variant = match ident {
                        Some(ident) => {
                            Ident::new(&ident.to_string().to_upper_camel_case(), ident.span())
                        }
                        None => format_ident!("Field{i}"),
                    };
                    let member = match ident {
                        Some(ident) => Member::Named(ident.clone()),
                        None => Member::Unnamed(i.into()),
                    };
                    Ok(Field {
                        flatten,
                        member,
                        variant,
                        ty: if flatten {
                            Type::Verbatim(quote! { ::fields::Field::<#ty> })
                        } else {
                            ty.clone()
                        },
                    })
                },
            )
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(Input {
            args,
            vis,
            parent: ident,
            generics,
            fields,
        })
    }
}
