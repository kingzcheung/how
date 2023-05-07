use std::{slice::Iter, vec};
use colored::*;
use markdown::mdast::{
    BlockQuote, Delete, Emphasis, FootnoteDefinition, Heading, Link, LinkReference, List, ListItem,
    MdxJsxFlowElement, Node, Paragraph, Root, Strong, Table, TableCell, TableRow, Text, Code, InlineCode,
};

pub trait CompileText {
    fn to_text(&self) -> PlainText;
}

pub trait Children {
    fn children(&self) -> Iter<Node>;
}

macro_rules! impl_children {
    ( $($t: ty),+ ) => {$(
        impl Children for &$t {
            fn children(&self)->Iter<Node> {
                return self.children.iter();
            }
        }
    )*};
}

impl_children!(
    Root,
    Heading,
    List,
    BlockQuote,
    FootnoteDefinition,
    MdxJsxFlowElement,
    Delete,
    Emphasis,
    Link,
    LinkReference,
    Strong,
    Table,
    TableCell,
    TableRow,
    ListItem,
    Paragraph
);

fn enter_children<C: Children>(c: C) -> PlainText {
    let mut pts: PlainText = vec![];
    for node in c.children() {
        pts.extend(enter(node));
    }
    pts
}
// pub struct PlainText {
//     block: Vec<String>,
// }

pub type PlainText = Vec<String>;

impl CompileText for Node {
    fn to_text(&self)->PlainText {
        enter(self)
    }
}

fn enter_text(text: &Text) -> PlainText {
    vec![text.value.clone()]
}

fn enter_code(code: &Code) ->PlainText {
    let c = code.value.clone().truecolor(0, 255, 136).to_string();
    vec!["\n\n".into(),c,"\n\n".into()]
}

fn enter_inline_code(code: &InlineCode) ->PlainText {
    vec![code.value.clone()]
}

/// 渲染加粗
fn render_strong(mut pts: PlainText)->PlainText{
    let last_index = pts.len() - 1;
    pts[last_index] = pts[last_index].bold().to_string();
    pts
}

/// 渲染斜体
fn render_emphasis(mut pts: PlainText)->PlainText {
    let last_index = pts.len() - 1;
    pts[last_index] = pts[last_index].italic().to_string();
    pts
}

fn render_heading(mut pts: PlainText) -> PlainText {
    let last_index = pts.len() - 1;
    pts[last_index] = pts[last_index].bold().to_string();
    pts.push("\n\n".into());
    pts
}

fn enter(node: &Node) -> PlainText {
    let mut pts: PlainText = vec![];

    let pt = match node {
        Node::Root(r) => enter_children(r),
        Node::BlockQuote(bq) => enter_children(bq),
        Node::FootnoteDefinition(fd) => enter_children(fd),
        Node::MdxJsxFlowElement(fe) => enter_children(fe),
        Node::List(list) => enter_children(list),
        Node::MdxjsEsm(_) => todo!(),
        Node::Toml(_) => todo!(),
        Node::Yaml(_) => todo!(),
        Node::Break(_) => todo!(),
        Node::InlineCode(code) => enter_inline_code(code),
        Node::InlineMath(_) => todo!(),
        Node::Delete(d) => render_delete(enter_children(d)),
        Node::Emphasis(e) => render_emphasis(enter_children(e)),
        Node::MdxTextExpression(_) => todo!(),
        Node::FootnoteReference(_) => todo!(),
        Node::Html(html) => enter_html(html),
        Node::Image(_) => todo!(),
        Node::ImageReference(_) => todo!(),
        Node::MdxJsxTextElement(_) => todo!(),
        Node::Link(link) => enter_children(link),
        Node::LinkReference(link) => enter_children(link),
        Node::Strong(strong) => render_strong(enter_children(strong)),
        Node::Text(text) => enter_text(text),
        Node::Code(code) => enter_code(code),
        Node::Math(_) => todo!(),
        Node::MdxFlowExpression(_) => todo!(),
        Node::Heading(h) => render_heading(enter_children(h)),
        Node::Table(t) => enter_children(t),
        Node::ThematicBreak(_) => todo!(),
        Node::TableRow(t) => enter_children(t),
        Node::TableCell(t) => enter_children(t),
        Node::ListItem(item) => render_list_item(enter_children(item)),
        Node::Definition(_) => todo!(),
        Node::Paragraph(item) => enter_children(item),
    };
    pts.extend(pt);

    pts
}

/// 渲染中划线
fn render_delete(mut pts: PlainText) -> PlainText {
    let last_index = pts.len() - 1;
    pts[last_index] = pts[last_index].dimmed().to_string();
    pts
}

fn render_list_item(mut pts: PlainText) -> PlainText {
    pts.push("\n".into());

    pts
}

fn enter_html(html: &markdown::mdast::Html) -> PlainText {
    vec![
        html.value.clone()
    ]
}

#[cfg(test)]
mod test {
    use super::CompileText;

    
    #[test]
    fn test_to_text(){
        let text = include_str!("../testdata/test.md");
        let mdtext = markdown::to_mdast(text, &markdown::ParseOptions::default()).unwrap();

        let res = mdtext.to_text();
        assert_eq!(113, res.len());
        println!("{}",res.join(""));
    }
}