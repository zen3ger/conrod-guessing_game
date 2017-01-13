#[macro_use]
extern crate conrod;

use conrod::backend::piston::{self,Window,WindowEvents,OpenGL};
use conrod::backend::piston::event::UpdateEvent;

mod app_data;
use app_data::GuessingGame;

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let opengl = OpenGL::V3_2;

    let mut window: Window = piston::window::WindowSettings::new("Guessing Game",[WIDTH,HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let mut events = WindowEvents::new();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64,HEIGHT as f64]).build();    
    let mut ids = Ids::new(ui.widget_id_generator());

    let mut text_texture_cache = piston::window::GlyphCache::new(&mut window,0,0);
    let image_map = conrod::image::Map::new();

    while let Some(event) = window.next_event(&mut events) {
        if let Some(e) = piston::window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| set_ui(ui.set_widgets(),&ids));

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
    }
}

fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids) {
    use conrod::Widget;
    use conrod::Colorable;
    use conrod::widget::{Canvas};

    Canvas::new().color(conrod::color::WHITE).pad(50.0).set(ids.canvas, ui);
}