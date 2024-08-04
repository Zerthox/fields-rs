mod args;

use self::args::*;
use attribute_derive::FromAttr;
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Field, Ident, Member};

/// Derive macro generating an impl of the `Fields` trait and an associated fields enum.
#[proc_macro_derive(Fields, attributes(fields))]
pub fn fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident: struct_ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let fields = match data {
        syn::Data::Struct(data) => data.fields,
        syn::Data::Enum(data) => {
            return syn::Error::new_spanned(&data.enum_token, "enum not supported by fields derive")
                .into_compile_error()
                .into()
        }
        syn::Data::Union(data) => {
            return syn::Error::new_spanned(
                &data.union_token,
                "union not supported by fields derive",
            )
            .into_compile_error()
            .into()
        }
    };

    let args = match Args::from_attributes(&attrs) {
        Ok(attr) => attr,
        Err(err) => return err.to_compile_error().into(),
    };
    let enum_ident = args.name(&struct_ident);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| args.filter(field))
        .map(|(i, Field { ident, ty, .. })| {
            let variant = match ident {
                Some(ident) => Ident::new(&ident.to_string().to_upper_camel_case(), ident.span()),
                None => format_ident!("Field{i}"),
            };
            let field = match ident {
                Some(ident) => Member::Named(ident.clone()),
                None => Member::Unnamed(i.into()),
            };
            (field, variant, ty)
        })
        .collect::<Vec<_>>();

    let variants = fields.iter().map(|(field, variant, ty)| {
        let field = match field {
            Member::Named(field) => field.to_string(),
            Member::Unnamed(unnamed) => unnamed.index.to_string(),
        };
        let doc = format!("Field [`{field}`]({struct_ident}::{field}) of [`{struct_ident}`].",);
        quote! {
            #[doc = #doc]
            #variant(#ty)
        }
    });

    let sets = fields.iter().map(|(field, variant, _)| {
        quote! { Self::Field::#variant(value) => self.#field = value }
    });

    let attributes = args.attributes(&struct_ident);
    quote! {
        #attributes
        #vis enum #enum_ident #ty_generics #where_clause {
            #(#variants),*
        }

        #[automatically_derived]
        impl #impl_generics ::fields::Fields for #struct_ident #ty_generics #where_clause {
            type Field = #enum_ident #ty_generics;

            #[inline]
            fn set(&mut self, field: Self::Field) {
                match field {
                    #(#sets),*
                }
            }
        }
    }
    .into()
}
