use proc_macro::TokenStream;

use derive_syn_parse::Parse;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{discouraged::Speculative, Parse, Result},
    parse_macro_input, Expr, Token,
};

pub fn perform(input: TokenStream) -> TokenStream {
    let input2: Element = parse_macro_input!(input as Element);

    quote! {
        #input2
    }
    .into()
}

#[derive(Parse, Debug)]
pub struct Tag {
    name: syn::Ident,
    #[call(Attribute::parse_inner)]
    attrs: Vec<Attribute>,
    #[allow(dead_code)]
    #[brace]
    open_brace: syn::token::Brace,
    #[inside(open_brace)]
    #[call(Element::parse_inner)]
    children: Vec<Element>,
}

impl ToTokens for Tag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_string();
        let attrs = &self.attrs;
        let children = &self.children;

        let res = quote! {
            VTag::new(#name.to_string(), vec![#(#attrs),*], vec![#(#children),*])
        };

        res.to_tokens(tokens);
    }
}

impl Tag {
    pub fn collect_events(&self) -> Vec<Expr> {
        let mut res = Vec::new();

        for attr in &self.attrs {
            res.extend(attr.collect_events());
        }

        for child in &self.children {
            res.extend(child.collect_events());
        }

        res
    }
}

#[derive(Parse, Debug)]
pub struct Attribute {
    name: syn::Ident,
    #[prefix(Token![:])]
    value: AttrsOrExpr,
}

#[derive(Debug)]
pub enum AttrsOrExpr {
    Attrs(Vec<Attribute>),
    Expr(Expr),
}

impl Parse for AttrsOrExpr {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        if let Ok(attrs) = Attribute::parse_braced_inner(&input) {
            Ok(AttrsOrExpr::Attrs(attrs))
        } else {
            Ok(AttrsOrExpr::Expr(input.parse()?))
        }
    }
}

impl ToTokens for AttrsOrExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AttrsOrExpr::Expr(expr) => quote! {
                #expr
            },
            AttrsOrExpr::Attrs(attrs) => quote! {
                vec![#(#attrs),*]
            },
        }
        .to_tokens(tokens);
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_string();
        let value = &self.value;

        let res = match name.as_str() {
            "click" => {
                quote! {VAttribute::new(#name.to_string(), VAttributeValue::Event(Box::new({ () })))}
            }
            "style" => {
                quote! {VAttribute::new(#name.to_string(), VAttributeValue::Attributes(#value))}
            }
            _ => {
                quote! {VAttribute::new(#name.to_string(), VAttributeValue::String(#value.to_string()))}
            }
        };

        res.to_tokens(tokens);
    }
}

impl Attribute {
    pub fn parse_inner(input: syn::parse::ParseStream) -> Result<Vec<Self>> {
        let mut attrs = Vec::new();

        while let Ok(attr) = input.parse() {
            attrs.push(attr);
        }

        Ok(attrs)
    }

    pub fn parse_braced_inner(input: syn::parse::ParseStream) -> Result<Vec<Self>> {
        let content;

        braced!(content in input);

        Self::parse_inner(&content)
    }

    pub fn collect_events(&self) -> Vec<Expr> {
        match &self.value {
            AttrsOrExpr::Expr(expr) => vec![expr.clone()],
            AttrsOrExpr::Attrs(attrs) => attrs.iter().flat_map(Attribute::collect_events).collect(),
        }
    }
}

#[derive(Debug)]
pub enum Element {
    Tag(Tag),
    Call(Expr),
    Text(Expr),
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Element::Tag(tag) => quote! { VElement::Tag(#tag) }.to_tokens(tokens),
            Element::Call(call) => quote! { #call }.to_tokens(tokens),
            Element::Text(text) => quote! { VElement::Text(#text.to_string()) }.to_tokens(tokens),
        }
    }
}

impl Parse for Element {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let input_forked = input.fork();

        if let Ok(tag) = input_forked.parse() {
            input.advance_to(&input_forked);

            Ok(Element::Tag(tag))
        } else {
            let expr: Expr = input.parse()?;

            Ok(match expr {
                Expr::Call(_) => Element::Call(expr),
                _ => Element::Text(expr),
            })
        }
    }
}

impl Element {
    fn parse_inner(input: syn::parse::ParseStream) -> Result<Vec<Element>> {
        let mut children = Vec::new();

        while let Ok(tag) = input.parse::<Element>() {
            children.push(tag);
        }

        Ok(children)
    }

    pub fn collect_events(&self) -> Vec<Expr> {
        match self {
            Element::Tag(tag) => tag.collect_events(),
            Element::Call(call) => vec![call.clone()],
            Element::Text(_) => Vec::new(),
        }
    }
}
