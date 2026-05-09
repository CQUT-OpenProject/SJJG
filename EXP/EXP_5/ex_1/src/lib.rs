#[derive(Debug)]
pub struct Node {
    data: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: char) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}

pub fn add(root: &mut Option<Box<Node>>, ch: char) {
    match root {
        Some(node) => {
            if ch < node.data {
                add(&mut node.left, ch);
            } else {
                add(&mut node.right, ch);
            }
        }
        None => {
            *root = Some(Box::new(Node::new(ch)));
        }
    }
}

pub fn build_tree(text: &str) -> Option<Box<Node>> {
    let mut root = None;

    for ch in text.chars() {
        if !ch.is_whitespace() {
            add(&mut root, ch);
        }
    }

    root
}

pub fn inorder(root: &Option<Box<Node>>, output: &mut String) {
    if let Some(node) = root {
        inorder(&node.left, output);
        output.push(node.data);
        inorder(&node.right, output);
    }
}

pub fn inorder_string(root: &Option<Box<Node>>) -> String {
    let mut result = String::new();
    inorder(root, &mut result);
    result
}
