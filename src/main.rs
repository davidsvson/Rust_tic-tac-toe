mod board;

use crate::board::Board;
use std::io;


fn main() {
    let mut board = Board::new();

    let stdin = io::stdin();

    board.print_board();

    while board.count < board.size() && !board.check_winner(){
        let mut valid_input = false;
        while !valid_input {
            let mut input = String::new();
            println!("Var vill du lägga?");
            stdin.read_line(&mut input).expect("Failed to read line");

            let place: usize = input.trim().parse().expect("Failed to parse usize");
            let marker = if board.count % 2 == 0 {'x'} else  {'o'};

            valid_input = board.place_marker(place, marker);
            if !valid_input {
                println!("Ogiltigt val")
            }
        }

        board.print_board();
    }

    if board.count == board.size() {
        println!("Vi har en vinnare!")
    } else {
        println!("Oavgjort!")
    }

}




