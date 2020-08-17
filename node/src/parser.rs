use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    token, Error, Expr, Ident, Token,
};

pub type EventType = Box<dyn Fn() -> Result<()>>;
#[derive(Clone)]
pub enum NodeChild {
    Node(RawNode),
    Block(Expr),
}

pub enum Actions {
    OnClick(EventType),
    OnHover(EventType),
}

#[derive(Clone)]
pub struct Attribute {
    key: String,
    value: Expr,
}

#[derive(Clone)]
pub struct RawNode {
    pub tag: String,
    pub children: Vec<NodeChild>,
    pub attributes: Vec<Attribute>,
}

impl RawNode {
    fn new() -> Self {
        Self {
            tag: "default".into(),
            children: Vec::new(),
            attributes: Vec::new(),
        }
    }

    fn get_node(tokens: ParseStream) -> Result<RawNode> {
        let mut node = RawNode::new();

        tokens.parse::<Token![<]>()?;
        node.tag = tokens.parse::<Ident>()?.to_string();

        if !RawNode::is_tag_ending(tokens) {
            node.get_attributes(tokens)?;
        } else {
            tokens.parse::<Token![>]>()?;
        }

        if !RawNode::is_node_ending(tokens) {
            loop {
                if RawNode::is_node_ending(tokens) {
                    break;
                }
                if tokens.peek(token::Brace) {
                    let child = get_block(tokens)?;
                    node.children.push(NodeChild::Block(child));
                } else {
                    let child = RawNode::get_node(tokens)?;
                    node.children.push(NodeChild::Node(child));
                }
            }
        }

        node.end_node(tokens)?;
        Ok(node)
    }

    fn get_attributes(&mut self, tokens: ParseStream) -> Result<()> {
        loop {
            if RawNode::is_tag_ending(tokens) {
                tokens.parse::<Token![>]>()?;
                break;
            };

            if let Ok(attribute) = tokens.parse::<Attribute>() {
                self.attributes.push(attribute);
            } else {
                return Err(Error::new(
                    Span::call_site(),
                    "Attributes must be inside brackets",
                ));
            };
        }
        Ok(())
    }

    fn is_node_ending(tokens: ParseStream) -> bool {
        tokens.peek(Token![<]) && tokens.peek2(Token![/])
    }

    fn is_tag_ending(tokens: ParseStream) -> bool {
        tokens.peek(Token![>])
    }

    fn end_node(&mut self, tokens: ParseStream) -> Result<()> {
        tokens.parse::<Token![<]>()?;
        tokens.parse::<Token![/]>()?;

        let tag = tokens.parse::<Ident>()?;

        tokens.parse::<Token![>]>()?;

        if self.tag == tag.to_string() {
            return Ok(());
        } else {
            return Err(Error::new(
                tag.span(),
                "Starting tag and ending tag not the same.",
            ));
        };
    }
}

impl ToTokens for NodeChild {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self {
            NodeChild::Node(node) => {
                let RawNode {
                    tag,
                    children,
                    attributes,
                } = node;

                tokens.extend(quote! {
                    {
                        use std::collections::HashMap;
                        let children: Vec<Node> = vec!(#(#children),*);

                        let mut node = Node::new(#tag.clone(), children, HashMap::new());
                        
                        #(#attributes),*

                        vec![node]
                    }
                });
            }
            NodeChild::Block(expr) => {
                tokens.extend(quote! {
                   {
                       let mut response: Vec<Node> = Vec::new();
                       for value in #expr {
                           response.push(value)
                       };

                       response
                   }
                });
            }
        }
    }
}

impl ToTokens for RawNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let RawNode {
            tag,
            children,
            attributes,
        } = self;

        tokens.extend(quote! { {
            use std::collections::HashMap;
            let children: Vec<Vec<Node>> = vec!(#(#children),*);

            let mut node = Node::new(#tag.clone(), children.into_iter().flatten().collect(), HashMap::new());

            #(#attributes),*
   
           node
        }});
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Attribute { key, value } = self;

        tokens.extend(quote! {
            node.set_attribute(#key.to_string(), #value.to_string());
        });
    }
}

impl Parse for RawNode {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(RawNode::get_node(input)?)
    }
}

impl Parse for Attribute {
    fn parse(tokens: ParseStream) -> Result<Self> {
        let key = tokens.parse::<Ident>()?;
        tokens.parse::<Token![=]>()?;

        Ok(Attribute {
            key: key.to_string(),
            value: get_block(tokens)?,
        })
    }
}

fn get_block(tokens: ParseStream) -> Result<Expr> {
    let buffer;
    braced!(buffer in tokens);

    Ok(buffer.parse::<Expr>()?)
}
