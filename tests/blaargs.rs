#[cfg(test)]

use rustboy::gameboy::GameBoy;

#[test]
fn add_two() {
    assert_eq!(2, 2);
}

#[test]
fn blaargs_1() {
    let game = "roms/";
    let mut gameboy  = GameBoy::new();
    gameboy.start_up(game);
}

