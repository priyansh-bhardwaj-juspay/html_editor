use html_editor::{operation::*, Element};
use html_editor::{parse, Node};

const HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Document</title>
    </head>
    <body>
        <p>Hello</p>
    </body>
    </html>"#;

const INSERTED_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Document</title>
    </head>
    <body>
        <p>Hello</p>
    <script>console.log("Hello World")</script></body>
    </html>"#;

const REMOVED_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        
        <title>Document</title>
    </head>
    <body>
        <p>Hello</p>
    </body>
    </html>"#;

const REPLACED_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Document</title>
    </head>
    <body>
        <p>Hello World!</p>
    </body>
    </html>"#;

#[test]
fn insert() {
    let body_selector = Selector::from("body");
    let script = Node::new_element(
        "script",
        Vec::new(),
        vec![Node::Text(r#"console.log("Hello World")"#.to_string())],
    );
    let html = parse(HTML)
        .unwrap()
        .insert_to(&body_selector, script)
        .html();
    assert_eq!(html, INSERTED_HTML);
}

#[test]
fn remove() {
    let meta_selector = Selector::from("meta");
    let html = parse(HTML).unwrap().remove_by(&meta_selector).html();
    assert_eq!(html, REMOVED_HTML);
}

#[test]
fn replace() {
    let p_selector = Selector::from("p");
    let html = parse(HTML)
        .unwrap()
        .replace_with(&p_selector, |p| {
            let new_text = format!("{} World!", p.children[0].html());
            let node = Node::Element(Element {
                name: "p".to_string(),
                attrs: vec![],
                children: vec![Node::Text(new_text)],
            });
            Ok(node)
        }).unwrap()
        .html();
    assert_eq!(html, REPLACED_HTML);
}
