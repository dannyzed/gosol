#![feature(test)]

extern crate gosol;
extern crate test;
extern crate rand;

use test::Bencher;
use gosol::game::simpleboard::*;
use gosol::game::board::*;
use rand::Rng;


#[bench]
fn bench_play_200_moves(b: &mut Bencher) {
    b.iter(|| {
        let mut game_board = SimpleBoard::new(19, 19);

        for i in 0..200 {
            let x = rand::thread_rng().gen_range(0, 19);
            let y = rand::thread_rng().gen_range(0, 19);

            if i % 2 == 0 {
                game_board.play_move(GoMove::Move{x: x, y: y, player: State::Black});
            }
            else {
                game_board.play_move(GoMove::Move{x: x, y: y, player: State::White});
            }
        }
    })
}

#[bench]
fn bench_play_10_moves_small(b: &mut Bencher) {
    b.iter(|| {
        let mut game_board = SimpleBoard::new(10, 10);

        for i in 0..10 {
            let x = rand::thread_rng().gen_range(0, 10);
            let y = rand::thread_rng().gen_range(0, 10);

            if i % 2 == 0 {
                game_board.play_move(GoMove::Move{x: x, y: y, player: State::Black});
            }
            else {
                game_board.play_move(GoMove::Move{x: x, y: y, player: State::White});
            }
        }
    })
}
