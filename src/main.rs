mod level1;
mod level2;
mod level3;
mod level4;

fn main() {
    // for i in 1..8 {
    //     level3::Game::new(&format!("level4_{}", i)).move_pac();
    // }

    // level3::Game::new(&format!("level4_{}", "example")).move_pac();

    let entity = level4::Game::new(&format!("level4_{}", "example"));

    entity.calculate();
    //     level4::Game::new(&format!("level4_{}", i)).move_pac();
    // }

    println!("{:?}", level4::Game::new(&format!("level4_{}", "example")));
}
