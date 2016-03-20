use game::simpleboard::SimpleBoard;
use game::board::{State, GoBoard, GoMove};
use search::evaluation::Evaluation;

pub fn minimax(board: &mut SimpleBoard, depth: i8, maximizing_player: bool) -> (f64, Option<GoMove>) {
    if depth == 0 {
        (board.evaluate(), None)
    }
    else {
        if maximizing_player {
            let player = State::Black;
            let mut best_value: f64 = -10000.0;
            // Loop over all possible moves
            let moves = board.move_list(player);
            let mut good_move = GoMove::Pass;
            for mv in moves {
                let mut new_board = board.clone();
                new_board.play_move(&mv);
                let (v, _) = minimax(&mut new_board, depth - 1, false);
                if v > best_value {
                    best_value = v;
                    good_move = mv.clone();
                }
                best_value = v.max(best_value);
            }
            (best_value, Some(good_move))
        }
        else {
            let player = State::White;
            let mut best_value: f64 = 10000.0;
            let moves = board.move_list(player);
            let mut good_move = GoMove::Pass;
            for mv in moves {
                let mut new_board = board.clone();
                new_board.play_move(&mv);
                let (v, _) = minimax(&mut new_board, depth - 1, true);
                if v < best_value {
                    best_value = v;
                    good_move = mv.clone();
                }
                best_value = v.min(best_value);
            }
            (best_value, Some(good_move))
        }
    }
}

#[cfg(test)]
mod tests {
    use game::simpleboard::SimpleBoard;
    use game::board::*;
    use super::*;

    #[test]
    fn three_space_life() {
        let mut game_board = SimpleBoard::new(6, 4);

        let black_x = vec![0, 1, 2, 3, 3];
        let black_y = vec![1, 1, 1, 1, 0];

        for (x, y) in black_x.iter().zip(black_y.iter()) {
            let mv = GoMove::Move{x: *x, y: *y, player: State::Black};
            game_board.play_move(&mv);
        }

        let white_x = vec![0, 1, 2, 3, 4, 4, 4];
        let white_y = vec![2, 2, 2, 2, 2, 1, 0];

        for (x, y) in white_x.iter().zip(white_y.iter()) {
            let mv = GoMove::Move{x: *x, y: *y, player: State::White};
            game_board.play_move(&mv);
        }

        let (_, mv) = minimax(&mut game_board, 8, true);

        assert_eq!(mv.unwrap(), GoMove::Move{x: 1, y: 0, player: State::Black});

    }

    #[test]
    fn corner_kill() {
        let mut game_board = SimpleBoard::new(7, 5);

        let black_x = vec![0, 1, 2, 3, 4, 5, 5, 5];
        let black_y = vec![3, 3, 3, 2, 2, 2, 1, 0];

        for (x, y) in black_x.iter().zip(black_y.iter()) {
            let mv = GoMove::Move{x: *x, y: *y, player: State::Black};
            game_board.play_move(&mv);
        }

        let white_x = vec![0, 1, 2, 3, 4, 1];
        let white_y = vec![2, 2, 2, 1, 1, 0];

        for (x, y) in white_x.iter().zip(white_y.iter()) {
            let mv = GoMove::Move{x: *x, y: *y, player: State::White};
            game_board.play_move(&mv);
        }

        let (_, mv) = minimax(&mut game_board, 7, true);

        assert_eq!(mv.unwrap(), GoMove::Move{x: 0, y: 0, player: State::Black});

    }

}
