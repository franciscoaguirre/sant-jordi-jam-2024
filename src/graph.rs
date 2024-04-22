#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;

use bevy::prelude::*;

/// Graph where nodes are added along with their indices.
/// When adding nodes, you must be sure that they're all connected.
/// The starting node must be added with index 0.
#[derive(Resource)]
pub struct Graph<Content, Simple, Choice, Context> {
    nodes: HashMap<usize, Node<Content, Simple, Choice>>,
    current_node: usize,
    pub context: Context,
}

impl<
        Content: fmt::Debug,
        Simple: fmt::Debug,
        Choice: fmt::Debug + ChoiceTrait<Context>,
        Context: fmt::Debug,
    > fmt::Debug for Graph<Content, Simple, Choice, Context>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn print_node<
            Content: fmt::Debug,
            Simple: fmt::Debug,
            Choice: fmt::Debug + ChoiceTrait<Context>,
            Context: fmt::Debug,
        >(
            graph: &Graph<Content, Simple, Choice, Context>,
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
                            print_node(graph, choice.next_node(&graph.context), depth + 1, f)?;
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
pub enum Node<Content, Simple, Choice> {
    Simple {
        content: Content,
        extra: Simple,
        next: Option<usize>,
    },
    Fork {
        content: Content,
        choices: Vec<Choice>,
    },
}

#[macro_export]
macro_rules! arc {
    (|$closure_var:ident| $inner:expr) => {
        Arc::new(|$closure_var| $inner)
    };
    ($inner:expr) => {
        Arc::new(|_| $inner)
    };
}

pub trait ChoiceTrait<Context> {
    fn next_node(&self, context: &Context) -> usize;
    fn change_state(&self, context: &mut Context);
}

impl<Content, Simple, Choice: ChoiceTrait<Context> + Clone, Context: Default>
    Graph<Content, Simple, Choice, Context>
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            current_node: 0,
            context: Default::default(),
        }
    }

    pub fn add_node(&mut self, index: usize, node: Node<Content, Simple, Choice>) {
        self.nodes.insert(index, node);
    }

    pub fn get_current_node(&self) -> &Node<Content, Simple, Choice> {
        self.nodes.get(&self.current_node).unwrap()
    }

    pub fn set_current_node(&mut self, index: usize) {
        self.current_node = index;
    }

    pub fn reset(&mut self) {
        self.current_node = 0;
    }

    pub fn get_content(&mut self) -> &Content {
        match self.get_current_node() {
            Node::Simple { content, .. } => content,
            Node::Fork { content, .. } => content,
        }
    }

    /// Returns whether or not the current node is a fork.
    pub fn is_fork(&mut self) -> bool {
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
        let (next_node, choice) = match self.get_current_node() {
            Node::Fork { choices, .. } => {
                let next = choices[index].next_node(&self.context);
                let choice = choices[index].clone();
                (next, choice)
            }
            _ => panic!("Current node was not a fork."),
        };
        choice.change_state(&mut self.context);
        self.current_node = next_node;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestContent {
        text: String,
    }

    type TestSimple = ();

    #[derive(Debug, Clone)]
    struct TestChoice {
        title: String,
        subtitle: String,
        next: usize,
    }

    impl ChoiceTrait<TestContext> for TestChoice {
        fn next_node(&self, _: &TestContext) -> usize {
            self.next
        }

        fn change_state(&self, _: &mut TestContext) {}
    }

    #[derive(Debug, Default)]
    struct TestContext {
        some_flag: bool,
    }

    /// The graph looks like this:
    ///       B
    ///      /
    /// Z - A
    ///      \
    ///       C - D
    #[test]
    fn simple_graph_works() {
        let mut graph = Graph::<TestContent, TestSimple, TestChoice, TestContext>::new();
        graph.add_node(
            0,
            Node::Simple {
                content: TestContent {
                    text: "Z".to_string(),
                },
                extra: (),
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
                extra: (),
                next: None,
            },
        );
        graph.add_node(
            3,
            Node::Simple {
                content: TestContent {
                    text: "C".to_string(),
                },
                extra: (),
                next: Some(4),
            },
        );
        graph.add_node(
            4,
            Node::Simple {
                content: TestContent {
                    text: "D".to_string(),
                },
                extra: (),
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
