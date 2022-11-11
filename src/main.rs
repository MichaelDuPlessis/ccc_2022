mod level1;
mod level2;
pub mod level3;

fn main() {
    for i in 1..8 {
        level3::Game::new(&format!("level4_{}", i)).move_pac();
    }

    level3::Game::new(&format!("level4_{}", "example")).move_pac();
}
