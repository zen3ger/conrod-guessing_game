#[macro_use]
extern crate conrod_core;
extern crate conrod_piston;
extern crate find_folder;
extern crate piston_window;
extern crate rand;

use self::piston_window::{
    texture::UpdateTexture, G2d, G2dTexture, OpenGL, PistonWindow, TextureSettings,
    UpdateEvent, Window, WindowSettings,
};

mod app;
mod event;
mod logic;

fn main() {
    // Necessary imports for the window, and the even-loop
    let mut data = app::AppData::new(450, 350, "Guessing Game");
    let mut game = app::GameData::new(10, [1, 50]);

    // Initialization of the ui, widget ids, and our font
    let font_path = app::load_font("UbuntuMono-R.ttf");
    let mut ui = conrod_core::UiBuilder::new([data.width as f64, data.height as f64]).build();
    ui.fonts.insert_from_file(font_path).unwrap();
    let ids = app::Ids::new(ui.widget_id_generator());

    // Initialization of the window
    let mut window: PistonWindow =
        WindowSettings::new(data.title.clone(), [data.width, data.height])
            .opengl(OpenGL::V3_2)
            .samples(4)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

    // I have no idea what this does but it is necessary apparently
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(data.width, data.height)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = data.width as usize * data.height as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture =
            G2dTexture::from_memory_alpha(factory, &init, data.width, data.height, &settings)
                .unwrap();
        (cache, texture)
    };

    let image_map = conrod_core::image::Map::new();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_vertex_data = Vec::new();

    // Event loop - draw loop
    while let Some(event) = window.next() {
        // Handle window resizing
        let size = window.size();

        let (win_w, win_h) = (
            size.width as conrod_core::Scalar,
            size.height as conrod_core::Scalar,
        );

        // Let our UI handle events
        if let Some(e) = event::convert(event.clone(), win_w, win_h) {
            ui.handle_event(e);
        }

        // update our UI state
        event.update(|_| logic::update(ui.set_widgets(), &ids, &mut game, &mut data));

        // draw our UI
        window.draw_2d(&event, |context, graphics| {
            if let Some(primitives) = ui.draw_if_changed() {
                // A function used for caching glyphs to the texture cache.
                let cache_queued_glyphs = |graphics: &mut G2d,
                                           cache: &mut G2dTexture,
                                           rect: conrod_core::text::rt::Rect<u32>,
                                           data: &[u8]| {
                    let offset = [rect.min.x, rect.min.y];
                    let size = [rect.width(), rect.height()];
                    let format = piston_window::texture::Format::Rgba8;
                    let encoder = &mut graphics.encoder;
                    text_vertex_data.clear();
                    text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                    UpdateTexture::update(
                        cache,
                        encoder,
                        format,
                        &text_vertex_data[..],
                        offset,
                        size,
                    )
                    .expect("failed to update texture")
                };

                // Specify how to get the drawable texture from the image. In this case, the image
                // *is* the texture.
                fn texture_from_image<T>(img: &T) -> &T {
                    img
                }

                // Draw the conrod `render::Primitives`.
                conrod_piston::draw::primitives(
                    primitives,
                    context,
                    graphics,
                    &mut text_texture_cache,
                    &mut glyph_cache,
                    &image_map,
                    cache_queued_glyphs,
                    texture_from_image,
                );
            }
        });
    }
}
