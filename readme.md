## Rusty2048

My Rust implementation of the famous little game. This is my first ever Rust code so bear with me.
Some stuff is mixed between methods or traits, I'm just experimenting. I'm pretty sure I'm violating best practices here and there, input is welcome.

In it's current state the game is not fully complete, tests remain to be added, bugs might be present.
I am a heretic and commiting to master, I'll create release branches later.

## Usage

* Check out code
* `cargo run`
* press arrow keys on the keyboard
* drink responsibly

## Current TODOs/known issues
* arrow keys handling does not seem to work on linux (at least on WSL)
* moves_available() function's logic is not complete yet
* need to clear out some TODOs as commented

## Roadmap

* v0.1.* - I am going to keep everything in hardcoded 4*4 array, command line only
* v0.2.* - Change to dynamic sizing/conatiners, command line only
* v0.3.* - Add some sort of GUI
* v1.* - lol nope, this is just a sandbox, I don't think it will ever be decent enough to call v1