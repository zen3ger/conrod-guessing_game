use super::app::{AppData, GameData, Ids};
use super::conrod_core;

pub fn update(ref mut ui: conrod_core::UiCell, ids: &Ids, game: &mut GameData, data: &mut AppData) {
    use conrod_core::Widget;
    use conrod_core::widget::text_box;
    use conrod_core::widget::range_slider;
    use conrod_core::widget::{Button, Canvas, RangeSlider, Text, TextBox};
    use conrod_core::{color, Colorable, Labelable, Positionable, Sizeable};

    let caption = format!("Guess number between {}", game.show_range());

    Canvas::new()
        .color(color::WHITE)
        .title_bar(&caption)
        .pad(40.0)
        .set(ids.canvas, ui);

    if !game.end() {
        for _click in Button::new()
            .top_left_with_margin_on(ids.canvas, 0.0)
            .label("Guess!")
            .w_h(100.0, 40.0)
            .color(color::WHITE)
            .press_color(color::RED)
            .set(ids.guess_button, ui)
        {
            data.info = game.new_guess(&data.guess);
        }

        Text::new(&(game.get_no_guess().to_string()))
            .middle_of(ids.guess_button)
            .down_from(ids.guess_button, 10.0)
            .set(ids.count_text, ui);

        for edit in TextBox::new(&data.guess)
            .right_from(ids.guess_button, 10.0)
            .w_h(200.0, 50.0)
            .set(ids.textbox, ui)
        {
            match edit {
                text_box::Event::Enter => {
                    data.info = game.new_guess(&data.guess);
                }
                text_box::Event::Update(text) => {
                    data.new_guess(&text);
                }
            }
        }

        Text::new(&data.info)
            .middle_of(ids.textbox)
            .right_from(ids.textbox, 10.0)
            .font_size(25)
            .set(ids.info_text, ui);
    } else {
        for (edge, value) in
            RangeSlider::new(game.range_min as f32, game.range_max as f32, -500.0, 500.0)
                .padded_w_of(ids.canvas, 100.0)
                .h(50.0)
                .mid_bottom_with_margin_on(ids.canvas, 10.0)
                .set(ids.slider, ui)
        {
            match edge {
                range_slider::Edge::Start => game.range_min = value as i32,
                range_slider::Edge::End => game.range_max = value as i32,
            }
        }
        for _click in Button::new()
            .middle_of(ids.canvas)
            .up_from(ids.slider, 40.0)
            .w_h(150.0, 50.0)
            .label_font_size(20)
            .label("New game")
            .set(ids.newgame, ui)
        {
            game.restart();
        }
    }
}
