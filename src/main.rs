mod level1;
mod level2;
mod level3;
mod level4;

fn main() {
    // for i in 1..8 {
    //     level4::Game::new(&format!("level4_{}", i)).move_pac();
    // }

    level4::Game::new(&format!("level4_{}", "example"));
}
