#![no_std]
#![warn(clippy::all)]
#![deny(missing_docs)]

//! Unknown Car Game.

mod game;

use game::GAME;
use wasm4_sx::Engine;

#[cfg(test)]
extern crate wasm4_stubs;

#[no_mangle]
fn start() {}

#[no_mangle]
fn update() {
    Engine::run_frame(|ctx| GAME.borrow_mut().run_game_frame(&ctx));
}

wasm4_sx::setup_panic_handler_w4!();
