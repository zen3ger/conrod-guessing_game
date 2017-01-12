extern crate rand;
use self::rand::Rng;

#[derive(Debug)]
pub struct GuessingGame {
    secret_num:i32,
    no_guess:i32,
    win:bool,
}

impl GuessingGame {
    pub fn new() -> GuessingGame {
        let rnd = rand::thread_rng().gen_range(0,101);
        GuessingGame {
            secret_num:rnd,
            no_guess:10,
            win:false,
        }
    }
    pub fn guess(&mut self, num:i32) {
        if num == self.secret_num { self.win = true; }
        else { self.no_guess -= 1; }
    }
    pub fn game_won(&self)->bool {
        self.win
    }
    pub fn new_game(&mut self) {
        self.secret_num = rand::thread_rng().gen_range(0,101);
        self.no_guess = 10;
        self.win = false;
    }
    pub fn game_lost(&self) -> bool {
        self.no_guess == 0
    }
}