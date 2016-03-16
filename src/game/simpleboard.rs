use game::board::{GoBoard, GoMove, State};

struct Group {
    indicies: Vec<usize>,
    liberties: i8,
    owner: State,
}

// A very simple GoBoard implementation where little attempt is made to optimize space and
// time complexities.
pub struct SimpleBoard {
    pub intersections: Vec<State>,
    groups: Vec<Group>,
    xsize: i8,
    ysize: i8,
}

impl GoBoard for SimpleBoard {
    fn new(ysize: i8, xsize: i8) -> SimpleBoard {
        SimpleBoard {
            intersections: vec![State::Empty; ((xsize as i32)*(ysize as i32)) as usize],
            groups: Vec::new(),
            xsize: xsize,
            ysize: ysize
        }
    }

    fn play_move(&mut self, mv: GoMove) {
        match mv {
            GoMove::Move {x, y, player} => self.play_at_coordinates(x, y, player),
            GoMove::Pass => {},
        };
    }
}

impl SimpleBoard {
    fn play_at_coordinates(&mut self, x: i8, y: i8, player: State) {
        // First check if there is already a stone here
        let index = self.coordinates_to_index(x, y);
        if self.intersections[index.unwrap()] != State::Empty {
            // not a valid move
        }
        else {
            // If our move was valid, add it to the board
            self.intersections[index.unwrap()] = player;
            let neighbors = self.neighbor_indicies(x, y);
            // Update the neighboring groups if there are any
            let mut group_indicies: Vec<usize> = Vec::new();
            let mut num_neighbors: i8 = 0;
            for neighbor in neighbors.iter() {
                match *neighbor {
                    Some(x) => {
                        let group_index = self.group_ref(x);
                        if group_index != None {
                            group_indicies.push(group_index.unwrap());
                        }
                        num_neighbors += 1;
                    },
                    None => {},
                }
            }
            group_indicies.sort();
            group_indicies.dedup();
            let added_to_group = self.update_groups(&group_indicies, index.unwrap(), player);
            if !added_to_group {
                // There were no neighboring groups, so we have to add one
                self.groups.push(Group{
                    indicies: vec![index.unwrap(); 1],
                    liberties: num_neighbors,
                    owner: player,
                })
            }
        }
    }

    fn group_ref(&mut self, index: usize) -> Option<usize> {
        let mut ret: Option<usize> = None;
        for (i, group) in self.groups.iter().enumerate() {
            if group.indicies.contains(&index) {
                ret = Some(i);
            }
        }
        ret
    }

    fn update_groups(&mut self, group_indicies: &Vec<usize>, play_index: usize, player: State) -> bool {
        let mut groups_to_merge: Vec<usize> = Vec::new();
        let mut added_to_group = false;
        for i in group_indicies.iter() {
            if self.groups[*i].owner == player {
                // Add the stone to the group
                self.groups[*i].indicies.push(play_index);
                groups_to_merge.push(*i);
                added_to_group = true;
            }
            else {
                // Attaching to an enemy group, this will reduce liberties by 1
                self.groups[*i].liberties -= 1;
            }
        }

        if groups_to_merge.len() > 1 {
            // We have friendly groups that the played stone merged together
            // group indices is already sorted so loop in reverse order, ignoring the
            // first group element
            for (i, gidx) in groups_to_merge.iter().rev().enumerate() {
                if i != 0 {
                    let other_group_copy: Vec<usize> = self.groups[*gidx].indicies.clone();
                    self.groups[groups_to_merge[0]].indicies.extend_from_slice(other_group_copy.as_slice());
                    self.groups.remove(*gidx);
                }
            }
        }
        // Kill off any groups that were killed by playing the immediate stone
        self.remove_all_dead_groups();
        // Recalculate all liberties
        self.recalculate_liberties();
        added_to_group
    }

    fn recalculate_liberties(&mut self) {
        let mut new_liberties: Vec<i8> = Vec::new();
        for group in &self.groups {
            let mut liberties: i8 = 0;
            let mut all_neighbors: Vec<Option<usize>> = Vec::new();
            for stone in &group.indicies {
                let (x, y) = self.index_to_coordinates(*stone);
                let neighbor = self.neighbor_indicies(x, y);

                all_neighbors.extend_from_slice(&neighbor);
            }
            all_neighbors.sort();
            all_neighbors.dedup();
            for n in all_neighbors.iter() {
                match *n {
                    Some(x) => {
                        if self.intersections[x] == State::Empty {
                            liberties += 1;
                        }
                    }
                    None => {},
                }
            }
            new_liberties.push(liberties);
        }
        for (i, l) in new_liberties.iter().enumerate() {
            self.groups[i].liberties = *l;
        }
    }

