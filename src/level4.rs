use core::panic;
use std::fmt::Debug;

use petgraph::data::Build;
use petgraph::visit::GraphProp;
use petgraph::algo;
use petgraph::graph::NodeIndex;
use petgraph::{prelude::UnGraph, graph};

const LEVEL: &str = "level4";

#[derive(Debug, Clone)]
struct Coord(usize, usize);

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Coin,
    Ghost,
    Wall,
    Pacman,
    Space,
}

#[derive(Debug, Clone)]
pub struct Entity {
    kind: Kind,
    pos: Coord,
}

impl Entity {
    fn new(kind: Kind, pos: Coord) -> Self {
        Self { kind, pos }
    }
}

#[derive(Debug)]
pub struct Game {
    graph: UnGraph<Entity, ()>,
    max_moves: usize,
    size: usize,
    pac_pos: Coord,
}

impl Game {
    pub fn new(file: &str) -> Self {
        let mut graph = UnGraph::new_undirected();

        let input = std::fs::read_to_string(format!("./{}/{}.in", LEVEL, file)).unwrap();
        let mut lines = input.lines();
        let size = lines.next().unwrap().parse::<usize>().unwrap();

        let mut max_moves = 0;
        let mut pac_pos = Coord(0, 0);

        for (i, line) in lines.enumerate() {
            if i < size {
                for (j, c) in line.chars().enumerate() {
                    let kind = match c {
                        'P' => {
                            pac_pos = Coord(i, j);
                            Kind::Pacman
                        }
                        'C' => Kind::Coin,
                        'G' => Kind::Ghost,
                        'W' => Kind::Wall,
                        _ => panic!("Why cruel world"),
                    };

                    graph.add_node(Entity::new(kind, Coord(i, j)));
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
                    if i + 1 != j {
                        if graph[NodeIndex::new(i + 1 + j * size)].kind != Kind::Wall
                            && graph[NodeIndex::new(i + 1 + j * size)].kind != Kind::Ghost
                        {
                            graph.update_edge(NodeIndex::new(i + 1), NodeIndex::new(j), ());
                        }
                    }
                }
                if i != 0 {
                    if i - 1 != j {
                        if graph[NodeIndex::new(i - 1 + j * size)].kind != Kind::Wall
                            && graph[NodeIndex::new(i - 1 + j * size)].kind != Kind::Ghost
                        {
                            graph.update_edge(NodeIndex::new(i - 1), NodeIndex::new(j), ());
                        }
                    }
                }
                if j != size - 1 {
                    if i != j + 1 {
                        if graph[NodeIndex::new(i + (j + 1) * size)].kind != Kind::Wall
                            && graph[NodeIndex::new(i + (j + 1) * size)].kind != Kind::Ghost
                        {
                            graph.update_edge(NodeIndex::new(i), NodeIndex::new(j + 1), ());
                        }
                    }
                }
                if j != 0 {
                    if i != j - 1 {
                        if graph[NodeIndex::new(i + (j - 1) * size)].kind != Kind::Wall
                            && graph[NodeIndex::new(i + (j - 1) * size)].kind != Kind::Ghost
                        {
                            graph.update_edge(NodeIndex::new(i), NodeIndex::new(j - 1), ());
                        }
                    }
                }
            }
        }

        Self {
            graph,
            size,
            max_moves,
            pac_pos,
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