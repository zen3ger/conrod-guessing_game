#[macro_use]
extern crate conrod;
extern crate find_folder;
extern crate rand;

mod app;
mod logic;

use app::Ids;

fn main() {
    /// Necessary imports for the window, and the even-loop
    use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
    use conrod::backend::piston::event::UpdateEvent;

    /// Setups
    let mut data = app::AppData::new(450, 350, "Guessing Game");

    let mut game = app::GameData::new(10, [1, 50]);

    let font_path = app::load_font("UbuntuMono-R.ttf");

    /// Initialization of the window
    let mut window: Window = piston::window::WindowSettings::new(data.title.clone(),
                                                                 [data.width, data.height])
        .opengl(OpenGL::V3_2)
        .samples(4)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let mut events = WindowEvents::new();

    /// Initialization of caches
    let mut text_texture_cache =
        piston::window::GlyphCache::new(&mut window, data.width, data.height);
    let image_map = conrod::image::Map::new();

    /// Initialization of the ui, and the widget ids
    let mut ui = conrod::UiBuilder::new([data.width as f64, data.height as f64]).build();
    ui.fonts.insert_from_file(font_path).unwrap();
    let ids = Ids::new(ui.widget_id_generator());


    /// Event loop - draw loop
    while let Some(event) = window.next_event(&mut events) {
        // Convert piston events to conrod if there's any, ie.: window resize
        if let Some(e) = piston::window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        /// Update the ui
        event.update(|_| logic::update(ui.set_widgets(), &ids, &mut game, &mut data));

        /// Draw the updated ui
        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T {
                    img
                };
                piston::window::draw(c,
                                     g,
                                     primitives,
                                     &mut text_texture_cache,
                                     &image_map,
                                     texture_from_image);
            }
        });
    }

}