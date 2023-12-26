use crate::compose::{Attribute, Tag};
use crate::dom::{Document, DomElement, DomNode};

/// Render a `Document` to a HTML string
pub fn render(page: Document) -> String {
    if !page.head.attributes.is_empty() || !page.body.attributes.is_empty() {
        panic!("Cannot use attributes on <head> or <body> tags (how did you even get this error?)");
    }

    format!(
        concat!(
            r"<!DOCTYPE html>",
            r"<html{lang}>",
            r"<head>",
            r"{}",
            r"</head>",
            r"<body>",
            r"{}",
            r"</body>",
            r"</html>",
        ),
        // ignores attributes!
        render_nodes(page.head.children),
        render_nodes(page.body.children),
        lang = match page.lang {
            Some(lang) => format!(" lang=\"{}\"", lang),
            None => "".to_string(),
        },
    )
}

/// Render multiple DOM nodes
pub(super) fn render_nodes(nodes: Vec<DomNode>) -> String {
    nodes
        .into_iter()
        .map(|node| render_node(node))
        .collect::<Vec<_>>()
        .join("")
}
/// Render a single DOM node
fn render_node(node: DomNode) -> String {
    match node {
        DomNode::Element(element) => render_element(element),
        DomNode::Text(text) => text,
    }
}

/// Render a DOM element to HTML string
fn render_element(element: DomElement) -> String {
    match element.tag {
        Tag::Meta | Tag::Link | Tag::Img | Tag::Input | Tag::Br | Tag::Hr => {
            render_element_self_closing(element)
        }
        _ => render_element_with_children(element),
    }
}

/// Render an element with no innerHTML. Eg. `<br/>`
fn render_element_self_closing(element: DomElement) -> String {
    if !element.children.is_empty() {
        panic!("Cannot use children on <{}/> tag", element.tag);
    }
    format!(
        "<{tag}{attrs}{slash}>",
        tag = element.tag,
        attrs = format_attributes(element.attributes),
        slash = if element.tag.is_void() { "" } else { "/" },
    )
}
/// Render an element with tag that supports innerHTML. Eg. `<a>...</a>`
fn render_element_with_children(element: DomElement) -> String {
    format!(
        "<{tag}{attrs}>{content}</{tag}>",
        tag = element.tag,
        attrs = format_attributes(element.attributes),
        content = render_nodes(element.children),
    )
}

/// Render attributes in key="value" format
fn format_attributes(attributes: Vec<Attribute>) -> String {
    if attributes.is_empty() {
        return String::new();
    }
    // Space to separate from tag name
    " ".to_string()
        + &attributes
            .into_iter()
            .map(|attribute| attribute.name + "=\"" + &attribute.value + "\"")
            .collect::<Vec<_>>()
            .join(" ")
}
