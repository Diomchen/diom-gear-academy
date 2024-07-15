#[cfg(test)]
mod tests {
    use gtest::{Program, System};
    use pebbles_game_io::*;

    const USERS: &[u64] = &[3, 4, 5];

#[test]
fn test_init() {
    // let path = "../target/wasm32-unknown-unknown/debug/pebbles_game.opt.wasm";

    let sys = System::new();
    let prog = Program::current_opt(&sys);
    sys.init_logger();

    // SUCCESS
    let success_init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 2,
    };
    let res = prog.send(USERS[0], success_init_data);
    assert!(!res.main_failed());
}

#[test]
fn test_failed_1(){
    
    let sys = System::new();
    let prog = Program::current_opt(&sys);
    sys.init_logger();

    // FAILED: max_pebbles_per_turn > pebbles_count
    let failed_init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 17,
    };
    let res_1 = prog.send(USERS[1], failed_init_data);
    assert!(res_1.main_failed());
}

#[test]
fn test_failed_2(){
    
    let sys = System::new();
    let prog = Program::current_opt(&sys);
    sys.init_logger();

    // FAILED: max_pebbles_per_turn == pebbles_count == 0
    let failed_init_data_1 = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 0,
        max_pebbles_per_turn: 0,
    };
    let res_2 = prog.send(USERS[2], failed_init_data_1);
    assert!(res_2.main_failed());
}

#[test]
fn test_user_easy_win(){
    // get random number 0

    let sys = System::new();
    let prog = Program::current_opt(&sys);
    sys.init_logger();

    // SUCCESS
    let success_init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 7,
    };
    let res = prog.send(USERS[0], success_init_data);
    assert!(!res.main_failed());

    // user turn 7
    prog.send(USERS[0], PebblesAction::Turn(7));

    // user turn 7
    prog.send(USERS[0], PebblesAction::Turn(7));

    let state:GameState = prog.read_state(()).expect("error");
    assert_eq!(state.winner, Some(Player::User));
}

#[test]
fn test_program_hard_win(){
    // get random number 0

    let sys = System::new();
    let prog = Program::current_opt(&sys);
    sys.init_logger();

    // SUCCESS
    let success_init_data = PebblesInit {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 14,
    };
    let res = prog.send(USERS[0], success_init_data);
    assert!(!res.main_failed());
    
    // user turn 7
    prog.send(USERS[0], PebblesAction::Turn(14));

    let state:GameState = prog.read_state(()).expect("error");
    assert_eq!(state.winner, Some(Player::Program)); 
}

#[test]
fn test_giveup(){
    // get random number 0

    let sys = System::new();
    let prog = Program::current_opt(&sys);
    sys.init_logger();

    // SUCCESS
    let success_init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 7,
    };
    let res = prog.send(USERS[0], success_init_data);
    assert!(!res.main_failed());

    // user turn 7
    prog.send(USERS[0], PebblesAction::Turn(7));

    // user give up
    prog.send(USERS[0], PebblesAction::GiveUp);

    // user turn 5
    prog.send(USERS[0], PebblesAction::Turn(5));

    let state:GameState = prog.read_state(()).expect("error");
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_restart(){
        // get random number 0

        let sys = System::new();
        let prog = Program::current_opt(&sys);
        sys.init_logger();
    
        // SUCCESS
        let success_init_data = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 7,
        };
        let res = prog.send(USERS[0], success_init_data);
        assert!(!res.main_failed());
    
        // user turn 7
        prog.send(USERS[0], PebblesAction::Turn(7));
    
        // user give up
        let restart = PebblesAction::Restart{
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 7,
        };
        prog.send(USERS[0], restart);
    
        // user turn 7
        prog.send(USERS[0], PebblesAction::Turn(7));
    
        let state:GameState = prog.read_state(()).expect("error");
        assert_eq!(state.pebbles_remaining, 7);
}

#[test]
fn test_get_state(){
    let sys = System::new();
    let prog = Program::current_opt(&sys);
    sys.init_logger();

    // SUCCESS
    let success_init_data = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 7,
    };
    let res = prog.send(USERS[0], success_init_data);
    assert!(!res.main_failed());

    // let state_query = GameState{};
    let game_state = GameState {
        pebbles_count: 15,
        max_pebbles_per_turn: 7,
        pebbles_remaining: 15,
        difficulty: DifficultyLevel::Easy,
        first_player: Player::User,
        winner: None,
    };
    let state:GameState = prog.read_state(()).expect("error");
    assert_eq!(state.pebbles_count, game_state.pebbles_count);
    assert_eq!(state.max_pebbles_per_turn, game_state.max_pebbles_per_turn);
    assert_eq!(state.pebbles_remaining, game_state.pebbles_remaining);
    assert_eq!(state.difficulty, game_state.difficulty);
    assert_eq!(state.first_player, game_state.first_player);
    assert_eq!(state.winner, game_state.winner);
}

}