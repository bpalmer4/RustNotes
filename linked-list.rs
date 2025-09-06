// Simple single linked list

use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T>
where
    T: PartialEq + Debug,
{
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    // Push a value to the front of the list
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    // Pop a value from the front of the list
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    // Remove the nth element (0-indexed) from the list
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        if index == 0 {
            return self.pop();
        }

        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        if let Some(node) = current {
            if let Some(target) = node.next.take() {
                node.next = target.next;
                self.size -= 1;
                return Some(target.data);
            }
        }
        None
    }

    // Check if the list contains a value
    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if &node.data == value {
                return true;
            }
            current = &node.next;
        }
        false
    }

    // Get the size of the list
    pub fn len(&self) -> usize {
        self.size
    }

    // Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    // Empty the list
	pub fn clear(&mut self) {
    	self.head = None;
	    self.size = 0;
	}
}

// --- Example usage
fn main() {
    let mut list = LinkedList::new();
    
    // Push some values
    list.push(1);
    list.push(2);
    list.push(3);
    println!("List after pushing 1, 2, 3: {:?}", list);
    
    // Check contains
    println!("Contains 2: {}", list.contains(&2));
    println!("Contains 5: {}", list.contains(&5));
    
    // Pop a value
    if let Some(value) = list.pop() {
        println!("Popped: {}", value);
    }
    println!("List after pop: {:?}", list);
    
    // Remove by index (index 0)
    match list.remove(0) {
        Some(value) => println!("Removed at index 0: {}", value),
        None => println!("Failed to remove at index 0"),
    }
    println!("List after remove: {:?}", list);
    
    // Add more elements for testing
    list.push(4);
    list.push(5);
    list.push(6);
    println!("List after pushing 4, 5, 6: {:?}", list);
    
    // Remove by non-zero index (middle element)
    if let Some(value) = list.remove(1) {
        println!("Removed at index 1: {}", value);
    } else {
        println!("Failed to remove at index 1");
    }
    println!("List after removing index 1: {:?}", list);
    
    // Try to remove out of bounds
    match list.remove(10) {
        Some(value) => println!("Removed at index 10: {}", value),
        None => println!("Failed to remove at index 10 (out of bounds)"),
    }
    println!("List after attempting out-of-bounds remove: {:?}", list);
    
    println!("List length: {}", list.len());
    
    // Clear the list 
    list.clear();
    println!("List after clear: {:?}", list);
    println!("Is empty: {}", list.is_empty());
}