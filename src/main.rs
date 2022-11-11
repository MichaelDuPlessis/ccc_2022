mod level1;
mod level2;
pub mod level3;

fn main() {
    for i in 1..6 {
        level3::Game::new(&format!("level3_{}", i)).move_pac();
    }

    level2::Game::new(&format!("level3_{}", "example")).move_pac();
}
