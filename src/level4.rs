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

        let mut input = lines.collect::<String>();
        println!("{input}");
        input.retain(|c| !c.is_whitespace());

        Self {
            graph
        }
    }
}