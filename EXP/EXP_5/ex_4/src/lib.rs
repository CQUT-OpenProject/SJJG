#[derive(Debug, Clone)]
struct ChildNode {
    child_index: usize,
    next: Option<Box<ChildNode>>,
}

#[derive(Debug, Clone)]
struct TreeNode {
    data: char,
    first_child: Option<Box<ChildNode>>,
}

#[derive(Debug, Clone)]
pub struct ChildTree {
    nodes: Vec<TreeNode>,
}

impl ChildTree {
    pub fn new(root: char) -> Self {
        Self {
            nodes: vec![TreeNode {
                data: root,
                first_child: None,
            }],
        }
    }

    fn find_index(&self, target: char) -> Option<usize> {
        let mut index = 0;

        while index < self.nodes.len() {
            if self.nodes[index].data == target {
                return Some(index);
            }
            index += 1;
        }

        None
    }

    pub fn add_child(&mut self, parent: char, child: char) -> bool {
        let parent_index = match self.find_index(parent) {
            Some(index) => index,
            None => return false,
        };

        if self.find_index(child).is_some() {
            return false;
        }

        self.nodes.push(TreeNode {
            data: child,
            first_child: None,
        });

        let child_index = self.nodes.len() - 1;
        let mut link = &mut self.nodes[parent_index].first_child;

        while let Some(current) = link {
            link = &mut current.next;
        }

        *link = Some(Box::new(ChildNode {
            child_index,
            next: None,
        }));

        true
    }

    pub fn child_values(&self, parent: char) -> Option<Vec<char>> {
        let parent_index = self.find_index(parent)?;
        let mut result = Vec::new();
        let mut current = self.nodes[parent_index].first_child.as_ref();

        while let Some(child) = current {
            result.push(self.nodes[child.child_index].data);
            current = child.next.as_ref();
        }

        Some(result)
    }
}

pub fn sample_tree() -> ChildTree {
    let mut tree = ChildTree::new('A');

    tree.add_child('A', 'B');
    tree.add_child('A', 'C');
    tree.add_child('A', 'D');
    tree.add_child('B', 'E');
    tree.add_child('B', 'F');
    tree.add_child('D', 'G');

    tree
}
