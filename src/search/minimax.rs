use game::simpleboard::SimpleBoard;
use search::evaluation::Evaluation;

pub fn minimax(board: &mut SimpleBoard, depth: i8, maximizing_player: bool) -> i64 {
    if depth == 0 {
        board.evaluate()
    }
    else {
        // Generate a list of all possible moves
        // for each move, play it out and continue
        // we either need to generate all moves beforehand or clone the board and keep track
        // of the full move list, we should calculate how much memory a typical full tree will
        // use
        if maximizing_player {
            let v = minimax(board, depth - 1, false);
            // return max(v, best_value)
        }
        else {
            let v = minimax(board, depth - 1, true);
        }
    }
}
