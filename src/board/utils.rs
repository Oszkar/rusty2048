use rand::Rng;

pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down
}

pub fn gib_num() -> u16 {
    let mut rng = rand::thread_rng();
    // TODO is 5% chance for a 4 OK? Should it increase in later stages of the game?
    let val: f32 = rng.gen();
    if val <= 0.95 {
        2
    } else {
        4
    }
}

pub fn gib_empty_loc(arr: [[u16; 4]; 4]) -> (u16, u16) {
    let mut rng = rand::thread_rng();

    // TODO what if there is no empty?
    let idx = loop {
        let i = rng.gen_range(0, 4);
        let j = rng.gen_range(0, 4);
        // need to ensure that we are trying to spawn on an empty location
        if arr[i as usize][j as usize] == 0 {
            break (i, j);
        }
    };
    idx
}
