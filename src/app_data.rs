extern crate rand;
use self::rand::Rng;

#[derive(Debug)]
pub struct GuessingGame {
    secret_num:i32,
    no_guess:i32,
    win:bool,

    guess_txt:String,
}

impl GuessingGame {
    pub fn new() -> GuessingGame {
        let rnd = rand::thread_rng().gen_range(0,101);
        GuessingGame {
            secret_num:rnd,
            no_guess:10,
            win:false,

            guess_txt:String::new(),
        }
    }
    pub fn new_guess(&mut self) {
        let num: i32 = self.guess_txt.parse().unwrap_or_else(|_| -1);

        if num == self.secret_num { self.win = true; }
        else { self.no_guess -= 1; }

        self.guess_txt = String::default();
    }
    pub fn game_won(&self)->bool {
        self.win
    }
    pub fn restart_game(&mut self) {
        self.secret_num = rand::thread_rng().gen_range(0,101);
        self.no_guess = 10;
        self.win = false;
    }
    pub fn guess(&self) -> String {
        self.guess_txt.clone()
    }
    pub fn update_guess(&mut self, guess:String) {
        self.guess_txt = guess;
    }
    pub fn game_lost(&self) -> bool {
        self.no_guess == 0 && !self.win
    }
}