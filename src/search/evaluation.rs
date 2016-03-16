use game::simpleboard::SimpleBoard;
use game::board::*;

trait Evaluation {
    fn evaluate(&self) -> i64;
}

impl Evaluation for SimpleBoard {
    fn evaluate(&self) -> i64 {
        // For now just count the difference between black and white stones
        let mut difference: i64 = 0;

        for state in self.intersections.iter() {
            if *state == State::White {
                difference -= 1;
            }
            else if *state == State::Black {
                difference += 1;
            }
        }
        difference
    }
}


#[cfg(test)]
mod tests {
    use game::simpleboard::*;
    use game::board::*;
    use search::evaluation::Evaluation;

    #[test]
    fn evaluate() {
        let mut game_board = SimpleBoard::new(3, 3);

        let mv1 = GoMove::Move{x: 0, y: 0, player: State::White};
        game_board.play_move(mv1);
        assert_eq!(game_board.evaluate(), -1);
        let mv2 = GoMove::Move{x: 1, y: 0, player: State::Black};
        game_board.play_move(mv2);
        let mv3 = GoMove::Move{x: 1, y: 1, player: State::Black};
        game_board.play_move(mv3);
        assert_eq!(game_board.evaluate(), 1);
    }
}
