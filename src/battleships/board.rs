#[repr(u8)]
pub enum TileState {
    Hidden,
    Blank,
    Opponent,
    Player,
}

pub struct Board {
    pub cols: u8,
    pub rows: u8,
    pub tiles: Vec<TileState>
}

fn save_board(b: Board) {
    // write!(f, b.cols); // 
    // write!(f, b.rows); //
    //for tile in b.tiles {
        // write!(f, tile)
    // } 
    let as_u8: Vec<u8> = b.into();
    // write!(f, as_u8);

    // let v = read!(f); // Vec<u8>
    // let b:Board = v.into();
}

fn transmit_board(b: Board) {
    // write!(network, b.cols); // 
    // write!(f, b.rows); //
    //for tile in b.tiles {
        // write!(f, tile)
    // } 
}

impl From<Board> for Vec<u8> {
    fn from(board: Board) -> Vec<u8> {
        // write!(f, b.cols); // 
        // write!(f, b.rows); //
        //for tile in b.tiles {
            // write!(f, tile)
        // }
        let out: Vec<u8> = Vec::with_capacity((board.cols * board.rows + 2) as usize);
 

        Vec::new()
    }
}

impl Into<Board> for Vec<u8>  {
    fn into(self) -> Board {
        Board {
            rows: 0,
            cols: 0,
            tiles: Vec::new()
        }
    }
}