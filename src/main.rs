#[macro_use]
extern crate conrod;

use conrod::backend::piston::{self,Window,WindowEvents,OpenGL};

mod app_data;
use app_data::GuessingGame;

//extern crate rand;
//use rand::Rng;
   // let rnd = rand::thread_rng().gen_range(1,101);

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let opengl = OpenGL::V3_2;
    
    let mut window: Window = piston::window::WindowSettings::new("Guessing Game",[WIDTH,HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let mut events = WindowEvents::new();



    while let Some(event) = window.next_event(&mut events) {


    }
}