use crossterm::{
    cursor,
    style::Colorize,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{io::stdout, fmt};

mod utils;
use utils::{gib_empty_loc, gib_num};

#[cfg(test)]
mod board_tests;

pub trait Moves {
    fn move_right(&mut self) -> bool;
    fn move_left(&mut self) -> bool;
    fn move_up(&mut self) -> bool;
    fn move_down(&mut self) -> bool;
}

pub struct Board {
    array: [[u16; 4]; 4],
}

impl Board {
    pub fn spawn(&mut self) {
        let (i, j) = gib_empty_loc(self.array);
        self.array[i as usize][j as usize] = gib_num();
    }

    pub fn size(&self) -> u16 {
        // TODO better error handling (it will be more important later when things are dynamic)
        if self.array.len() != self.array[0].len() {
            panic!("Array size mismatch!");
        }
        self.array.len() as u16
    }

    pub fn moves_available(&self) -> bool {
        // TODO this is not a full logic yet, determining whether there are moves should also involve merges
        self.contains_num(0)
    }

    pub fn is_won(&self) -> bool {
        // TODO the number should be configurable eventually
        self.contains_num(2048)
    }

    fn contains_num(&self, num: u16) -> bool {
        let mut is_zero = false;

        for j in 0..self.size() {
            for i in 0..self.size() {
                if self.array[i as usize][j as usize] == num {
                    is_zero = true;
                }
            }
        }

        is_zero
    }

    // TODO might not need this
    // fn set_board(&mut self, array: [[u16; 4]; 4]) {
    //     self.array = array;
    // }
}

impl Default for Board {
    fn default() -> Board {
        // TODO there might be a better way to store this than u16. We only have 12 possible values.
        // I just thought u16 is small enough and now it is all over the code
        // TODO variable board size
        let mut array = [[0; 4]; 4];
        let (ai, aj) = gib_empty_loc(array);
        let (bi, bj) = loop {
            // need to ensure that the 2 indices are different
            // TODO though actually do we? :) if we don't, we can just use spawn()
            let (tmpi, tmpj) = gib_empty_loc(array);
            if ai != tmpi || aj != tmpj {
                break (tmpi, tmpj);
            }
        };

        array[ai as usize][aj as usize] = gib_num();
        array[bi as usize][bj as usize] = gib_num();

        Board { array: array }
    }
}

impl Moves for Board {
    fn move_right(&mut self) -> bool {
        let mut moved = false;

        // we're going down from size-2. Rightmost column will never move to the right
        for row in 0..self.size() {
            let mut merged = false;

            for col in (0..(self.size() - 1)).rev() {
                // if the current item is 0, nothing to do
                if self.array[row as usize][col as usize] == 0 {
                    continue;
                }

                let mut zeros_to_the_right = 0;
                // this loop will go "ahead" and check how many zeros we have to the right (i.e. how many places we need to move the current item)
                while (col + zeros_to_the_right + 1) < self.size()
                    && self.array[row as usize][(col + zeros_to_the_right + 1) as usize] == 0
                {
                    // we might never hit this part
                    zeros_to_the_right += 1;
                }
                // the second clause is there to prevent us from moving zeros (zeros represent empty tile)
                if zeros_to_the_right > 0 {
                    self.array[row as usize][(col + zeros_to_the_right) as usize] =
                        self.array[row as usize][col as usize];
                    self.array[row as usize][col as usize] = 0;
                    moved = true;
                }

                // finally, let's check if we ended up next to a number that is same as the current one (except if it's the rightmost column)
                // if it's the same, let's add them together
                if (col + zeros_to_the_right + 1) < self.size()
                    && self.array[row as usize][(col + zeros_to_the_right) as usize]
                        == self.array[row as usize][(col + zeros_to_the_right + 1) as usize]
                    && !merged
                {
                    self.array[row as usize][(col + zeros_to_the_right + 1) as usize] =
                        self.array[row as usize][(col + zeros_to_the_right) as usize] * 2;
                    self.array[row as usize][(col + zeros_to_the_right) as usize] = 0;
                    moved = true;
                    merged = true;
                } else {
                    // this whole merged clause is needed so we don't double-merge in a case of:
                    // 4, 4, 2, 2 -> (move right) -> 0, 4, 4, 4
                    // this case we need some dirty flag that the rightmost 4 is from merge and we shouldn't further merge into it
                    merged = false;
                }
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;

        // we're going up from 1 to size-1. Leftmost column will never move to the left
        for row in 0..self.size() {
            let mut merged = false;

            for col in 1..self.size() {
                // if the current item is 0, nothing to do
                if self.array[row as usize][col as usize] == 0 {
                    continue;
                }

                let mut zeros_to_the_left = 0;
                // this loop will go "ahead" and check how many zeros we have to the left (i.e. how many places we need to move the current item)
                while (col - zeros_to_the_left) > 0
                    && self.array[row as usize][(col - zeros_to_the_left - 1) as usize] == 0
                {
                    // we might never hit this part
                    zeros_to_the_left += 1;
                }
                // the second clause is there to prevent us from moving zeros (zeros represent empty tile)
                if zeros_to_the_left > 0 {
                    self.array[row as usize][(col - zeros_to_the_left) as usize] =
                        self.array[row as usize][col as usize];
                    self.array[row as usize][col as usize] = 0;
                    moved = true;
                }

                // finally, let's check if we ended up next to a number that is same as the current one (except if it's the leftmost column)
                // if it's the same, let's add them together
                if (col - zeros_to_the_left) > 0
                    && self.array[row as usize][(col - zeros_to_the_left - 1) as usize]
                        == self.array[row as usize][(col - zeros_to_the_left) as usize]
                    && !merged
                {
                    self.array[row as usize][(col - zeros_to_the_left - 1) as usize] =
                        self.array[row as usize][(col - zeros_to_the_left) as usize] * 2;
                    self.array[row as usize][(col - zeros_to_the_left) as usize] = 0;
                    moved = true;
                    merged = true;
                } else {
                    // this whole merged clause is needed so we don't double-merge in a case of:
                    // 2, 2, 4, 4 -> (move left) -> 4, 4, 4, 0
                    // this case we need some dirty flag that the leftmost 4 is from merge and we shouldn't further merge into it
                    merged = false;
                }
            }
        }
        moved
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;

        // we're going up from 0 to size-1 but this time on the other axis. Bottom row will never move down
        for row in 1..self.size() {
            let mut merged = false;

            for col in 0..self.size() {
                // if the current item is 0, nothing to do
                if self.array[row as usize][col as usize] == 0 {
                    continue;
                }

                let mut zeros_to_the_top = 0;
                // this loop will go "ahead" and check how many zeros we have to the left (i.e. how many places we need to move the current item)
                while (row - zeros_to_the_top) > 0
                    && self.array[(row - zeros_to_the_top - 1) as usize][col as usize] == 0
                {
                    // we might never hit this part
                    zeros_to_the_top += 1;
                }
                // the second clause is there to prevent us from moving zeros (zeros represent empty tile)
                if zeros_to_the_top > 0 {
                    self.array[(row - zeros_to_the_top) as usize][col as usize] =
                        self.array[row as usize][col as usize];
                    self.array[row as usize][col as usize] = 0;
                    moved = true;
                }

                // finally, let's check if we ended up next to a number that is same as the current one (except if it's the bottom column)
                // if it's the same, let's add them together
                if (row - zeros_to_the_top) > 0
                    && self.array[(row - zeros_to_the_top) as usize][col as usize]
                        == self.array[(row - zeros_to_the_top - 1) as usize][col as usize]
                    && !merged
                {
                    self.array[(row - zeros_to_the_top - 1) as usize][col as usize] =
                        self.array[(row - zeros_to_the_top) as usize][col as usize] * 2;
                    self.array[(row - zeros_to_the_top) as usize][col as usize] = 0;
                    moved = true;
                    merged = true;
                } else {
                    merged = false;
                }
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;

        // we're going down from size-2 but this time on the other axis. Top row will never move up
        for row in (0..(self.size() - 1)).rev() {
            let mut merged = false;

            for col in 0..self.size() {
                // if the current item is 0, nothing to do
                if self.array[row as usize][col as usize] == 0 {
                    continue;
                }

                let mut zeros_to_the_bottom = 0;
                // this loop will go "ahead" and check how many zeros we have to the left (i.e. how many places we need to move the current item)
                while (row + zeros_to_the_bottom + 1) < self.size()
                    && self.array[(row + zeros_to_the_bottom + 1) as usize][col as usize] == 0
                {
                    // we might never hit this part
                    zeros_to_the_bottom += 1;
                }
                // the second clause is there to prevent us from moving zeros (zeros represent empty tile)
                if zeros_to_the_bottom > 0 {
                    self.array[(row + zeros_to_the_bottom) as usize][col as usize] =
                        self.array[row as usize][col as usize];
                    self.array[row as usize][col as usize] = 0;
                    moved = true;
                }

                // finally, let's check if we ended up next to a number that is same as the current one (except if it's the bottom column)
                // if it's the same, let's add them together
                if (row + zeros_to_the_bottom + 1) < self.size()
                    && self.array[(row + zeros_to_the_bottom) as usize][col as usize]
                        == self.array[(row + zeros_to_the_bottom + 1) as usize][col as usize]
                    && !merged
                {
                    self.array[(row + zeros_to_the_bottom + 1) as usize][col as usize] =
                        self.array[(row + zeros_to_the_bottom) as usize][col as usize] * 2;
                    self.array[(row + zeros_to_the_bottom) as usize][col as usize] = 0;
                    moved = true;
                    merged = true;
                } else {
                    merged = false;
                }
            }
        }
        moved
    }
}

impl fmt::Display for Board {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        for (_i, row) in self.array.iter().enumerate() {
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
        Ok(())
    }
}
