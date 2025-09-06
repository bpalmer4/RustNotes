#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord + Clone> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root.take() {
            None => {
                self.root = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
                self.size += 1;
            }
            Some(node) => {
                self.root = Some(Self::insert_node(node, value, &mut self.size));
            }
        }
    }

    fn insert_node(mut node: Box<Node<T>>, value: T, size: &mut usize) -> Box<Node<T>> {
        match value.cmp(&node.value) {
            std::cmp::Ordering::Less => {
                match node.left.take() {
                    None => {
                        node.left = Some(Box::new(Node {
                            value,
                            left: None,
                            right: None,
                        }));
                        *size += 1;
                    }
                    Some(left_node) => {
                        node.left = Some(Self::insert_node(left_node, value, size));
                    }
                }
            }
            std::cmp::Ordering::Greater => {
                match node.right.take() {
                    None => {
                        node.right = Some(Box::new(Node {
                            value,
                            left: None,
                            right: None,
                        }));
                        *size += 1;
                    }
                    Some(right_node) => {
                        node.right = Some(Self::insert_node(right_node, value, size));
                    }
                }
            }
            std::cmp::Ordering::Equal => {
                // Value already exists, don't insert duplicate
            }
        }
        node
    }

    pub fn contains(&self, value: &T) -> bool {
        Self::contains_node(&self.root, value)
    }

    fn contains_node(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Less => Self::contains_node(&n.left, value),
                std::cmp::Ordering::Greater => Self::contains_node(&n.right, value),
            },
        }
    }

    pub fn remove(&mut self, value: &T) -> bool {
        let (new_root, removed) = Self::remove_node(self.root.take(), value);
        self.root = new_root;
        if removed {
            self.size -= 1;
        }
        removed
    }

    fn remove_node(node: Option<Box<Node<T>>>, value: &T) -> (Option<Box<Node<T>>>, bool) {
        match node {
            None => (None, false),
            Some(mut n) => match value.cmp(&n.value) {
                std::cmp::Ordering::Less => {
                    let (new_left, removed) = Self::remove_node(n.left.take(), value);
                    n.left = new_left;
                    (Some(n), removed)
                }
                std::cmp::Ordering::Greater => {
                    let (new_right, removed) = Self::remove_node(n.right.take(), value);
                    n.right = new_right;
                    (Some(n), removed)
                }
                std::cmp::Ordering::Equal => {
                    match (n.left.take(), n.right.take()) {
                        (None, None) => (None, true),
                        (Some(left), None) => (Some(left), true),
                        (None, Some(right)) => (Some(right), true),
                        (Some(left), Some(right)) => {
                            let (min_value, new_right) = Self::extract_min(right);
                            let new_node = Box::new(Node {
                                value: min_value,
                                left: Some(left),
                                right: new_right,
                            });
                            (Some(new_node), true)
                        }
                    }
                }
            },
        }
    }

    fn extract_min(mut node: Box<Node<T>>) -> (T, Option<Box<Node<T>>>) {
        match node.left.take() {
            None => (node.value, node.right),
            Some(left) => {
                let (min_value, new_left) = Self::extract_min(left);
                node.left = new_left;
                (min_value, Some(node))
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

// Example usage
fn main() {
    let mut tree = BinaryTree::new();
    
    // Insert some values
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(9);
    
    println!("Tree length: {}", tree.len());
    
    // Test contains
    println!("Contains 5: {}", tree.contains(&5));
    println!("Contains 8: {}", tree.contains(&8));
    
    // Test remove
    println!("Removing 3: {}", tree.remove(&3));
    println!("Tree length after removal: {}", tree.len());
    println!("Contains 3 after removal: {}", tree.contains(&3));
    
    // Clear the tree
    tree.clear();
    println!("Tree length after clear: {}", tree.len());
    println!("Is empty: {}", tree.is_empty());
}
