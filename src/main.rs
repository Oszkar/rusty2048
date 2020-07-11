use std::{io::stdout, process};
use crossterm::{ExecutableCommand, cursor, terminal::{Clear, ClearType}, event::{read, Event, KeyCode}, style::{Colorize}};
use rand::Rng;

// TODO I guess I should move this in it's own module and make the raw array inaccessible from the outside
struct Board {
    array: [[u16; 4]; 4],
}

trait Size {
    fn size(&self) -> u16;
}

trait Moves {
    fn move_right(&mut self) -> bool;
    fn move_left(&mut self) -> bool;
    fn move_up(&mut self) -> bool;
    fn move_down(&mut self) -> bool;
}

impl Board {
    fn spawn(&mut self) {
        let (i, j) = gib_loc(self.array);
        self.array[i as usize][j as usize] = gib_num();
    }
}

impl Default for Board {
    fn default() -> Board {
        // TODO there might be a better way to store this than u16. We only have 12 possible values
        // TODO variable board size
        let mut array = [[0; 4]; 4];
        
        let (ai, aj) = gib_loc(array);
        let (bi, bj) = loop {
            // need to ensure that the 2 indices are different
            // TODO though actually do we? :) if we don't, we can just use spawn()
            let (tmpi, tmpj) = gib_loc(array);
            if ai != tmpi || aj != tmpj { break (tmpi, tmpj); }
        };

        array[ai as usize][aj as usize] = gib_num();
        array[bi as usize][bj as usize] = gib_num();

        Board {
            array: array,
        }
    }
}

impl Moves for Board {
    fn move_right(&mut self) -> bool {
        let mut moved = false;

        // we're going down from size-2. Rightmost column will never move to the right
        for j in 0..self.size() {
            for i in (0..(self.size() - 1)).rev() {
                if self.array[j as usize][(i + 1) as usize ] == 0 && self.array[j as usize][i as usize] != 0 {
                    self.array[j as usize][(i + 1) as usize ] = self.array[j as usize][i as usize];
                    self.array[j as usize][i as usize ] = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;

        // we're going up from 1 to size-1. Leftmost column will never move to the left
        for j in 0..self.size() {
            for i in 1..self.size() {
                if self.array[j as usize][(i - 1) as usize ] == 0 && self.array[j as usize][i as usize] != 0 {
                    self.array[j as usize][(i - 1) as usize ] = self.array[j as usize][i as usize];
                    self.array[j as usize][i as usize ] = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;

        // we're going up from 0 to size-1 but this time on the other axis. Bottom row will never move down
        for j in 1..self.size() {
            for i in 0..self.size() {
                if self.array[(j - 1) as usize][i as usize ] == 0 && self.array[j as usize][i as usize] != 0 {
                    self.array[(j - 1) as usize][i as usize ] = self.array[j as usize][i as usize];
                    self.array[j as usize][i as usize ] = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;

        // we're going down from size-2 but this time on the other axis. Top row will never move up
        for j in (0..(self.size() - 1)).rev() {
            for i in 0..self.size() {
                if self.array[(j + 1) as usize][i as usize ] == 0 && self.array[j as usize][i as usize] != 0 {
                    self.array[(j + 1) as usize][i as usize ] = self.array[j as usize][i as usize];
                    self.array[j as usize][i as usize ] = 0;
                    moved = true;
                }
            }
        }
        moved
    }
}

impl Size for Board {
    fn size(&self) -> u16 {
        // TODO remove hardcode
        4
    }
}

fn gib_num() -> u16 {
    // TODO give chance to something other than 2
    2
}

fn gib_loc(arr: [[u16; 4]; 4]) -> (u16, u16) {
    let mut rng = rand::thread_rng();
    
    let idx = loop {
        let i = rng.gen_range(0, 4);
        let j = rng.gen_range(0, 4);
        // need to ensure that we are trying to spawn on an empty location
        if arr[i as usize][j as usize] == 0 { break (i, j); }
    };

    // originally I did:
    //  let a = rng.gen_range(1, 16);
    //  array[a / 4][a % 4] = ...
    // but reverted back to generating 2 indices
    
    idx
}

// TODO this probably should be in a Display trait
fn print_matrix(arr: &[[u16; 4]; 4]) {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).expect("UH OH");
    stdout.execute(cursor::MoveTo(0, 0)).expect("UH OH");
    
    println!("{}", "Welcome to the Matrix. You know the drill. Just use the arrows".blue());
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
                    // TODO move this logic into Board
                    KeyCode::Up => if b.move_up() { b.spawn()},
                    KeyCode::Down => if b.move_down() { b.spawn()},
                    KeyCode::Left => if b.move_left() { b.spawn()},
                    KeyCode::Right => if b.move_right() { b.spawn()},
                    KeyCode::Esc => tear_down(),
                    _ => (),
                }
            }
            _ => (), // we don't care about mouse and resize events for now
        }
        print_matrix(&b.array);
    }
}
