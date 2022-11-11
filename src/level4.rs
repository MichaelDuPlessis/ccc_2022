use core::panic;
use std::fmt::Debug;

use petgraph::data::Build;
use petgraph::visit::GraphProp;
use petgraph::{prelude::UnGraph, graph};

const LEVEL: &str = "level4";

use petgraph::{algo, prelude::*};

use petgraph::{prelude::UnGraph, graph::NodeIndex};

const LEVEL: &str = "level4";

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Entity {
    Coin,
    Ghost,
    Wall,
    Pacman,
    Space,
}

#[derive(Debug)]
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

        let working_nodes = self.graph.node_weights();

        let coin_indices = Vec::new();

        for (index, weight) in nodes.enumerate() {
            if *weight == Entity::Coin {
                coin_indices.push(index);
            } 
        }

        let start_pos = (0, 0);

        let start_node = graph.node_indices().last().unwrap();

        let mut all_paths = Vec::new();

        let mut coins_left = 0;

        while 
        for (index, node) in working_nodes.enumerate() {
            coins_left = 0;
            if *node == Entity::Coin {
                all_paths.push(algo::all_simple_paths::<Vec<_>, _>(&graph, start_node, index as NodeIndex, 0, None).collect::<Vec<_>>());
                coins_left += 1;
            }
        }

        println!("end");

        println!("{:?}", self.graph);
        
    }
}