pub struct Board {
    positions: [char; 9],
    pub count: usize
}

impl Board {
    pub fn new() -> Self {
        return Board {
            positions: [' '; 9],
            count: 0
        };
    }

    pub fn size(&self) -> usize {
        return self.positions.len()
    }

    pub fn print_board(&self) {
        println!(" {} | {} | {} " , self.positions[0], self.positions[1], self.positions[2]);
        println!(" {} | {} | {} " , self.positions[3], self.positions[4], self.positions[5]);
        println!(" {} | {} | {} " , self.positions[6], self.positions[7], self.positions[8]);
    }

    pub fn place_marker(&mut self, place: usize, mark: char) -> bool {
        if  place < 1 || place > self.size() || self.positions[place - 1] != ' ' {
            return false
        }

        self.positions[place - 1] = mark;
        self.count += 1;
        return true
    }

    pub fn check_winner(&self) -> bool {
        let win_conditions: [[usize;3];8] = [[0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6]];

        for win in win_conditions.iter() {
            if self.positions[win[0]] == self.positions[win[1]] &&
                self.positions[win[0]] == self.positions[win[2]] &&
                self.positions[win[0]] != ' ' {
                return true
            }
        }
        return false;
    }
}
