#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    Empty,
    White,
    Black,
}

// A representation of a single move played on the go board.
pub enum GoMove {
    Move { x: i8, y: i8, player: State},
    Pass
}

// Trait representing the go board.  These are the required attributes in order
// to perform the search over the board.
pub trait GoBoard {
    // Initializes an empty go board with sizes xsize and ysize
    fn new(xsize: i8, ysize: i8) -> Self;

    // Plays a given move on the goboard
    fn play_move(&mut self, GoMove);
}
