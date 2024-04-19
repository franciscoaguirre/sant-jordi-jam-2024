#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;

use bevy::prelude::*;

/// Graph where nodes are added along with their indices.
/// When adding nodes, you must be sure that they're all connected.
/// The starting node must be added with index 0.
#[derive(Resource)]
pub struct Graph<Content, Choice> {
    nodes: HashMap<usize, Node<Content, Choice>>,
    current_node: usize,
}

impl<Content: fmt::Debug, Choice: fmt::Debug + GetNextNode> fmt::Debug for Graph<Content, Choice> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node<Content: fmt::Debug, Choice: fmt::Debug + GetNextNode>(
            graph: &Graph<Content, Choice>,
            node_index: usize,
            depth: usize,
            f: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            if let Some(node) = graph.nodes.get(&node_index) {
                for _ in 0..depth {
                    write!(f, "\t")?;
                }
                write!(f, "{:?}", node)?;
                match node {
                    Node::Simple { next, .. } => {
                        if let Some(next) = next {
                            writeln!(f)?;
                            print_node(graph, *next, depth + 1, f)?;
                        }
                    }
                    Node::Fork { choices, .. } => {
                        for choice in choices.iter() {
                            writeln!(f)?;
                            print_node(graph, choice.next_node(), depth + 1, f)?;
                        }
                    }
                };
            };
            Ok(())
        }

        if self.nodes.contains_key(&0) {
            print_node(self, 0, 0, f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Node<Content, Choice> {
    Simple {
        content: Content,
        next: Option<usize>,
    },
    Fork {
        content: Content,
        choices: Vec<Choice>,
    },
}

pub trait GetNextNode {
    fn next_node(&self) -> usize;
}

impl<Content, Choice: GetNextNode> Graph<Content, Choice> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            current_node: 0,
        }
    }

    pub fn add_node(&mut self, index: usize, node: Node<Content, Choice>) {
        self.nodes.insert(index, node);
    }

    pub fn get_current_node(&self) -> &Node<Content, Choice> {
        self.nodes.get(&self.current_node).unwrap()
    }

    pub fn reset(&mut self) {
        self.current_node = 0;
    }

    pub fn get_content(&self) -> &Content {
        match self.get_current_node() {
            Node::Simple { content, .. } => content,
            Node::Fork { content, .. } => content,
        }
    }

    /// Returns whether or not the current node is a fork.
    pub fn is_fork(&self) -> bool {
        matches!(self.get_current_node(), Node::Fork { .. })
    }

    /// Current node must be simple.
    pub fn advance(&mut self) {
        match self.get_current_node() {
            Node::Simple { next, .. } => {
                if let Some(next_index) = next {
                    self.current_node = *next_index;
                }
            }
            _ => panic!("Current node was not simple."),
        }
    }

    /// Current node must be a fork.
    pub fn choose(&mut self, index: usize) {
        match self.get_current_node() {
            Node::Fork { choices, .. } => {
                self.current_node = choices[index].next_node();
            }
            _ => panic!("Current node was not a fork."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestContent {
        text: String,
    }

    #[derive(Debug)]
    struct TestChoice {
        title: String,
        subtitle: String,
        next: usize,
    }

    impl GetNextNode for TestChoice {
        fn next_node(&self) -> usize {
            self.next
        }
    }

    /// The graph looks like this:
    ///       B
    ///      /
    /// Z - A
    ///      \
    ///       C - D
    #[test]
    fn simple_graph_works() {
        let mut graph = Graph::<TestContent, TestChoice>::new();
        graph.add_node(
            0,
            Node::Simple {
                content: TestContent {
                    text: "Z".to_string(),
                },
                next: Some(1),
            },
        );
        graph.add_node(
            1,
            Node::Fork {
                content: TestContent {
                    text: "A".to_string(),
                },
                choices: vec![
                    TestChoice {
                        title: "To B".to_string(),
                        subtitle: "This choice leads to B".to_string(),
                        next: 2,
                    },
                    TestChoice {
                        title: "To C".to_string(),
                        subtitle: "This choice leads to C".to_string(),
                        next: 3,
                    },
                ],
            },
        );
        graph.add_node(
            2,
            Node::Simple {
                content: TestContent {
                    text: "B".to_string(),
                },
                next: None,
            },
        );
        graph.add_node(
            3,
            Node::Simple {
                content: TestContent {
                    text: "C".to_string(),
                },
                next: Some(4),
            },
        );
        graph.add_node(
            4,
            Node::Simple {
                content: TestContent {
                    text: "D".to_string(),
                },
                next: None,
            },
        );

        let mut texts = Vec::new();
        texts.push(graph.get_content().text.clone()); // Z
        graph.advance();
        texts.push(graph.get_content().text.clone()); // A
        graph.choose(1);
        texts.push(graph.get_content().text.clone()); // C
        graph.advance();
        texts.push(graph.get_content().text.clone()); // D
        assert_eq!(texts, vec!["Z", "A", "C", "D"]);

        let mut texts = Vec::new();
        graph.reset();
        texts.push(graph.get_content().text.clone()); // Z
        graph.advance();
        texts.push(graph.get_content().text.clone()); // A
        graph.choose(0);
        texts.push(graph.get_content().text.clone()); // B
        assert_eq!(texts, vec!["Z", "A", "B"]);
    }
}
