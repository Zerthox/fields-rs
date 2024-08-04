use attribute_derive::{
    parsing::{AttributeBase, AttributeValue, PositionalValue, SpannedValue},
    FromAttr,
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Field, Ident, Path, Token, VisRestricted, Visibility};

#[derive(FromAttr)]
#[attribute(ident = fields)]
pub struct Args {
    name: Option<String>,

    #[attribute(optional)]
    derive: Vec<Path>,

    #[attribute(optional)]
    visibility: Vec<Vis>,
}

impl Args {
    pub fn name(&self, parent: &Ident) -> Ident {
        self.name
            .as_ref()
            .map(|name| Ident::new(name, Span::call_site()))
            .unwrap_or_else(|| format_ident!("{parent}Field"))
    }

    pub fn filter(&self, field: &Field) -> bool {
        if self.visibility.is_empty() {
            true
        } else {
            match &field.vis {
                Visibility::Public(_) => self.visibility.contains(&Vis::Public),
                Visibility::Inherited => self.visibility.contains(&Vis::Private),
                Visibility::Restricted(got) => self.visibility.iter().any(
                    |expect| matches!(expect, Vis::Restricted(expect) if got.path == expect.path),
                ),
            }
        }
    }

    pub fn attributes(&self, parent: &Ident) -> TokenStream {
        let derives = &self.derive;
        let doc = format!("A field of [`{parent}`].");
        quote! {
            #[doc = #doc]
            #[derive(#(#derives),*)]
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Vis {
    Private,
    Public,
    Restricted(VisRestricted),
}

impl AttributeBase for Vis {
    type Partial = Self;
}

impl AttributeValue for Vis {
    fn parse_value(input: syn::parse::ParseStream) -> syn::Result<SpannedValue<Self::Partial>> {
        let lookahead = input.lookahead1();
        if input.peek(Token![priv]) {
            let token = input.parse::<Token![priv]>()?;
            Ok(SpannedValue::new(Self::Private, token.span()))
        } else if lookahead.peek(Token![pub]) {
            let vis = input.parse::<Visibility>()?;
            let span = vis.span();
            Ok(SpannedValue::new(
                match vis {
                    Visibility::Public(_) => Self::Public,
                    Visibility::Restricted(rest) => Self::Restricted(rest),
                    Visibility::Inherited => {
                        unreachable!("inherited visibility despite pub token")
                    }
                },
                span,
            ))
        } else {
            Err(lookahead.error())
        }
    }
}

impl PositionalValue for Vis {}
