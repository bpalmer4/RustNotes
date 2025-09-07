// Minimalistic Double-Linked List Implementation
//
// A generic doubly-linked list with basic operations. Uses Rc<RefCell> for safe shared ownership.
// Most operations work with any type T, while comparison-based operations (has, remove_val) 
// require T to implement PartialEq.
//
// Design choices:
// - Uses Rc<RefCell<Node<T>>> for next pointers (owning, shared references)
// - Uses Weak<RefCell<Node<T>>> for prev pointers (non-owning, breaks reference cycles)
// - This approach eliminates all unsafe code and enables trivial memory management
// - clear() simply drops references; reference counting handles cleanup automatically
// - Trade-off: Runtime overhead from reference counting vs memory safety and simplicity

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
    length: usize,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.take(),
            prev: None,
        }));
        
        if let Some(ref old_head) = new_node.borrow().next {
            old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        } else {
            self.tail = Some(Rc::downgrade(&new_node));
        }
        
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn push_end(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: self.tail.clone(),
        }));
        
        if let Some(old_tail) = self.tail.as_ref().and_then(|w| w.upgrade()) {
            old_tail.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        
        self.tail = Some(Rc::downgrade(&new_node));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            } else {
                self.tail = None;
            }
            self.length -= 1;
            
            // Extract the data from the Rc<RefCell<Node<T>>>
            match Rc::try_unwrap(old_head) {
                Ok(cell) => cell.into_inner().data,
                Err(_) => panic!("Multiple references to node during pop"),
            }
        })
    }

    pub fn pop_end(&mut self) -> Option<T> {
        self.tail.as_ref()?.upgrade().map(|old_tail| {
            let prev = old_tail.borrow_mut().prev.take();
            
            if let Some(new_tail) = prev.as_ref().and_then(|w| w.upgrade()) {
                new_tail.borrow_mut().next = None;
                self.tail = Some(Rc::downgrade(&new_tail));
            } else {
                self.head = None;
                self.tail = None;
            }
            
            self.length -= 1;
            
            match Rc::try_unwrap(old_tail) {
                Ok(cell) => cell.into_inner().data,
                Err(_) => panic!("Multiple references to node during pop_end"),
            }
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.length = 0;
    }

    pub fn reverse(&mut self) {
        let mut current = self.head.clone();
        let mut new_head = None;
        
        while let Some(node) = current {
            let next = node.borrow().next.clone();
            
            // Swap next and prev pointers using tuple destructuring
            let old_prev = node.borrow().prev.as_ref().and_then(|w| w.upgrade());
            (node.borrow_mut().next, node.borrow_mut().prev) = (
                old_prev,
                new_head.as_ref().map(|n| Rc::downgrade(n))
            );
            
            new_head = Some(node);
            current = next;
        }
        
        // Swap head and tail using tuple destructuring
        let old_tail = self.tail.as_ref().and_then(|w| w.upgrade());
        (self.head, self.tail) = (
            new_head,
            old_tail.map(|t| Rc::downgrade(&t))
        );
    }
}

impl<T: PartialEq> DoubleLinkedList<T> {
    pub fn has(&self, value: &T) -> bool {
        let mut current = self.head.clone();
        
        while let Some(node) = current {
            if node.borrow().data == *value {
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

    pub fn remove_val(&mut self, value: &T) -> bool {
        let mut current = self.head.clone();
        
        while let Some(node) = current {
            if node.borrow().data == *value {
                self.remove_node(node);
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }
    
    fn remove_node(&mut self, node: Rc<RefCell<Node<T>>>) {
        let prev = node.borrow().prev.as_ref().and_then(|w| w.upgrade());
        let next = node.borrow().next.clone();
        
        match (prev, next) {
            (Some(p), Some(n)) => {
                p.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(Rc::downgrade(&p));
            }
            (None, Some(n)) => {
                n.borrow_mut().prev = None;
                self.head = Some(n);
            }
            (Some(p), None) => {
                p.borrow_mut().next = None;
                self.tail = Some(Rc::downgrade(&p));
            }
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
        }
        
        self.length -= 1;
    }
}

fn main() {
    let mut list = DoubleLinkedList::new();
    
    println!("=== Testing push and pop operations ===");
    
    // Test push (front)
    list.push(1);
    list.push(2);
    list.push(3);
    println!("After push(1,2,3): length {}", list.len());
    
    // Test push_end (back)
    list.push_end(4);
    list.push_end(5);
    println!("After push_end(4,5): length {}", list.len());
    
    // Pop from front
    println!("Pop front: {:?}", list.pop());
    println!("Pop front: {:?}", list.pop());
    
    // Pop from back
    println!("Pop back: {:?}", list.pop_end());
    println!("Current length: {}", list.len());
    
    println!("\n=== Testing clear functionality ===");
    list.push_end(10);
    list.push_end(20);
    list.push_end(30);
    
    println!("Before clear: length {}", list.len());
    list.clear();
    println!("After clear: length {}", list.len());
    
    // Test operations on empty list
    println!("Pop on empty: {:?}", list.pop());
    println!("Pop_end on empty: {:?}", list.pop_end());
    
    println!("\n=== Testing has and remove_val ===");
    list.push_end(100);
    list.push_end(200);
    list.push_end(300);
    list.push_end(400);
    
    println!("List contents (popping): ");
    let mut temp_list = DoubleLinkedList::new();
    temp_list.push_end(100);
    temp_list.push_end(200);
    temp_list.push_end(300);
    temp_list.push_end(400);
    while let Some(val) = temp_list.pop() {
        print!("{} ", val);
    }
    println!();
    
    println!("Has 200: {}", list.has(&200));
    println!("Has 999: {}", list.has(&999));
    
    println!("Remove 200: {}", list.remove_val(&200));
    println!("Has 200 after remove: {}", list.has(&200));
    println!("Remove 999 (not exists): {}", list.remove_val(&999));
    
    println!("Remove first (100): {}", list.remove_val(&100));
    println!("Remove last (400): {}", list.remove_val(&400));
    println!("Length after removes: {}", list.len());
    
    println!("\n=== Testing edge cases ===");
    list.clear();
    
    // Single element
    list.push(42);
    println!("Single element - has 42: {}", list.has(&42));
    println!("Single element - pop: {:?}", list.pop());
    println!("Length after single pop: {}", list.len());
    
    // Push and pop_end mix
    list.push(1);
    list.push_end(2);
    list.push(3);
    list.push_end(4);
    
    println!("Mixed operations - final order:");
    while let Some(val) = list.pop() {
        print!("{} ", val);
    }
    println!();

    println!("\n=== Testing reverse ===");
    list.push_end(1);
    list.push_end(2);
    list.push_end(3);
    list.push_end(4);
    
    println!("Before reverse:");
    let mut temp_list = DoubleLinkedList::new();
    temp_list.push_end(1);
    temp_list.push_end(2);
    temp_list.push_end(3);
    temp_list.push_end(4);
    while let Some(val) = temp_list.pop() {
        print!("{} ", val);
    }
    println!();
    
    list.reverse();
    
    println!("After reverse:");
    while let Some(val) = list.pop() {
        print!("{} ", val);
    }
    println!();
    
    println!("\n=== Testing memory cleanup ===");
    
    // Create nodes and track reference counts
    list.push_end(1);
    list.push_end(2);
    list.push_end(3);
    
    // Get reference to nodes to check their ref counts
    let first_node = list.head.as_ref().unwrap().clone();
    let second_node = first_node.borrow().next.as_ref().unwrap().clone();
    let third_node = second_node.borrow().next.as_ref().unwrap().clone();
    
    println!("Before clear:");
    println!("Node 1 ref count: {}", Rc::strong_count(&first_node));
    println!("Node 2 ref count: {}", Rc::strong_count(&second_node));  
    println!("Node 3 ref count: {}", Rc::strong_count(&third_node));
    println!("List length: {}", list.len());
    
    // Clear the list
    list.clear();
    
    println!("\nAfter clear:");
    println!("Node 1 ref count: {}", Rc::strong_count(&first_node));
    println!("Node 2 ref count: {}", Rc::strong_count(&second_node));
    println!("Node 3 ref count: {}", Rc::strong_count(&third_node));
    println!("List length: {}", list.len());
    
    // Drop our local references
    drop(first_node);
    drop(second_node); 
    drop(third_node);
    
    println!("\nAfter dropping local references:");
    println!("List is empty: {}", list.len() == 0);
    println!("Head is None: {}", list.head.is_none());
    println!("Tail is None: {}", list.tail.is_none());
    
    println!("\n=== Memory cleanup test completed ===");
    println!("If ref counts drop to 1 after clear, memory will be freed when local refs are dropped");
    println!("\n=== All tests completed ===");
}