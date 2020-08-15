use crossterm::{
    event::{read, Event, KeyCode},
    style::Colorize,
};
use std::{process};
// TODO I might want to play with the log crate

mod board;
use board::{Board, Moves, utils::MoveDirection};

#[allow(dead_code)]
enum EndState {
    Win,
    Lose,
    Error,
    UserQuit,
}

fn tear_down(done: EndState) {
    println!();
    // TODO better messages
    match done {
        EndState::UserQuit => {
            println!("Thanks for playing. Bye!");
            process::exit(0);
        }
        EndState::Win => {
            println!("{}", "Congratulations, you won!".green());
            process::exit(0);
        }
        EndState::Lose => {
            println!("{}", ":(".dark_red());
            process::exit(0);
        }
        EndState::Error => {
            println!("Ooh la la, something went wrong");
            process::exit(0);
        }
    }
}

fn main() {
    let mut b = Board::default();
    print!("{}", b);
    loop {
        // `read()` blocks until an `Event` is available
        match read().expect("Something went very wrong with the keyboard input.") {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up | KeyCode::Char('w') => { b.move_in_dir(MoveDirection::Up).expect("Move Error"); }
                    KeyCode::Down | KeyCode::Char('s')  => { b.move_in_dir(MoveDirection::Down).expect("Move Error"); }
                    KeyCode::Left | KeyCode::Char('a')  => { b.move_in_dir(MoveDirection::Left).expect("Move Error"); }
                    KeyCode::Right | KeyCode::Char('d')  => { b.move_in_dir(MoveDirection::Right).expect("Move Error"); }
                    KeyCode::Esc => tear_down(EndState::UserQuit),
                    _ => (),
                }
            }
            _ => (), // we don't care about mouse and resize events for now
        }
        print!("{}", b);

        // after the move, check the state
        if b.is_won() {
            tear_down(EndState::Win);
        } else if !b.moves_available() {
            tear_down(EndState::Lose);
        }
    }
}
