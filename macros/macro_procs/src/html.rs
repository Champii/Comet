use proc_macro::TokenStream;

use derive_syn_parse::Parse;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    parse_macro_input, Expr, Pat, Token,
};

pub fn perform(input: TokenStream) -> TokenStream {
    let input2: Element = parse_macro_input!(input as Element);

    quote! {
        (#input2).pop().unwrap()
    }
    .into()
}

pub fn parse_classes(input: ParseStream) -> Result<Vec<syn::Ident>> {
    let mut classes = Vec::new();

    while input.peek(Token![.]) {
        let _ = input.parse::<Token![.]>()?;
        let class = input.parse::<syn::Ident>()?;

        classes.push(class);
    }

    Ok(classes)
}

#[allow(dead_code)]
#[derive(Parse, Debug)]
pub struct Tag {
    name: syn::Ident,

    sharp: Option<Token![#]>,

    #[parse_if(sharp.is_some())]
    id: Option<syn::Ident>,

    #[call(parse_classes)]
    classes: Vec<syn::Ident>,

    #[call(Attribute::parse_inner)]
    attrs: Vec<Attribute>,

    #[brace]
    open_brace: syn::token::Brace,

    #[inside(open_brace)]
    #[call(Element::parse_inner)]
    children: Vec<Element>,
}

fn extend_id_classes(
    attrs: &mut Vec<Attribute>,
    id: &Option<syn::Ident>,
    classes: &Vec<syn::Ident>,
) {
    if let Some(id) = id {
        let id_str = id.to_string();

        attrs.push(Attribute {
            name: syn::Ident::new("id", proc_macro2::Span::call_site()),
            value: AttrsOrExpr::Expr(syn::parse_quote! {#id_str}),
        });
    }

    if classes.is_empty() {
        return;
    }

    let classes_str = classes
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    attrs.iter_mut().for_each(|attr| {
        if attr.name == "class" {
            let old_value = attr.value.clone();

            attr.value =
                AttrsOrExpr::Expr(syn::parse_quote! {format!("{} {}", #classes_str, #old_value)});
        }
    });

    if attrs.iter().find(|attr| attr.name == "class").is_none() {
        let class_attr = Attribute {
            name: syn::Ident::new("class", proc_macro2::Span::call_site()),
            value: AttrsOrExpr::Expr(syn::parse_quote! {#classes_str.to_string()}),
        };

        attrs.push(class_attr);
    }
}

impl ToTokens for Tag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name.to_string();
        let attrs = self.attrs.clone();
        let children = &self.children;
        let id = &self.id;
        let mut classes = self.classes.clone();

        let mut attrs2: Vec<Attribute> = vec![];
        let mut events = vec![];
        let mut binds: Vec<String> = vec![];

        for attr in &attrs {
            if attr.name == "click" {
                match &attr.value {
                    AttrsOrExpr::Expr(_event) => events.push(quote! { {
                        let (name, closure) = #attr;
                        (comet::prelude::percy_dom::event::EventName::ONCLICK, Rc::new(RefCell::new(closure)))
                    }}),
                    AttrsOrExpr::Attrs(_attr) => panic!("click event can't have attributes"),
                }
            } else if attr.name == "change" {
                match &attr.value {
                    AttrsOrExpr::Expr(_event) => events.push(quote! { {
                        let (name, closure) = #attr;
                        ("onchange".into(), Rc::new(RefCell::new(closure)))
                    }}),
                    AttrsOrExpr::Attrs(_attr) => panic!("click event can't have attributes"),
                }
            } else {
                let attr = if attr.name == "bind" {
                    let uuid = uuid::Uuid::new_v4().to_string();
                    let uuid = uuid.replace("-", "");
                    let name = format!("_{}", uuid);

                    binds.push(name.to_string());
                    classes.push(syn::Ident::new(&name, proc_macro2::Span::call_site()));

                    events.push(quote! { {
                        let callback = callback.clone();

                        let closure = move || {
                            callback(None);
                        };
                        (comet::prelude::percy_dom::event::EventName::ONINPUT, Rc::new(RefCell::new(closure)))
                    }});

                    let mut attr = attr.clone();
                    attr.name = syn::Ident::new("value", proc_macro2::Span::call_site());
                    attr
                } else {
                    attr.clone()
                };

                let attr = if attr.name == "r#type" {
                    let attr = attr.clone();
                    Attribute {
                        name: syn::Ident::new("type", proc_macro2::Span::call_site()),
                        value: attr.value,
                    }
                } else {
                    attr.clone()
                };

                attrs2.push(attr.clone());
            }
        }

        extend_id_classes(&mut attrs2, &id, &classes);

        let bind_block = if !binds.is_empty() {
            quote! {
                {
                    let mut bindings = bindings.write().await;
                    #(bindings.push(#binds.to_string());)*
                }
            }
        } else {
            quote! {}
        };

        let res = quote! {
            {
                let mut velem = VElement::new(#name.to_string());

                #(
                    let (name, closure) = #events;
                    velem.events.insert_no_args(name, closure);
                )*

                let attrs_vec: Vec<(String, AttributeValue)> = vec![#(#attrs2),*];
                velem.attrs.extend(attrs_vec);

                #bind_block

                #(
                    velem.children.extend(#children);
                )*

                velem
            }
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

    pub fn collect_bindings(&self) -> Vec<Expr> {
        let mut res = vec![];

        for attr in &self.attrs {
            res.extend(attr.collect_bindings());
        }

        for child in &self.children {
            res.extend(child.collect_bindings());
        }

        res
    }
}

#[derive(Parse, Debug, Clone)]
pub struct Attribute {
    name: syn::Ident,
    #[prefix(Token![:])]
    value: AttrsOrExpr,
}

#[derive(Debug, Clone)]
pub enum AttrsOrExpr {
    Attrs(Vec<Attribute>),
    Expr(Expr),
}

impl AttrsOrExpr {
    pub fn as_expr(&self) -> &Expr {
        match self {
            AttrsOrExpr::Expr(expr) => expr,
            _ => panic!("AttrsOrExpr is not an Expr"),
        }
    }
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
            AttrsOrExpr::Expr(_expr) => {
                quote! {
                    {
                        let msg = events.remove(0);
                        let callback = callback.clone();

                        move || {
                            let msg = msg.clone();
                            callback(Some(msg));
                        }
                    }
                }
            }
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
                quote! {(#name.to_string(), #value)}
            }
            "change" => {
                quote! {(#name.to_string(), #value)}
            }
            "style" => {
                match value {
                    AttrsOrExpr::Attrs(attrs) => {
                        let (names, exprs): (Vec<String>, Vec<Expr>) = attrs
                            .iter()
                            .map(|attr| {
                                let expr = attr.value.as_expr();
                                let name = attr.name.to_string();

                                (name.clone(), expr.clone())
                            })
                            .unzip();

                        quote! {(#name.to_string(), AttributeValue::String({
                            let mut s = String::new();
                            #(

                                s.push_str(#names);
                                s.push_str(":");
                                s.push_str(&#exprs.to_string());
                                s.push_str(";");
                            )*
                            s
                        }))}
                    }
                    // preformated string
                    AttrsOrExpr::Expr(expr) => {
                        quote! {(#name.to_string(), #expr)}
                    }
                }
            }
            _ => {
                match value {
                    AttrsOrExpr::Attrs(_attrs) => {
                        panic!("Attributes can't have attributes");
                    }
                    // preformated string
                    AttrsOrExpr::Expr(expr) => {
                        quote! {(#name.to_string(), AttributeValue::String(#expr.to_string()))}
                    }
                }
                // quote! {(#name.to_string(), #value.to_string())}
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
        if self.name.to_string() == "click" || self.name.to_string() == "change" {
            if let AttrsOrExpr::Expr(expr) = &self.value {
                return vec![expr.clone()];
            } else {
                return vec![];
            }
        } else {
            return vec![];
        }
    }

    pub fn collect_bindings(&self) -> Vec<Expr> {
        let mut res = vec![];

        if self.name.to_string() == "bind" {
            if let AttrsOrExpr::Expr(expr) = &self.value {
                res.push(expr.clone());
            }
        }

        res
    }
}

#[derive(Debug)]
pub enum Element {
    Tag(Tag),
    // Call(Expr),
    Into(Expr),
    If(If),
    For(For),
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Element::Tag(tag) => quote! { vec![VirtualNode::Element(#tag)] }.to_tokens(tokens),
            // Element::Call(call) => quote! { #call.to_virtual_node().await }.to_tokens(tokens),
            Element::Into(expr) => {
                quote! { vec![crate::Wrapper(#expr).to_virtual_node().await] }.to_tokens(tokens)
            }
            Element::If(expr_if) => quote! { #expr_if }.to_tokens(tokens),
            Element::For(expr_for) => quote! { #expr_for }.to_tokens(tokens),
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
            // let input_forked = input.fork();

            if let Ok(if_) = input.parse() {
                Ok(Element::If(if_))
            } else if let Ok(for_) = input.parse() {
                Ok(Element::For(for_))
            } else {
                let expr: Expr = input.parse()?;

                Ok(match expr {
                    // Expr::Call(_) => Element::Call(expr),
                    _ => Element::Into(expr),
                })
            }
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
            // Element::Call(call) => vec![call.clone()],
            Element::Into(_) => Vec::new(),
            Element::If(expr_if) => expr_if.collect_events(),
            Element::For(expr_for) => expr_for.collect_events(),
        }
    }

    pub fn collect_bindings(&self) -> Vec<Expr> {
        match self {
            Element::Tag(tag) => tag.collect_bindings(),
            // Element::Call(_call) => vec![],
            Element::Into(_) => vec![],
            Element::If(expr_if) => expr_if.collect_bindings(),
            Element::For(expr_for) => expr_for.collect_bindings(),
        }
    }
}

#[derive(Parse, Debug)]
pub struct If {
    pub if_token: Token![if],
    pub cond: Expr,

    #[brace]
    pub open_brace: syn::token::Brace,

    #[inside(open_brace)]
    pub then: Box<Element>,
    pub else_token: Option<Token![else]>,
    #[parse_if(else_token.is_some())]
    pub else_: Option<Box<Element>>,
}

impl ToTokens for If {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let cond = &self.cond;
        let then = &self.then;

        let empty_elem = Box::new(Element::Into(syn::parse_quote! { () }));

        let else_ = if let Some(else_) = &self.else_ {
            else_
        } else {
            &empty_elem
        };

        quote! {
            if #cond {
                #then
            } else {
                #else_
            }
        }
        .to_tokens(tokens)
    }
}

impl If {
    pub fn collect_events(&self) -> Vec<Expr> {
        let mut res = vec![];

        res.extend(self.then.collect_events());

        // FIXME: Not sure about that, could f*** up the fix_event phase
        if let Some(else_) = &self.else_ {
            res.extend(else_.collect_events());
        }

        res
    }

    pub fn collect_bindings(&self) -> Vec<Expr> {
        let mut res = vec![];

        res.extend(self.then.collect_bindings());

        if let Some(else_) = &self.else_ {
            res.extend(else_.collect_bindings());
        }

        res
    }
}

#[derive(Parse, Debug)]
pub struct For {
    pub for_token: Token![for],
    pub pat: Pat,
    pub in_token: Token![in],
    pub cond: Expr,

    #[brace]
    pub open_brace: syn::token::Brace,

    #[inside(open_brace)]
    pub block: Box<Element>,
}

impl ToTokens for For {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let pat = &self.pat;
        let cond = &self.cond;
        let block = &self.block;

        quote! {
            {
                let mut arr = vec![];

                for #pat in #cond {
                    arr.extend(#block);
                }

                arr
            }
        }
        .to_tokens(tokens)
    }
}

impl For {
    pub fn collect_events(&self) -> Vec<Expr> {
        self.block.collect_events()
    }

    pub fn collect_bindings(&self) -> Vec<Expr> {
        self.block.collect_bindings()
    }
}
