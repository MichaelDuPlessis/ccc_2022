use core::panic;

use petgraph::{prelude::UnGraph, graph};

const LEVEL: &str = "level4";

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Entity {
    Coin,
    Ghost,
    Wall,
    Pacman,
    Space,
}

pub struct Game {
    graph: UnGraph<Entity, ()>
}

impl Game {
    pub fn new(file: &str) -> Self {
        let mut graph = UnGraph::new_undirected();

        let mut input = std::fs::read_to_string(format!("./{}/{}.in", LEVEL, file)).unwrap();
        let mut lines = input.lines();
        let size = lines.next().unwrap().parse::<usize>().unwrap();

        for (i, line) in lines.enumerate() {
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

            if i == size - 1 {
                break;
            }
        }

        // adding edges
        for i in 0..size*size {
            if i != 0 {

            }
            
            if i == size - 1 {
                
            }
        }

        Self {
            graph
        }
    }
}