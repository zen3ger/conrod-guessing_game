#[macro_use]
extern crate conrod;
extern crate find_folder;

use conrod::backend::piston::{self,Window,WindowEvents,OpenGL};
use conrod::backend::piston::event::UpdateEvent;

mod app_data;
use app_data::GuessingGame;

fn main() {
    let mut game = GuessingGame::new();

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 300;

    let opengl = OpenGL::V3_2;

    let mut window: Window = piston::window::WindowSettings::new("Guessing Game",[WIDTH,HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut events = WindowEvents::new();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64,HEIGHT as f64]).build();    
    let mut ids = Ids::new(ui.widget_id_generator());

    // már lesz caching
    let mut text_texture_cache = piston::window::GlyphCache::new(&mut window,WIDTH,HEIGHT);
    
    let image_map = conrod::image::Map::new();

    //a font kell ahhoz, hogy megjeleníthető legyen a szöveg!
    let assets = find_folder::Search::KidsThenParents(3,5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/UbuntuMono-R.ttf");
    ui.fonts.insert_from_file(font_path).unwrap(); 

    while let Some(event) = window.next_event(&mut events) {
        if let Some(e) = piston::window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| set_ui(ui.set_widgets(),&ids, &mut game));

        window.draw_2d(&event, |c,g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img:&T) -> &T { img };
                piston::window::draw(c,g, primitives,
                                     &mut text_texture_cache,
                                     &image_map,
                                     texture_from_image);
            }
        });
    }
}

widget_ids! {
    struct Ids {
        canvas,
        button,
        textbox,
    }
}

fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids, game: &mut GuessingGame) {
    use conrod::color::Color;
    use conrod::{Colorable, Labelable, Positionable, Sizeable};
    use conrod::Widget;
    use conrod::widget::text_box;    
    use conrod::widget::{Canvas, Button, TextBox};
    

    let (color,button_label) = if !game.game_lost() {
        (Color::Rgba(1.,1.,1.,1.), "GUESS!")
    } else {
        (Color::Rgba(0.1,0.1,0.1,1.0), "TRY AGAIN!")
    };

    let canvas_title = if game.game_won() { "YOU GUESSED IT!" } else { "GUESS NUMBER BETWEEN 0-100!" };

    Canvas::new()
        .title_bar(canvas_title)
        .color(color)
        .pad(40.0)
        .set(ids.canvas, ui);
    if !game.game_won() {
        for _click in Button::new()
            .mid_bottom_of(ids.canvas)
            .w_h(200.0,100.0)
            .label(button_label)
            .label_color(conrod::color::BLACK)
            .color(conrod::color::ORANGE)
            .set(ids.button,ui) 
            {
                if game.game_lost() {
                    game.restart_game();
                } else {
                    game.new_guess();
                }
                //println!("{:?}",game);
            }

        if !game.game_lost() {
            let content = &(game.guess());
            for edit in TextBox::new(content)
                .align_text_middle()
                .up_from(ids.button,15.0)
                .w_h(200.0,100.0)
                .set(ids.textbox, ui)
                {
                    match edit {
                        text_box::Event::Enter => game.new_guess(),
                        text_box::Event::Update(text) => game.update_guess(text),
                    }
                }

        }
    }
}