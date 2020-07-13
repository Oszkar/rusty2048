#[path = "utils.rs"] mod utils; // TODO why do I need to do this? Should I put stuff in subfolders?
use utils::{gib_loc, gib_num};

pub trait Moves {
    fn move_right(&mut self) -> bool;
    fn move_left(&mut self) -> bool;
    fn move_up(&mut self) -> bool;
    fn move_down(&mut self) -> bool;
}

pub struct Board {
    // TODO remove pub
    pub array: [[u16; 4]; 4],
}

impl Board {
    pub fn spawn(&mut self) {
        let (i, j) = gib_loc(self.array);
        self.array[i as usize][j as usize] = gib_num();
    }

    pub fn size(&self) -> u16 {
        // TODO remove hardcode
        4
    }

    pub fn moves_available(&self) -> bool {
        // TODO this is not a full logic yet, determining whether there are moves should also involve merges
        self.contains_zero()
    }

    fn contains_zero(&self) -> bool {
        let mut is_zero = false;

        for j in 0..self.size() {
            for i in 0..self.size() {
                if self.array[i as usize][j as usize] == 0 { is_zero = true; }
            }
        }

        is_zero
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
        for row in 0..self.size() {
            for col in (0..(self.size() - 1)).rev() {
                let mut zeros_to_the_right = 0;
                // this loop will go "ahead" and check how many zeros we have to the right (i.e. how many places we need to move the current item)
                while (col + zeros_to_the_right + 1) < self.size() && self.array[row as usize][(col + zeros_to_the_right + 1) as usize ] == 0 {
                    // we might never hit this part
                    zeros_to_the_right += 1;
                }
                // the second clause is there to prevent us from moving zeros (zeros represent empty tile)
                if zeros_to_the_right > 0 && self.array[row as usize][col as usize ] != 0 {
                    self.array[row as usize][(col + zeros_to_the_right) as usize ] = self.array[row as usize][col as usize];
                    self.array[row as usize][col as usize ] = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;

        // we're going up from 1 to size-1. Leftmost column will never move to the left
        for row in 0..self.size() {
            for col in 1..self.size() {
                let mut zeros_to_the_left = 0;
                // this loop will go "ahead" and check how many zeros we have to the left (i.e. how many places we need to move the current item)
                while (col - zeros_to_the_left) > 0 && self.array[row as usize][(col - zeros_to_the_left - 1) as usize ] == 0 {
                    // we might never hit this part
                    zeros_to_the_left += 1;
                }
                // the second clause is there to prevent us from moving zeros (zeros represent empty tile)
                if zeros_to_the_left > 0 && self.array[row as usize][col as usize ] != 0 {
                    self.array[row as usize][(col - zeros_to_the_left) as usize ] = self.array[row as usize][col as usize];
                    self.array[row as usize][col as usize ] = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;

        // we're going up from 0 to size-1 but this time on the other axis. Bottom row will never move down
        for row in 1..self.size() {
            for col in 0..self.size() {
                let mut zeros_to_the_top = 0;
                // this loop will go "ahead" and check how many zeros we have to the left (i.e. how many places we need to move the current item)
                while (row - zeros_to_the_top) > 0 && self.array[(row - zeros_to_the_top - 1) as usize][col as usize ] == 0 {
                    // we might never hit this part
                    zeros_to_the_top += 1;
                }
                // the second clause is there to prevent us from moving zeros (zeros represent empty tile)
                if zeros_to_the_top > 0 && self.array[row as usize][col as usize ] != 0 {
                    self.array[(row - zeros_to_the_top) as usize][col as usize ] = self.array[row as usize][col as usize];
                    self.array[row as usize][col as usize ] = 0;
                    moved = true;
                }
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;

        // we're going down from size-2 but this time on the other axis. Top row will never move up
        for row in (0..(self.size() - 1)).rev() {
            for col in 0..self.size() {
                let mut zeros_to_the_bottom = 0;
                // this loop will go "ahead" and check how many zeros we have to the left (i.e. how many places we need to move the current item)
                while (row + zeros_to_the_bottom + 1) < self.size() && self.array[(row + zeros_to_the_bottom + 1) as usize][col as usize ] == 0 {
                    // we might never hit this part
                    zeros_to_the_bottom += 1;
                }
                // the second clause is there to prevent us from moving zeros (zeros represent empty tile)
                if zeros_to_the_bottom > 0 && self.array[row as usize][col as usize ] != 0 {
                    self.array[(row + zeros_to_the_bottom) as usize][col as usize ] = self.array[row as usize][col as usize];
                    self.array[row as usize][col as usize ] = 0;
                    moved = true;
                }
            }
        }
        moved
    }
}