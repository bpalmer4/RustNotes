// Complete AVL Tree Implementation
//
// A self-balancing binary search tree where the heights of the two child 
// subtrees of any node differ by at most one. Both insertion and deletion
// maintain this property through rotations, guaranteeing O(log n) operations.

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: u8,
}

#[derive(Debug)]
pub struct AvlTree<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord + Clone + std::fmt::Display + std::fmt::Debug> AvlTree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    fn node_height(node: &Option<Box<Node<T>>>) -> u8 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn update_height(node: &mut Node<T>) {
        node.height = 1 + Self::node_height(&node.left).max(Self::node_height(&node.right));
    }

    fn balance_factor(node: &Node<T>) -> i8 {
        Self::node_height(&node.left) as i8 - Self::node_height(&node.right) as i8
    }

    fn rotate_right(mut root: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = root.left.take().unwrap();
        root.left = new_root.right.take();
        Self::update_height(&mut root);
        new_root.right = Some(root);
        Self::update_height(&mut new_root);
        new_root
    }

    fn rotate_left(mut root: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = root.right.take().unwrap();
        root.right = new_root.left.take();
        Self::update_height(&mut root);
        new_root.left = Some(root);
        Self::update_height(&mut new_root);
        new_root
    }

    fn rebalance(mut node: Box<Node<T>>) -> Box<Node<T>> {
        Self::update_height(&mut node);
        let balance = Self::balance_factor(&node);
        
        // Left heavy
        if balance > 1 {
            let left_balance = if let Some(ref left) = node.left {
                Self::balance_factor(left)
            } else { 0 };
            
            // Left-Right case
            if left_balance < 0 {
                node.left = Some(Self::rotate_left(node.left.take().unwrap()));
            }
            // Left-Left case
            Self::rotate_right(node)
        }
        // Right heavy 
        else if balance < -1 {
            let right_balance = if let Some(ref right) = node.right {
                Self::balance_factor(right)
            } else { 0 };
            
            // Right-Left case
            if right_balance > 0 {
                node.right = Some(Self::rotate_right(node.right.take().unwrap()));
            }
            // Right-Right case
            Self::rotate_left(node)
        }
        // Already balanced
        else {
            node
        }
    }

    pub fn insert(&mut self, value: T) {
        let (new_root, inserted) = Self::insert_node(self.root.take(), value);
        self.root = new_root;
        if inserted {
            self.size += 1;
        }
    }

    fn insert_node(node: Option<Box<Node<T>>>, value: T) -> (Option<Box<Node<T>>>, bool) {
        match node {
            None => {
                let new_node = Box::new(Node {
                    value,
                    left: None,
                    right: None,
                    height: 1,
                });
                (Some(new_node), true)
            }
            Some(mut n) => {
                let inserted = match value.cmp(&n.value) {
                    std::cmp::Ordering::Less => {
                        let (left, ins) = Self::insert_node(n.left.take(), value);
                        n.left = left;
                        ins
                    }
                    std::cmp::Ordering::Greater => {
                        let (right, ins) = Self::insert_node(n.right.take(), value);
                        n.right = right;
                        ins
                    }
                    std::cmp::Ordering::Equal => false, // No duplicates
                };
                
                let result_node = if inserted { Self::rebalance(n) } else { n };
                (Some(result_node), inserted)
            }
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
            Some(mut n) => {
                let removed = match value.cmp(&n.value) {
                    std::cmp::Ordering::Less => {
                        let (left, rem) = Self::remove_node(n.left.take(), value);
                        n.left = left;
                        rem
                    }
                    std::cmp::Ordering::Greater => {
                        let (right, rem) = Self::remove_node(n.right.take(), value);
                        n.right = right;
                        rem
                    }
                    std::cmp::Ordering::Equal => {
                        return match (n.left.take(), n.right.take()) {
                            (None, None) => (None, true),
                            (Some(left), None) => (Some(left), true),
                            (None, Some(right)) => (Some(right), true),
                            (Some(left), Some(right)) => {
                                let (successor, new_right) = Self::extract_min(right);
                                let mut new_node = Box::new(Node {
                                    value: successor,
                                    left: Some(left),
                                    right: new_right,
                                    height: 1,
                                });
                                Self::update_height(&mut new_node);
                                (Some(Self::rebalance(new_node)), true)
                            }
                        };
                    }
                };
                
                let result_node = if removed { Self::rebalance(n) } else { n };
                (Some(result_node), removed)
            }
        }
    }

    fn extract_min(mut node: Box<Node<T>>) -> (T, Option<Box<Node<T>>>) {
        match node.left.take() {
            None => (node.value, node.right),
            Some(left) => {
                let (min_val, new_left) = Self::extract_min(left);
                node.left = new_left;
                let rebalanced = Self::rebalance(node);
                (min_val, Some(rebalanced))
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            match value.cmp(&node.value) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => current = &node.left,
                std::cmp::Ordering::Greater => current = &node.right,
            }
        }
        false
    }

    pub fn len(&self) -> usize { 
        self.size 
    }

    pub fn is_empty(&self) -> bool { 
        self.size == 0 
    }

    pub fn height(&self) -> u8 { 
        Self::node_height(&self.root) 
    }

    pub fn clear(&mut self) { 
        self.root = None; 
        self.size = 0; 
    }

    pub fn print_root(&self) {
        match &self.root {
            None => println!("Root: None (empty tree)"),
            Some(node) => {
                println!("Root: {} (height: {}, balance: {}, left: {:?}, right: {:?})", 
                    node.value,
                    node.height,
                    Self::balance_factor(node),
                    node.left.as_ref().map(|n| &n.value),
                    node.right.as_ref().map(|n| &n.value)
                );
            }
        }
    }

    // Verify AVL property (for debugging)
    pub fn is_balanced(&self) -> bool {
        Self::check_balanced(&self.root).is_some()
    }

    fn check_balanced(node: &Option<Box<Node<T>>>) -> Option<u8> {
        match node {
            None => Some(0),
            Some(n) => {
                let left_height = Self::check_balanced(&n.left)?;
                let right_height = Self::check_balanced(&n.right)?;
                
                let balance = left_height as i8 - right_height as i8;
                if balance.abs() > 1 {
                    None // Not balanced
                } else {
                    Some(1 + left_height.max(right_height))
                }
            }
        }
    }
}

fn main() {
    let mut tree = AvlTree::new();
    
    println!("=== AVL Tree Test: Insert 1-10 ===");
    for i in 1..=10 {
        tree.insert(i);
        println!("After inserting {}: height {}, balanced: {}", 
                 i, tree.height(), tree.is_balanced());
        tree.print_root();
    }
    
    println!("\n=== Remove 1, 2, 3 ===");
    for i in 1..=3 {
        tree.remove(&i);
        println!("After removing {}: {} nodes, height {}, balanced: {}", 
                 i, tree.len(), tree.height(), tree.is_balanced());
        tree.print_root();
    }
    
    println!("\n=== Insert 11-25 ===");
    for i in 11..=25 {
        tree.insert(i);
        println!("After inserting {}: {} nodes, height {}, balanced: {}", 
                 i, tree.len(), tree.height(), tree.is_balanced());
        tree.print_root();
    }
    
    println!("\n=== Final Verification ===");
    println!("Final tree: {} nodes, height {}", tree.len(), tree.height());
    println!("Is balanced: {}", tree.is_balanced());
    println!("Contains 3: {}", tree.contains(&3));
    println!("Contains 20: {}", tree.contains(&20));
}