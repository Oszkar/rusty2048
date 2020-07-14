use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    style::Colorize,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{io::stdout, process};
// TODO I might want to play with the log crate

mod board;
use board::{Board, Moves};

#[allow(dead_code)]
enum EndState {
    Win,
    Lose,
    Error,
    UserQuit,
}

// TODO this probably should be in a Display trait
fn print_matrix(arr: &[[u16; 4]; 4]) {
    let mut stdout = stdout();
    // TODO should we panic here just because screen operation failed? It is quite unexpected...
    stdout
        .execute(Clear(ClearType::All))
        .expect("Terminal screen clearing failed.");
    stdout
        .execute(cursor::MoveTo(0, 0))
        .expect("Terminal cursor updat failed.");

    println!(
        "{}",
        "Welcome to the Matrix. You know the drill. Just use the arrows".blue()
    );
    println!();

    for (_i, row) in arr.iter().enumerate() {
        for (_j, item) in row.iter().enumerate() {
            // TODO maybe add some sort of colormap instead of big if-else
            if *item == 0 {
                print!("{:3} ", item);
            } else if *item == 2 || *item == 4 {
                // bit hacky but we need to do the format first, we can only add color to string or &str
                print!("{} ", format!("{:3}", item).yellow());
            } else if *item == 8 || *item == 16 {
                print!("{} ", format!("{:3}", item).magenta());
            } else if *item == 32 || *item == 64 {
                print!("{} ", format!("{:3}", item).cyan());
            } else if *item == 128 || *item == 256 {
                print!("{} ", format!("{:3}", item).blue());
            } else if *item == 512 || *item == 1024 {
                print!("{} ", format!("{:3}", item).green());
            } else if *item == 2048 {
                print!("{} ", format!("{:3}", item).red());
            }
        }
        println!();
    }
}

fn tear_down(done: EndState) {
    println!();
    // TODO better messages
    match done {
        EndState::UserQuit => {
            println!("Kabbe gyikok");
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
    print_matrix(&b.array);
    loop {
        // `read()` blocks until an `Event` is available
        match read().expect("Something went very wrong with the keyboard input.") {
            Event::Key(event) => {
                match event.code {
                    // TODO move this logic into Board
                    KeyCode::Up => {
                        if b.move_up() {
                            b.spawn();
                        }
                    }
                    KeyCode::Down => {
                        if b.move_down() {
                            b.spawn();
                        }
                    }
                    KeyCode::Left => {
                        if b.move_left() {
                            b.spawn();
                        }
                    }
                    KeyCode::Right => {
                        if b.move_right() {
                            b.spawn();
                        }
                    }
                    KeyCode::Esc => tear_down(EndState::UserQuit),
                    _ => (),
                }
            }
            _ => (), // we don't care about mouse and resize events for now
        }
        print_matrix(&b.array);

        // after the move, check the state
        if b.is_won() {
            tear_down(EndState::Win);
        } else if !b.moves_available() {
            tear_down(EndState::Lose);
        }
    }
}
