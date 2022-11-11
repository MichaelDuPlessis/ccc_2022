const LEVEL: &str = "level4";

use petgraph::{algo, prelude::*};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Entity {
    Coin,
    Ghost,
    Pacman,
    Space,
}

pub struct Game {

}

impl Game {
    pub fn new(file: &str) -> Self {
        let input = std::fs::read_to_string(format!("./{}/{}.in", LEVEL, file)).unwrap();

        let mut lines = input.lines();

        Self {

        }
    }

    pub fn calculate() {
        let mut graph = DiGraph::<&str, i32>::new();
        
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let c = graph.add_node("c");
        let d = graph.add_node("d");

        graph.extend_with_edges(&[(a, b, 1), (b, c, 1), (c, d, 1), (a, b, 1), (b, d, 1)]);

        let ways = algo::all_simple_paths::<Vec<_>, _>(&graph, a, d, 0, None).collect::<Vec<_>>();

        for way in &ways {
            println!("{:?}", way.clone());
        }

        println!("end");

        
    }
}