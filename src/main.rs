mod battleships;

use battleships::world::World;

fn main() {
    // initialise the first game state 
    let game = World::new();

    loop {
        game.process();
    }
}
