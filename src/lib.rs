#![no_std]

use pebbles_game_io::*;
use gstd::{prelude::*, msg, exec::random};
use gstd::debug;

static mut PEBBLES_GAME: Option<GameState> = None;

// get random number
#[cfg(not(feature = "testr"))]
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[cfg(feature = "testr")]
fn get_random_u32() -> u32 {
    0
}

// select the difficulty
fn determine_program_action(pebbles_remaining: u32, max_pebbles_per_turn: u32, difficulty: DifficultyLevel) -> u32 {
    match difficulty {
        DifficultyLevel::Easy => {
            let block_number = get_random_u32();
            debug!("random block_number:{:?}", block_number);
            block_number % max_pebbles_per_turn + 1
        },
        DifficultyLevel::Hard => {
            // I don't kown the hard level
            let block_number = get_random_u32();
            debug!("random block_number:{:?}", block_number);
            block_number % max_pebbles_per_turn + 1
        },
    }
}

#[no_mangle]
extern "C" fn init(){
    let PebblesInit {
        difficulty,
        mut pebbles_count,
        max_pebbles_per_turn,
    }= msg::load().expect("Failed to load init data");

    debug!("pebbles_count: {pebbles_count}, max_pebbles_per_turn: {max_pebbles_per_turn}");

    if pebbles_count == 0 || max_pebbles_per_turn == 0 || max_pebbles_per_turn > pebbles_count {
        panic!(
            "Please check pebbles_count == 0 || max_pebbles_per_turn == 0 || max_pebbles_per_turn > pebbles_count"
        );
    }

    let block_number = get_random_u32();
    debug!("random block_number:{:?}", block_number);
    let first_player = if block_number % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    if first_player == Player::Program {
        let action = determine_program_action(pebbles_count, max_pebbles_per_turn, difficulty.clone());
        pebbles_count -= action;
    }

    // update PEBBLES_GAME
    unsafe {
        PEBBLES_GAME = Some(GameState {
            pebbles_count,
            max_pebbles_per_turn,
            pebbles_remaining: pebbles_count,
            difficulty,
            first_player,
            winner: None,
        });
    }
    
}

#[no_mangle]
extern "C" fn handle(){
    let pebbles_action:PebblesAction = msg::load().expect("Failed to load pebbles_action.");
    let game_state = unsafe{ PEBBLES_GAME.as_mut().expect("State isn's initialized.")};

    let result:PebblesEvent = match pebbles_action{
        PebblesAction::Turn(pebbles) => {
            if pebbles <=0 || pebbles > game_state.max_pebbles_per_turn || pebbles > game_state.pebbles_remaining {
                return ();
            } else {
                // do user's turn
                game_state.pebbles_remaining -= pebbles;
                debug!("after USER opt:{:?}", game_state);
                if game_state.pebbles_remaining <= 0 {
                    let user = Player::User;
                    game_state.winner = Some(user.clone());
                    debug!("USER won:{:?}", game_state);
                    PebblesEvent::Won(user)
                } else {
                    // do program's turn 
                    let program_turn = determine_program_action(game_state.pebbles_remaining, game_state.max_pebbles_per_turn, game_state.difficulty.clone());
                    game_state.pebbles_remaining -= program_turn;
                    debug!("after PROGRAM opt:{:?}", game_state);
                    if game_state.pebbles_remaining <= 0 {
                        let program = Player::Program;
                        game_state.winner = Some(program.clone());
                        debug!("PROGRAM won:{:?}", game_state);
                        PebblesEvent::Won(program)
                    }
                    else{
                        PebblesEvent::CounterTurn(program_turn)
                    }
                }
            }
        },
        PebblesAction::GiveUp => {
            // do program's turn 
            let program_turn = determine_program_action(game_state.pebbles_remaining, game_state.max_pebbles_per_turn, game_state.difficulty.clone());
            game_state.pebbles_remaining -= program_turn;
            debug!("USER giveup, after PROGRAM opt:{:?}", game_state);
            if game_state.pebbles_remaining <= 0 {
                let program = Player::Program;
                game_state.winner = Some(program.clone());
                debug!("USER giveup, after PROGRAM won:{:?}", game_state);
                PebblesEvent::Won(program)
            } else{
                PebblesEvent::CounterTurn(program_turn)
            }
        },
        PebblesAction::Restart {difficulty, mut pebbles_count, max_pebbles_per_turn} => {
            
            if pebbles_count == 0 || max_pebbles_per_turn == 0 || max_pebbles_per_turn > pebbles_count {
                panic!(
                    "Please check pebbles_count == 0 || max_pebbles_per_turn == 0 || max_pebbles_per_turn > pebbles_count"
                );
            }
        
            let block_number = get_random_u32();
            debug!("random block_number:{:?}", block_number);
            let first_player = if block_number % 2 == 0 {
                Player::User
            } else {
                Player::Program
            };

            let result = if first_player == Player::Program {
                let action = determine_program_action(pebbles_count, max_pebbles_per_turn, difficulty.clone());
                pebbles_count -= action;
                
                if pebbles_count <= 0 {
                    PebblesEvent::Won(first_player.clone())
                } else {
                    PebblesEvent::CounterTurn(action)
                }
            } else {
                PebblesEvent::CounterTurn(0)
            };
        
            // update PEBBLES_GAME
            unsafe {
                PEBBLES_GAME = Some(GameState {
                    pebbles_count,
                    max_pebbles_per_turn,
                    pebbles_remaining: pebbles_count,
                    difficulty,
                    first_player: first_player,
                    winner: None,
                });
            };  

            result
        },
    };

    msg::reply(result, 0).expect("Failed to reply handle.");
}

#[no_mangle]
extern "C" fn state(){
    let game_state = unsafe { PEBBLES_GAME.as_ref() }.unwrap();
    debug!("game_state:{:?}",game_state);
    msg::reply(game_state.clone(), 0).expect("error state");
}
