use std::collections::VecDeque;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default)]
pub struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    pub fn from_seq_table(data: &[Option<char>]) -> Self {
        Self {
            root: Self::build_node(data, 1),
        }
    }

    fn build_node(data: &[Option<char>], index: usize) -> Option<Box<Node>> {
        if index == 0 || index > data.len() {
            return None;
        }

        match data[index - 1] {
            Some(ch) => {
                let mut node = Node::new(ch);
                node.left = Self::build_node(data, index * 2);
                node.right = Self::build_node(data, index * 2 + 1);
                Some(Box::new(node))
            }
            None => None,
        }
    }

    pub fn preorder(&self) -> Vec<char> {
        let mut result = Vec::new();
        Self::preorder_node(&self.root, &mut result);
        result
    }

    fn preorder_node(node: &Option<Box<Node>>, result: &mut Vec<char>) {
        if let Some(current) = node {
            result.push(current.data);
            Self::preorder_node(&current.left, result);
            Self::preorder_node(&current.right, result);
        }
    }

    pub fn inorder(&self) -> Vec<char> {
        let mut result = Vec::new();
        Self::inorder_node(&self.root, &mut result);
        result
    }

    fn inorder_node(node: &Option<Box<Node>>, result: &mut Vec<char>) {
        if let Some(current) = node {
            Self::inorder_node(&current.left, result);
            result.push(current.data);
            Self::inorder_node(&current.right, result);
        }
    }

    pub fn postorder(&self) -> Vec<char> {
        let mut result = Vec::new();
        Self::postorder_node(&self.root, &mut result);
        result
    }

    fn postorder_node(node: &Option<Box<Node>>, result: &mut Vec<char>) {
        if let Some(current) = node {
            Self::postorder_node(&current.left, result);
            Self::postorder_node(&current.right, result);
            result.push(current.data);
        }
    }

    pub fn level_order(&self) -> Vec<char> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(root) = self.root.as_ref() {
            queue.push_back(root.as_ref());
        }

        while let Some(node) = queue.pop_front() {
            result.push(node.data);

            if let Some(left) = node.left.as_ref() {
                queue.push_back(left.as_ref());
            }

            if let Some(right) = node.right.as_ref() {
                queue.push_back(right.as_ref());
            }
        }

        result
    }

    pub fn depth(&self) -> usize {
        Self::depth_node(&self.root)
    }

    fn depth_node(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(current) => {
                let left_depth = Self::depth_node(&current.left);
                let right_depth = Self::depth_node(&current.right);
                left_depth.max(right_depth) + 1
            }
            None => 0,
        }
    }
}

pub fn sample_seq_table() -> Vec<Option<char>> {
    vec![
        Some('A'),
        Some('B'),
        Some('C'),
        Some('D'),
        None,
        Some('E'),
        Some('F'),
        None,
        Some('G'),
        None,
        None,
        Some('H'),
        None,
    ]
}

pub fn sample_tree() -> BinaryTree {
    BinaryTree::from_seq_table(&sample_seq_table())
}
