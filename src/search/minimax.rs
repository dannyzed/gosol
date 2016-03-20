use game::simpleboard::SimpleBoard;
use search::evaluation::Evaluation;

pub fn minimax(board: &mut SimpleBoard, depth: i8, maximizing_player: bool) -> f64 {
    if depth == 0 {
        board.evaluate()
    }
    else {
        if maximizing_player {
            let mut best_value: f64 = -10000.0;
            // Loop over all possible moves
            let v = minimax(board, depth - 1, false);
            best_value = v.max(best_value);
            best_value
        }
        else {
            let mut best_value: f64 = 10000.0;
            // Loop over all possible moves
            let v = minimax(board, depth - 1, false);
            best_value = v.min(best_value);
            best_value
        }
    }
}
