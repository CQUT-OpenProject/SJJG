use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sample_tree() -> Self {
        let left_subtree = Some(Box::new(Node {
            data: 3,
            left: Some(Box::new(Node::new(2))),
            right: Some(Box::new(Node {
                data: 4,
                left: None,
                right: Some(Box::new(Node {
                    data: 9,
                    left: Some(Box::new(Node::new(8))),
                    right: Some(Box::new(Node::new(9))),
                })),
            })),
        }));

        let right_subtree = Some(Box::new(Node {
            data: 18,
            left: Some(Box::new(Node::new(13))),
            right: Some(Box::new(Node::new(21))),
        }));

        Self {
            root: Some(Box::new(Node {
                data: 10,
                left: left_subtree,
                right: right_subtree,
            })),
        }
    }

    pub fn count_nodes(&self) -> usize {
        Self::count_nodes_node(&self.root)
    }

    fn count_nodes_node(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(current) => {
                1 + Self::count_nodes_node(&current.left) + Self::count_nodes_node(&current.right)
            }
            None => 0,
        }
    }

    pub fn count_leaves(&self) -> usize {
        Self::count_leaves_node(&self.root)
    }

    fn count_leaves_node(node: &Option<Box<Node>>) -> usize {
        match node {
            Some(current) => {
                if current.left.is_none() && current.right.is_none() {
                    1
                } else {
                    Self::count_leaves_node(&current.left)
                        + Self::count_leaves_node(&current.right)
                }
            }
            None => 0,
        }
    }

    pub fn preorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::preorder_node(&self.root, &mut result);
        result
    }

    fn preorder_node(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        if let Some(current) = node {
            result.push(current.data);
            Self::preorder_node(&current.left, result);
            Self::preorder_node(&current.right, result);
        }
    }

    pub fn inorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder_node(&self.root, &mut result);
        result
    }

    fn inorder_node(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        if let Some(current) = node {
            Self::inorder_node(&current.left, result);
            result.push(current.data);
            Self::inorder_node(&current.right, result);
        }
    }

    pub fn postorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::postorder_node(&self.root, &mut result);
        result
    }

    fn postorder_node(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
        if let Some(current) = node {
            Self::postorder_node(&current.left, result);
            Self::postorder_node(&current.right, result);
            result.push(current.data);
        }
    }

    pub fn level_order(&self) -> Vec<i32> {
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

    pub fn subtree_depth(&self, target: i32) -> Option<usize> {
        Self::subtree_depth_node(&self.root, target)
    }

    fn subtree_depth_node(node: &Option<Box<Node>>, target: i32) -> Option<usize> {
        match node {
            Some(current) => {
                if current.data == target {
                    return Some(Self::depth_node(node));
                }

                if let Some(found) = Self::subtree_depth_node(&current.left, target) {
                    return Some(found);
                }

                Self::subtree_depth_node(&current.right, target)
            }
            None => None,
        }
    }
}
