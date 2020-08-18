use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    token, Error, Expr, Ident, Token,
};

#[derive(Clone)]
pub enum NodeChild {
    Node(RawNode),
    Block(Expr),
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

        let self_closed = node
            .get_attributes(tokens)
            .and_then(|_| node.end_tag(tokens))?;

        loop {
            if node.is_node_over(tokens) || self_closed {
                break;
            };

            if tokens.peek(token::Brace) {
                let child = get_block(tokens)?;
                node.children.push(NodeChild::Block(child));
            } else {
                let child = RawNode::get_node(tokens)?;
                node.children.push(NodeChild::Node(child));
            }
        }

        if !self_closed {
            node.end_node(tokens)?;
        }

        Ok(node)
    }

    fn get_attributes(&mut self, tokens: ParseStream) -> Result<bool> {
        loop {
            if RawNode::is_tag_ending(tokens) {
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
        Ok(false)
    }

    fn is_node_over(&self, tokens: ParseStream) -> bool {
        tokens.peek(Token![<]) && tokens.peek2(Token![/])
    }

    fn is_tag_ending(tokens: ParseStream) -> bool {
        (tokens.peek(Token![/]) && tokens.peek2(Token![>])) || tokens.peek(Token![>])
    }

    fn end_tag(&self, tokens: ParseStream) -> Result<bool> {
        //Return true if self closed node <node/>
        if tokens.peek(Token![/]) {
            tokens.parse::<Token![/]>()?;
            tokens.parse::<Token![>]>()?;
            Ok(true)
        } else {
            tokens.parse::<Token![>]>()?;
            return Ok(false);
        }
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

fn get_block(tokens: ParseStream) -> Result<Expr> {
    let buffer;
    braced!(buffer in tokens);

    Ok(buffer.parse::<Expr>()?)
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
                //Todo broken here
                tokens.extend(quote! {
                    {
                        let child_node = {
                            let mut node = Node::new(#tag.clone());
                                #(#children)*
                                #(#attributes)*
                                node
                        };

                        node.append(child_node);
                    }
                });
            }
            NodeChild::Block(expr) => {
                tokens.extend(quote! {
                   {
                       for child in #expr {
                           node.append(child);
                       };
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
            let mut node: Node = Node::new(#tag.clone());

            #(#children)*
            #(#attributes)*

           node
        }});
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Attribute { key, value } = self;
        let action: &str = key;
        match action {
            "onClick" => {
                tokens.extend(quote! {
                    node.set_event(Events::OnClick(Box::new(#value)));
                });
            }
            "onHover" => {
                tokens.extend(quote! {
                    node.set_event(Events::OnHover(Box::new(#value)));
                });
            }
            _ => {
                tokens.extend(quote! {
                    node.set_attribute(#key.to_string(), #value.to_string());
                });
            }
        };
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

// Debugging only

impl std::fmt::Debug for Attribute {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{{ key: {:?} }}", self.key)
    }
}

impl std::fmt::Debug for RawNode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Node {{ tag: {:?}, children: {:?}, attributes: {:?} }}",
            self.tag, self.children, self.attributes
        )
    }
}

impl std::fmt::Debug for NodeChild {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            NodeChild::Node(node) => write!(fmt, " Node {{ node: {:?} }}", node),
            NodeChild::Block(_expr) => write!(fmt, ""),
        }
    }
}
