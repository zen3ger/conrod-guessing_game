/// This module contains code, that is not
/// required to be bound to the application logic.

/// Macro used to generate the ids
use std::path::PathBuf;

widget_ids! {
    pub struct Ids {
        canvas, // Our main UI container
        guess_button,
        count_text,
        info_text,
        textbox,
        slider,
        newgame
    }
}

pub struct AppData {
    pub width: u32,
    pub height: u32,
    pub guess: String,
    pub title: String,
    pub info: String,
}

impl AppData {
    pub fn new(width: u32, height: u32, title: &str) -> AppData {
        AppData {
            width: width,
            height: height,
            guess: String::new(),
            title: title.to_owned(),
            info: "? X".to_owned(),
        }
    }

    pub fn new_guess(&mut self, guess: &str) {
        self.guess = guess.to_owned();
    }
}

pub struct GameData {
    secret_num: i32,
    no_guess: i32,
    pub range_min: i32,
    pub range_max: i32,
    win: bool,
}

impl GameData {
    pub fn new(guess_num: i32, range: [i32; 2]) -> Self {
        use super::rand::Rng;
        // in the fn signature I use [i32;2] because for me [num0,num1] resembels a closed interval, like in math
        // change to (guess_num, min, max) format if you'd like to
        let (min, max) = (range[0], range[1]);
        GameData {
            secret_num: rand::thread_rng().gen_range(min, max + 1),
            no_guess: guess_num,
            range_min: min,
            range_max: max,
            win: false,
        }
    }
    // to make it impossible to overwrite it outside of the rules
    pub fn get_no_guess(&self) -> i32 {
        self.no_guess
    }
    // add get() and set() for range bounds if you want to, I don't really...

    // defines how you can modify the no_guess and the rest of the internals
    pub fn new_guess(&mut self, guess: &str) -> String {
        if guess != "" {
            let g: i32 = guess
                .parse()
                .expect("`guess` cannot be converted to number");
            if g == self.secret_num {
                self.win = true;
                return "= X".to_owned();
            } else {
                self.no_guess -= 1;
                return if g < self.secret_num {
                    "< X".to_owned()
                } else {
                    "> X".to_owned()
                };
            }
        }
        "? X".to_owned()
    }
    // easiest way to concat converted values and do the formatting
    pub fn show_range(&self) -> String {
        format!("({}, {})", self.range_min, self.range_max)
    }

    pub fn end(&self) -> bool {
        self.win || self.no_guess == 0
    }

    pub fn restart(&mut self) {
        use super::rand::Rng;

        self.secret_num = rand::thread_rng().gen_range(self.range_min, self.range_max + 1);
        self.win = false;
        self.no_guess = 10;
    }
}

pub fn load_font(font: &str) -> PathBuf {
    use super::find_folder::Search::KidsThenParents;

    let fonts_dir = KidsThenParents(3, 5)
        .for_folder("fonts")
        .expect("`fonts/` not found!");
    let font_path = fonts_dir.join(font);

    font_path
}
