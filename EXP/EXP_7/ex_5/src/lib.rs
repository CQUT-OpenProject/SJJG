#[derive(Debug, Clone)]
pub struct TreeNode {
    pub key: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

#[derive(Debug, Clone)]
pub struct BinaryTree {
    pub root: Option<Box<TreeNode>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: i32) -> bool {
        insert_node(&mut self.root, key)
    }

    pub fn search(&self, key: i32) -> bool {
        let mut curr = self.root.as_ref();

        while let Some(node) = curr {
            if key == node.key {
                return true;
            }

            if key < node.key {
                curr = node.left.as_ref();
            } else {
                curr = node.right.as_ref();
            }
        }

        false
    }

    pub fn inorder(&self) -> Vec<i32> {
        let mut result = Vec::new();
        inorder_node(&self.root, &mut result);
        result
    }
}

fn insert_node(link: &mut Option<Box<TreeNode>>, key: i32) -> bool {
    match link {
        Some(node) => {
            if key == node.key {
                false
            } else if key < node.key {
                insert_node(&mut node.left, key)
            } else {
                insert_node(&mut node.right, key)
            }
        }
        None => {
            *link = Some(Box::new(TreeNode {
                key,
                left: None,
                right: None,
            }));
            true
        }
    }
}

fn inorder_node(link: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
    if let Some(node) = link {
        inorder_node(&node.left, result);
        result.push(node.key);
        inorder_node(&node.right, result);
    }
}

pub fn build_tree(data: &[i32]) -> BinaryTree {
    let mut tree = BinaryTree::new();
    let mut i = 0;

    while i < data.len() {
        tree.insert(data[i]);
        i += 1;
    }

    tree
}

pub fn sample_data() -> [i32; 10] {
    [45, 24, 53, 12, 37, 93, 28, 40, 60, 70]
}
