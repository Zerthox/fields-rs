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

    let variants = fields.iter().map(
        |Field {
             member,
             variant,
             ty,
             ..
         }| {
            let field = match member {
                Member::Named(field) => field.to_string(),
                Member::Unnamed(unnamed) => unnamed.index.to_string(),
            };
            let doc = format!("Field [`{field}`]({parent}::{field}) of [`{parent}`].",);
            quote! {
                #[doc = #doc]
                #variant(#ty)
            }
        },
    );

    let all_normal = fields.iter().filter(|field| !field.flatten).map(
        |Field {
             member, variant, ..
         }| {
            quote! { Self::Field::#variant(self.#member) }
        },
    );

    let all_flattened = fields.iter().filter(|field| field.flatten).map(
        |Field {
             member, variant, ..
         }| {
            quote! { .chain(::fields::Fields::into_all(self.#member).map(Self::Field::#variant)) }
        },
    );

    let sets = fields.iter().map(
        |Field {
             flatten,
             member,
             variant,
             ..
         }| {
            if *flatten {
                quote! { Self::Field::#variant(value) => ::fields::Fields::set(&mut self.#member, value) }
            } else {
                quote! { Self::Field::#variant(value) => self.#member = value }
            }
        },
    );

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
                [ #(#all_normal),* ].into_iter() #(#all_flattened)*
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
