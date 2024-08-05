mod args;
mod input;

use self::{args::*, input::*};
use quote::quote;
use syn::{parse_macro_input, Member};

/// Derive macro generating an impl of the `Fields` trait and an associated fields enum.
#[proc_macro_derive(Fields, attributes(fields))]
pub fn fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        args,
        vis,
        parent,
        generics,
        fields,
    } = parse_macro_input!(input as Input);

    let variants = fields.iter().map(|(field, variant, ty)| {
        let field = match field {
            Member::Named(field) => field.to_string(),
            Member::Unnamed(unnamed) => unnamed.index.to_string(),
        };
        let doc = format!("Field [`{field}`]({parent}::{field}) of [`{parent}`].",);
        quote! {
            #[doc = #doc]
            #variant(#ty)
        }
    });

    let all = fields.iter().map(|(field, variant, _)| {
        quote! { Self::Field::#variant(self.#field) }
    });

    let sets = fields.iter().map(|(field, variant, _)| {
        quote! { Self::Field::#variant(value) => self.#field = value }
    });

    let attributes = args.attributes(&parent);
    let enum_ident = args.name(&parent);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        #attributes
        #vis enum #enum_ident #ty_generics #where_clause {
            #(#variants),*
        }

        #[automatically_derived]
        impl #impl_generics ::fields::Fields for #parent #ty_generics #where_clause {
            type Field = #enum_ident #ty_generics;

            fn into_all(self) -> impl ::core::iter::Iterator<Item = Self::Field> {
                [ #(#all),* ].into_iter()
            }

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
