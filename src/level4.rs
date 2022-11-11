<<<<<<< HEAD
use core::panic;
use std::fmt::Debug;

use petgraph::{prelude::UnGraph, graph};

const LEVEL: &str = "level4";

use petgraph::{algo, prelude::*};

=======
use petgraph::{prelude::UnGraph, graph::NodeIndex};

const LEVEL: &str = "level4";

>>>>>>> 208f4b846bd8a57b08c3e7002146c86ab0a239bc
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Entity {
    Coin,
    Ghost,
    Wall,
    Pacman,
    Space,
}

<<<<<<< HEAD


=======
#[derive(Debug)]
>>>>>>> 208f4b846bd8a57b08c3e7002146c86ab0a239bc
pub struct Game {
    graph: UnGraph<Entity, ()>,
    max_moves: usize,
    size: usize,
}

impl Game {
    pub fn new(file: &str) -> Self {
        let mut graph = UnGraph::new_undirected();

        let input = std::fs::read_to_string(format!("./{}/{}.in", LEVEL, file)).unwrap();
        let mut lines = input.lines();
        let size = lines.next().unwrap().parse::<usize>().unwrap();

        let mut max_moves= 0;

        for (i, line) in lines.enumerate() {
            if i < size {
                for (j, c) in line.chars().enumerate() {
                     match c {
                        'P' => {
                            graph.add_node(Entity::Pacman);
                        },
                        'C' => {
                            graph.add_node(Entity::Coin);
                        },
                        'G' => {
                            graph.add_node(Entity::Ghost);
                        },
                        'W' => {
                            graph.add_node(Entity::Wall);
                        },
                        _ => panic!("Why cruel world"),
                    };
                }
            }

            if i == size + 1 {
                max_moves = line.parse::<usize>().unwrap();
            }
        }

        // adding edges
        for i in 0..size {
            for j in 0.. size {
                if i != size - 1 {
                    graph.update_edge(NodeIndex::new(i + 1), NodeIndex::new(j), ());
                }
                if i != 0 {
                    graph.update_edge(NodeIndex::new(i - 1), NodeIndex::new(j), ());
                }
                if j != size - 1 {
                    graph.update_edge(NodeIndex::new(i), NodeIndex::new(j + 1), ());
                }
                if j != 0 {
                    graph.update_edge(NodeIndex::new(i), NodeIndex::new(j - 1), ());
                }
            }
        }

        Self {
            graph,
            size,
            max_moves,
        }
    }

    pub fn calculate(&self) {
        let mut graph = self.graph.clone();

        let ways = algo::all_simple_paths::<Vec<_>, _>(&graph, a, d, 0, None).collect::<Vec<_>>();

        for way in &ways {
            println!("{:?}", way.clone());
        }  

        let nodes = graph.node_weights();

        for weight in nodes {
            
        }

        println!("end");

        println!("{:?}", self.graph);
        
    }
}