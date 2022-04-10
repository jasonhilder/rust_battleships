use std::{thread, time};
use super::board::Board;
// use super::player::PlayState;

enum State { // horrible name
    MainMenu,
    Lobby,
    Paused,
    Playing
} 

enum PlayState {
    None,
    Player,
    Opponent,
    Won,
    Lost,
}

struct Input {
    // input needs to be handled on the UI screens
    // as well as the actual board
}

pub struct World {
    renderer: Box<dyn Renderer + 'static>,
    board: Board,
    game_state: State,
    play_state: PlayState,
}

trait Renderer {
    // fn new() -> ;
    fn clear(&self);
    fn render(&self, board: &Board);
    fn destroy(&self);
}

trait InputHandler {
    fn process_input(world: &mut World);
}


struct MouseHandler;
impl InputHandler for MouseHandler {
    fn process_input(world: &mut World) {
        /*

            get_x_y_mouse

            let mut tile = render.tile_at(...); // mapping the X,Y of the mouse to the renderer and then return a tile
        
            if Some(tile) = tile {
                //the user clicked on a tile, take action
                world.tile_hit(tile); 
            }
            */
    }
}

struct Keyboard {
    buffer: Vec<u8>,
}

impl InputHandler for Keyboard {
    fn process_input(world: &mut World)  {
        /*
        if key == "enter" {
            let x = buffer[0];
            let y = buffer[1];

            let tile = renderer.tile_at(x,y);

            if Some(tile) = tile {
                //the user clicked on a tile, take action
                world.tile_hit(tile); 
            }
            buffer.clear();
        } else {
            buffer.push(key)
        }

        1st key = x, // is the first keybetween 0 - 9
        2nd key = y // same
        enter = hit // over here

        */
    }
}

// Terminal renderer, webGL, JS dom render

struct Terminal {
    width: usize,
    height: usize,
}

impl Renderer for Terminal {
    fn clear(&self) {
        // clear terminal
    }
    
    fn render(&self, board: &Board) {
        /*

            80x25
          1. clear screen
          2. position at 1,1
          3. iterate over columns and print char at and then at col 80, print \n
          
            map Player = 'A'
            map Enemy = 'B';
            map Hidden = '='
        */
    }

    fn destroy(&self) {
        // clear screen?
        self.clear();
    }
}


impl World {
    pub fn new(renderer: Box<impl Renderer + 'static>, input: Box<impl InputHandler + 'static>) -> Self {
        Self{
            renderer: renderer,
            board: Board{
                cols: 10,
                rows: 10,
                tiles: Vec::new()
            },
            game_state: State::MainMenu,
            play_state: PlayState::None
        }
    }

    /// Render to view depending on the game_state
    pub fn process(&self) {
        // input.process_input(self);

        println!("render!");
        match &self.game_state {
            State::MainMenu => println!("render main menu"),
            _ => println!("")

        }

        self.renderer.render(&self.board);

        thread::sleep(time::Duration::from_secs(5));
    }
}