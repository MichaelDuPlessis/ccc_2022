mod level1;
mod level2;

fn main() {
    for i in 1..6 {
        level2::Game::new(&format!("level2_{}", i)).move_pac();
    }
}
