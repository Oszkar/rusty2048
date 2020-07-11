use rand::Rng;

pub fn gib_num() -> u16 {
    // TODO give chance to something other than 2
    2
}

pub fn gib_loc(arr: [[u16; 4]; 4]) -> (u16, u16) {
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