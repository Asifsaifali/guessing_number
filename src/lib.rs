use wasm_bindgen::prelude::*;
use rand::Rng;

static mut SECRET: u32 = 0;

#[wasm_bindgen]
pub fn start_game() {
    let random = rand::thread_rng().gen_range(1..=100);
    unsafe {
        SECRET = random;
    }
}

#[wasm_bindgen]
pub fn check_guess(guess: u32) -> String {
    unsafe {
        if guess < SECRET {
            "Too small!".into()
        } else if guess > SECRET {
            "Too big!".into()
        } else {
            "You win!".into()
        }
    }
}
