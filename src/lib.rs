#![no_std]

use pebbles_game_io::*;
use gstd::{prelude::*, msg, exec::random};
use gstd::debug;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init(){}


#[no_mangle]
extern "C" fn handle(){}


#[no_mangle]
extern "C" fn state(){}