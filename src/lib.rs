use std::{char, collections::HashMap};

pub struct TrieNode {
    pub _letter: char,
    pub finished: bool,
    pub children: HashMap<char, TrieNode>
}

pub struct AetherWall {
    pub root: TrieNode
}

impl TrieNode {
    pub fn new(letter: char) -> TrieNode {
        Self { _letter: letter, finished: false, children: HashMap::new() }
    }
}

impl AetherWall {
    pub fn new() -> AetherWall {
        Self { root: TrieNode::new(' ') }
    }

    pub fn insert(&mut self, domain: &str) { 
        let mut current_node = &mut self.root;

        let mut segments = domain.split('.').rev().peekable();

        while let Some(segment) = segments.next() {
            for char in segment.chars() {
                current_node = current_node.children.entry(char).or_insert_with(|| TrieNode::new(char))
            }
            if segments.peek().is_some() {
                current_node = current_node.children.entry('.').or_insert_with(|| TrieNode::new('.'));
            }
        }
        current_node.finished = true;
    }

    pub fn contains(&self, domain: &str) -> bool {
        let mut current_node = &self.root;

        let mut segments = domain.split('.').rev().peekable();
        while let Some(segment) = segments.next() {
            for char in segment.chars() {
                if let Some(node) = current_node.children.get(&char) {
                    current_node = node;
                } else {
                    return false;
                }
            }

            if segments.peek().is_some() {
                if let Some(node) = current_node.children.get(&'.') {
                    current_node = node;
                    if current_node.finished { return true; }
                } else {
                    return false;
                }
            }
        }

        current_node.finished
    }
}