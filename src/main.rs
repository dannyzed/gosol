mod game;
use game::simpleboard::{SimpleBoard};
use game::board::{GoBoard};

fn main() {
    let game_board = SimpleBoard::new(4, 4);
    game_board.print_board();
}