    fn remove_all_dead_groups(&mut self) {
        // Remove all groups with 0 liberties
        let mut groups_to_kill: Vec<usize> = Vec::new();
        for (i, group) in self.groups.iter().enumerate() {
            if group.liberties == 0 {
                groups_to_kill.push(i);
            }
        }
        // Group indicies will already be sorted, so iterate in reverse order and remove
        // them from the board
        for i in groups_to_kill.iter().rev() {
            self.remove_group(*i);
        }
    }

    fn remove_group(&mut self, group_index: usize) {
        for index in self.groups[group_index].indicies.iter() {
            self.intersections[*index] = State::Empty;
        }
        self.groups.remove(group_index);
    }

    fn neighbor_indicies(&self, x: i8, y: i8) -> [Option<usize>; 4] {
        [self.coordinates_to_index(x-1, y),
         self.coordinates_to_index(x+1, y),
         self.coordinates_to_index(x, y-1),
         self.coordinates_to_index(x, y+1)]
    }

    fn coordinates_to_index(&self, x: i8, y: i8) -> Option<usize> {
        let index: i32 = ((x as i32) + (y as i32)*(self.xsize as i32)) as i32;
        if index < 0 || index >= (self.xsize as i32)*(self.ysize as i32) {
            None
        }
        else {
            Some(((x as i32) + (y as i32)*(self.xsize as i32)) as usize)
        }
    }

    fn index_to_coordinates(&self, index: usize) -> (i8, i8) {
        let x = (index % (self.xsize as usize)) as i8;
        let y = (index / (self.xsize as usize)) as i8;
        (x, y)
    }

    pub fn print_board(&self) {
        // upper boundary
        // println!("{}", iter::repeat("_").take(10).collect::<String>());

        for y in 0..self.ysize {
            for x in 0..self.xsize {
                let index = (x + y*self.xsize) as usize;
                print!("|{:?}", self.intersections[index]);
            }
            println!("|", );
        }

        // lower boundary
        // println!("{}", iter::repeat("_").take(10).collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::board::*;

    #[test]
    fn play_move() {
        let mut game_board = SimpleBoard::new(3, 4);
        let mv = GoMove::Move{x: 2, y: 2, player: State::White};
        game_board.play_move(mv);
    }

    #[test]
    #[should_panic]
    fn play_move_out_of_bounds() {
        let mut game_board = SimpleBoard::new(3, 3);
        let mv = GoMove::Move{x: 3, y: 3, player: State::White};
        game_board.play_move(mv);
    }

    #[test]
    fn corner_capture() {
        let mut game_board = SimpleBoard::new(3, 3);
        let mv1 = GoMove::Move{ x: 0, y: 0, player: State::White};
        let mv2 = GoMove::Move{ x: 0, y: 1, player: State::Black};
        let mv3 = GoMove::Move{ x: 1, y: 0, player: State::Black};

        game_board.play_move(mv1);
        game_board.play_move(mv2);
        game_board.play_move(mv3);

        assert_eq!(State::Empty, game_board.intersections[0]);
        assert_eq!(State::Black, game_board.intersections[1]);
        assert_eq!(State::Black, game_board.intersections[3]);
    }

    #[test]
    fn pass() {
        let mut game_board = SimpleBoard::new(3, 3);
        let mv = GoMove::Pass;
        game_board.play_move(mv);
    }

    #[test]
    fn merge_groups() {
        let mut game_board = SimpleBoard::new(3, 3);
        let mv1 = GoMove::Move{ x: 0, y: 0, player: State::Black};
        let mv2 = GoMove::Move{ x: 2, y: 0, player: State::Black};
        let mv3 = GoMove::Move{ x: 1, y: 0, player: State::Black};

        game_board.play_move(mv1);
        game_board.play_move(mv2);

        assert_eq!(game_board.groups.len(), 2);

        game_board.play_move(mv3);

        assert_eq!(game_board.groups.len(), 1);
    }

    #[test]
    fn capture_with_one_eye() {
        let mut game_board = SimpleBoard::new(2, 2);
        let mv1 = GoMove::Move{ x: 0, y: 1, player: State::Black };
        let mv2 = GoMove::Move{ x: 1, y: 1, player: State::Black };
        let mv3 = GoMove::Move{ x: 1, y: 0, player: State::Black };

        game_board.play_move(mv1);
        game_board.play_move(mv2);
        game_board.play_move(mv3);

        let mv4 = GoMove::Move{ x: 0, y: 0, player: State::White };

        game_board.play_move(mv4);

        assert_eq!(State::White, game_board.intersections[0]);
        assert_eq!(State::Empty, game_board.intersections[1]);
        assert_eq!(State::Empty, game_board.intersections[2]);
        assert_eq!(State::Empty, game_board.intersections[3]);

        assert_eq!(game_board.groups.len(), 1);

        assert_eq!(game_board.groups[0].liberties, 2);
    }
}
