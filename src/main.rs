use std::{io::stdout, process};
use crossterm::{ExecutableCommand, cursor, terminal::{Clear, ClearType}, event::{read, Event, KeyCode}, style::{self, Colorize}};
use rand::Rng;

struct Board {
    array: [[u16; 4]; 4],
}

trait Size {
    fn size(&self) -> u16;
}

trait Moves {
    fn move_right(&mut self);
    fn move_left(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    //fn get_a(&mut self) -> &mut [[u16; 4]; 4];
}

impl Default for Board {
    fn default() -> Board {
        // TODO there might be a better way to store this. We only have 12 possible values
        // TODO variable board size
        let mut array = [[0; 4]; 4];
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(1, 16);
        let mut b;
        loop {
            // need to ensure a != b
            // TODO there must be nicer way to pick 2 unique numbers from a range
            b = rng.gen_range(1, 16);
            if b != a { break; }
        }

        // TODO give chance to something other than 2
        array[a / 4][a % 4] = 2;
        array[b / 4][b % 4] = 2;

        Board {
            array: array,
        }
    }
}

impl Moves for Board {
    fn move_right(&mut self) {
        // we're going down from size-2. Rightmost column will never move to the right
        for j in 0..self.size() {
            for i in (0..(self.size() - 1)).rev() {
                if self.array[j as usize][(i + 1) as usize ] == 0 {
                    self.array[j as usize][(i + 1) as usize ] = self.array[j as usize][i as usize ];
                    self.array[j as usize][i as usize ] = 0;
                }
            }
        }
    }

    fn move_left(&mut self) {
        // we're going up from 1 to size-1. Leftmost column will never move to the left
        for j in 0..self.size() {
            for i in 1..self.size() {
                if self.array[j as usize][(i - 1) as usize ] == 0 {
                    self.array[j as usize][(i - 1) as usize ] = self.array[j as usize][i as usize ];
                    self.array[j as usize][i as usize ] = 0;
                }
            }
        }
    }

    fn move_up(&mut self) {
        // we're going up from 0 to size-1 but this time on the other axis. Bottom row will never move down
        for j in 1..self.size() {
            for i in 0..self.size() {
                if self.array[(j - 1) as usize][i as usize ] == 0 {
                    self.array[(j - 1) as usize][i as usize ] = self.array[j as usize][i as usize ];
                    self.array[j as usize][i as usize ] = 0;
                }
            }
        }
    }

    fn move_down(&mut self) {
        // we're going down from size-2 but this time on the other axis. Top row will never move up
        for j in (0..(self.size() - 1)).rev() {
            for i in 0..self.size() {
                if self.array[(j + 1) as usize][i as usize ] == 0 {
                    self.array[(j + 1) as usize][i as usize ] = self.array[j as usize][i as usize ];
                    self.array[j as usize][i as usize ] = 0;
                }
            }
        }
    }
    
    // fn get_a(&mut self) -> &mut [[u16; 4]; 4] {
    //     &mut self.array
    // }
}

impl Size for Board {
    fn size(&self) -> u16 {
        // TODO remove hardcode
        4
    }
}

// TODO this probably should be in a Display trait
fn print_matrix(arr: &[[u16; 4]; 4]) {
    // 2D version
    // for (i, item) in arr.iter().enumerate() {
    //     print!("{:3} ", item);
    //     // shift it back to non-zero indexing so we avoid 0 remainder from the 0 index
    //     if (i + 1) % 4 == 0 {
    //         println!();
    //     }
    // }

    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).expect("UH OH");
    stdout.execute(cursor::MoveTo(0, 0)).expect("UH OH");
    
    println!("Welcome to the Matrix. You know the drill. Just use the arrows");
    println!();

    for (_i, row) in arr.iter().enumerate() {
        for (_j, item) in row.iter().enumerate() {
            if *item == 0 {
                print!("{:3} ", item);
            } else {
                // bit hacky but we need to do the format first, we can only add color to string or &str
                print!("{} ", format!("{:3}", item).yellow());
            }
        }
        println!();
    }
}

fn tear_down() {
    println!("Kabbe gyikok");
    process::exit(0);
}

fn main() {
    let mut b = Board::default();
    print_matrix(&b.array);
    
    loop {
        // `read()` blocks until an `Event` is available
        match read().expect("OH NOES") {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => b.move_up(),
                    KeyCode::Down => b.move_down(),
                    KeyCode::Left => b.move_left(),
                    KeyCode::Right => b.move_right(),
                    KeyCode::Esc => tear_down(),
                    _ => (),
                }
            }
            _ => (), // we don't care about mouse and resize events for now
        }
        print_matrix(&b.array);
    }
}
