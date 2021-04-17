use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
    style::Colorize,
};
use std::{process, env};
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
    if env::consts::OS == "linux" {
        // set it back to normal
        disable_raw_mode().expect("Failed to set raw mode. You might need to press ENTER after each key press.");
    }

    // TODO better messages
    match done {
        EndState::UserQuit => {
            println!("Thanks for playing. Bye!\r");
            process::exit(0);
        }
        EndState::Win => {
            println!("{}", "Congratulations, you won!\r".green());
            process::exit(0);
        }
        EndState::Lose => {
            println!("{}", ":(\r".dark_red());
            process::exit(0);
        }
        EndState::Error => {
            println!("Ooh la la, something went wrong\r");
            process::exit(0);
        }
    }
}

fn main() {
    let mut b = Board::default();

    // crossterm raw mode: https://docs.rs/crossterm/0.17.7/crossterm/terminal/#raw-mode
    if env::consts::OS == "linux" {
        enable_raw_mode().expect("Failed to set raw mode. You might need to press ENTER after each key press.");
    }

    print!("{}", b);
    loop {
        // `read()` blocks until an `Event` is available
        match read().expect("Something went very wrong with the keyboard input.\r") {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up | KeyCode::Char('w') => { b.move_in_dir(MoveDirection::Up).expect("Move Error\r"); }
                    KeyCode::Down | KeyCode::Char('s')  => { b.move_in_dir(MoveDirection::Down).expect("Move Error\r"); }
                    KeyCode::Left | KeyCode::Char('a')  => { b.move_in_dir(MoveDirection::Left).expect("Move Error\r"); }
                    KeyCode::Right | KeyCode::Char('d')  => { b.move_in_dir(MoveDirection::Right).expect("Move Error\r"); }
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
    };
}
