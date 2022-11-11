mod level1;
mod level2;
pub mod level3;

fn main() {
    for i in 1..6 {
        level2::Game::new(&format!("level2_{}", i)).move_pac();
    }

    level2::Game::new(&format!("level2_{}", "example")).move_pac();
}
