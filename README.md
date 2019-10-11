## Using Conrod - Let's build something simple!

We're about to make a simple application step by step. The example app will be a graphical version of the Guessing Game. I'm pretty sure you came across this one in the Rust Book. This time, we're going to make it look 'pretty'!
![Guessing Game UI](/illustration/app_ui.png) 

But before we move on, let me tell you how I'm going to structure the application based on how `conrod` and IMGUIs work.

### How to think IMGUI
In an IMGUI library widgets do not hold state, they are recreated with always the actual data provided at each update cycle. It means you don't have to manually update the content of the widgets. Take a counter example in Visual C#:

```cs
private void ButtonInc_Click(object sender, EventArgs e)
{
	count += 1;
    LabelCount.Text = count.ToString();
}
```

First, the update is tied to the `callback` of the `ButtonInc` object and we also have to tell the `LabelCount` object to update its `Text` field. At each point in the code you change `count` you also have to update the label. Ok, you can encapsulat these changes into one single function to call it at each point `count` changes, like:

```cs
private void UpdateCount(int amount)
{
	count += amount;
    LabelCount.Text = count.ToString();
}
```

The point is, you store both `count` and `LabelCount.Text` while they represent the same thing.
In `conrod` the code would look like this:

```rust
let mut count = 0;
//... stuff here
for click in Button::new()
	.options_come_here() //all widget settings will be defined here, like size, color...
    {
    	count += 1;
    }

Text::new(&count.to_string())
	.options_come_here();
```

An IMGUI is a bit more descriptive, if you read to code what it says at each click increment the counter and show me it as text. Let's say you have a dozen buttons. Each adds, subtracts or multiplies `count`, you still only have to do the `count.to_string()` once and only once, when you redraw the `Text` in comparison to always calling `UpdateCount()` in the  C# example.

The point of the above is that the **content** you see on screen is driven by the **logic** of the program, while the logic is driven by the application **data**! The separation in the application that I'm going to use will be the same. There will be a `main.rs` with all the setup and initialisation for showing the **content**. There will also be a `logic.rs` and an `app.rs` with the application data, and some helper functions which doesn't necessarily belongs to the other 2 categories. Finally, we'll add an `event.rs` as single point of managing events.

So, remember:
![IMGUI content flow](/illustration/imgui_content_flow.png)

**Note 1:** in C# there's also a way to use the same function for multpile callbacks and then check who's the sender and perform the appropiate computations and call `UpdateCount()` only once.

**Note 2:** IMGUIs are not perfect for everything, it's great for displaying live data, and having a reactive UI, ie. in game UI or for displaying sensor data, soundwave forms.

### 0. Setting up the project:
Type `cargo new --bin guessing_game` into your terminal to get the skeleton of the project. First we need to add the dependencies for our application, so open up the `Cargo.toml` file in the freshly created directory and lets add a few lines to it!

```toml
[dependencies]
piston_window = "0.89"
conrod_core = "0.67.0"
conrod_piston = "0.67.0"
find_folder="*"
rand="*"
```

We will be using:
- `piston_window` to manage the window within which our application will exist.
- `conrod_core` to manage our UI's widgets, and associated events.
- `conrod_piston` as a bridge between `piston` and `conrod`
- `find_folder` will help us manage assetss for our UI
- `rand` will be used to generate the 'secret' number.  With a constant number it would be a silly game, don't you think?

For this guide, the `piston` and `conrod` dependencies are locked at their current version at the time of writing. These may become out of date within the lifetime of this guide (at which point, you are encouraged to submit an update). 

As it can take a while to download and compile for the first time, let's do the heavy lifting before we start coding, so type `cargo run` into your terminal to fetch and compile all necessary packages. If everything is fine, you'll see `Hello, world!` printed out on in your terminal.

All cool? Great! Let's move on!

### 1. Basic window
In this section we are about to write all the code needed to get an empty window show up.

#### Imports:
Open up your `main.rs` file and edit to make it look like this:
```rust
#[macro_use]
extern crate piston_window;
extern crate conrod_core;
extern crate conrod_piston;
extern crate find_folder;
extern crate rand;

fn main() {

    /*
    	...
    */
}
```

For now, that's all we need to import. The rest is going to be inside functions and modules.

You see that `#[macro_use]` up top, right? `conrod` has a macro wich is used always, `widget_ids!`. Wait until we start working with widgets, I'll tell you more.

#### Definitions:
The 3 basic things we will need, is a title, the width and the height of the window.
```rust
fn main() {
	let width: u32 = 450;
    let height: u32 = 350;
    let title = "Guessing Game";

    // ...
}
```

To get a nice window we are going to use something called `method chaining` or `builder pattern`. Think about it like ordering some food! You first tell the cashier you want pizza, than you say "Yo, I love spicy food, add some pepperoni, and coke too".

```rust
let my_dinner = Restaurant::Order::new()
	.food("pizza")
	.with_extra("pepperoni")
	.drink("coke")
	.done();
```

In our case, rather than ordering pizza from a restaurant, we want to order a window from `piston`.

Let's tell `piston` which parameters we want our new window to have.

```rust
let mut window: PistonWindow =
        WindowSettings::new(title, [width, height])
            .opengl(OpenGL::V3_2)
            .samples(4)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();
```

Let's go through what we specified here:
* `WindowSettings::new()` returns with a 'builder' for the `Window` type. It contains a bunch of defaults, which we can modify by adding additional functions
* `.opengl()` sets the version of OpenGL the backend should use for the renderig. You can choose from a predefined set of versions. To check out from what you can choose from, click [here](http://docs.piston.rs/conrod/conrod/backend/piston/enum.OpenGL.html).
* `exit_on_esc()` tells the window to listen for the `esc` key and close.
* `vsync()` can help you get rid of screen tearing during resize events.
* `build()` is the function which executes the 'order' and returns with either a `Window` or some error message.

You can take a look at all the other options that can be set [here](http://docs.piston.rs/conrod/conrod/backend/piston/window/struct.WindowSettings.html).

We also need to get the events from the window, for that we have to create an event loop iterator.

```rust
while let Some(event) = window.next() {
    // Handle events...
}
```

#### The event loop:

The event loop we defined above basically states "while there is any event, I'll keep doing `{ // Handle events... }`", meaning this loop runs while the window is open.
We are going to use this loop to draw, update and do pretty much everything we need. But don't be confused! It doesn't mean the entire program will be an if-else spagetti, remember the separation I talked at the beginning. :)

If you do `cargo run` a black window should pop up with the given title and dimensions. Congrats! :)

### 2. Application and game data
In this section our goal is to define what kind of data are we going to track. This is something you may not have to do in traditional, retained mode GUI systems as widgets store many of that data. But remember `conrod` is an IMGUI library of which the most mentionable is that widgets do not store state!

For instance `TextBox` doesn't store the string it displays, so it basically writes into the a variable, and reads back from a variable at each update. The window data is stored, so you don't have to pass `window`, or it's size and title to the draw function. Writing a function to accept a struct is still much easier than adding more and more parameters and then fighting with the borrow checker, I think.

Taking care of widget state is sometimes a problem, but most of the time is a great way of specifing no more than what we need!

So let's create an `app.rs` file and move the bits of **data** we defined earlier (`width`,`height`, etc...) from `main` (it's not necessary for the **content** to be displayed). In `app` we have:

```rust
pub struct AppData {
    // fields are public, so less `fn` to implement
    // for control over data manipulation feel free to extend the `impl` block

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

```
If you remember our initial UI design, it had a `Button`, `TextBox` for entering the guess and a `Text` for information about the previous guess.

In our data `info` is used for the `Text`, and `guess` used for the `TextBox`s content. There was also one more `Text` for showing the guesses left, the data for that is going to be in an other struct, as it has more to do with the rules of the game.

Now we define all the data for the game, and the behaviour aka. rules of the game.
```rust
pub struct GameData {
    secret_num: i32,
    no_guess: i32,
    pub range_min: i32,
    pub range_max: i32,
    win: bool,
}

impl GameData {
    pub fn new(guess_num: i32, range: [i32; 2]) -> Self {
        use super::rand::{self, Rng};
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

	// defines how you can modify the no_guess and the rest of the internals
    pub fn new_guess(&mut self, guess: &str) -> String {
        if guess != "" {
            let g: i32 = guess.parse().expect("`guess` cannot be converted to number");
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
        use super::rand::{self, Rng};

        self.secret_num = rand::thread_rng().gen_range(self.range_min, self.range_max + 1);
        self.win = false;
        self.no_guess = 10;
    }
}
```

We will need to make additional modifications in `main.rs` to match our new structure:

```rust
/* externs */

mod app;

fn main() {
    /* imports */

    // Necessary imports for the window, and the even-loop
    let mut data = app::AppData::new(450, 350, "Guessing Game");
    let mut game = app::GameData::new(10, [1, 50]);

    // Initialization of the window
    let mut window: PistonWindow =
        WindowSettings::new(data.title.clone(), [data.width, data.height])
            .opengl(OpenGL::V3_2)
            .samples(4)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

    // event loop...
}
```

### 3. UI setups
The next step is going to be creating the `Ui`. This is special as `Ui` in `conrod` is the data structure that keeps track of the state of every `Widget`, the `Theme` you use globally (this is for in app themeing, does not support OS theme at the moment) and many more. In generall, `Ui` handles draw events, like highlighting a hovered button. It also tries to reduce the number of draw calls, meaning it updates the screen if and only if there's an update which demands a redraw. (Of course, you can demand continuous redraws!)

For more information, see [here](http://docs.piston.rs/conrod/conrod/struct.Ui.html).

#### UiBuilder:
`UiBuilder` let's you specify the ui's dimensions, theme (if none than the default is used) and widget capacity.
As the `Ui` tracks widgets in a graph, it can grow organically or you can help it predefineing how many widgets you're about to instantiate, this could help app launch times, so you're not expand the graph dynamically in the very first draw cycle.

Right now, lets stick with the dimensions only, so you can experiment with adding more to what I've done.

```rust

// add this to main()
let mut ui = conrod_core::UiBuilder::new([data.width as f64, data.height as f64]).build();

```

#### Widget Ids:
As the widget states are store inside the graph, we use `Ids` to acces specific widgets. `conrod` provides a macro for creating an `Ids` struct with the given fields. In my opinion `Ids` are part of the **data**, so place the macro in `app.rs`.

```rust
widget_ids! {
    pub struct Ids {
        canvas,
        guess_button,
        count_text,
        info_text,
        textbox,
    }
}

// ...AppData and GameData
```
So, the macro will create the `Ids` struct for us, with fields with the name representing a unique widget id. These `Ids` needs a way of knowing, if they're exist in the `ui`. If you add more widgets, don't forget to extend `Ids`.

Let's update the `main()` fn in `main.rs` to reference our widget id's:

```rust
let ids = app::Ids::new(ui.widget_id_generator());
```

I think the names are descriptive enough. The only thing to mention is that `ids.canvas` is the id of `Canvas`, a widget that can hold other widgets and usually fills the entire window.

### 4. Content rendering
The next step is to finally add something to our draw loop!

Note that this will not comile yet, as we have not yet implemented our `logic::update()` function.

```rust
while let Some(event) = window.next_event(&mut events) {
    // update our game state
    event.update(|_| logic::update(ui.set_widgets(), &ids, &mut game, &mut data));
}
```

Let's create the `logic::update()` just to see something on screen. This function is going to be the **logic**. You know what to do! Create `logic.rs` and place this snippet:

```rust
use app::{GameData, AppData, Ids};
use super::conrod;

pub fn update(ref mut ui: conrod::UiCell, ids: &Ids, game: &mut GameData, data: &mut AppData) {
    use conrod::{Colorable, Labelable, Positionable, Sizeable};
    use conrod::Widget;
    use conrod::widget::text_box;
    use conrod::widget::{Canvas, Button, Text, TextBox};

    let mut caption = "Guess number between ".to_owned();
    let range = game.show_range();
    caption.push_str(&range);

    Canvas::new()
        .color(conrod::color::WHITE)
        .title_bar(&caption)
        .pad(40.0)
        .set(ids.canvas, ui);
}
```

If you run our game now via `cargo run`, you will see that it's still a black window, well... Every cycle you tell `conrod` to update the `canvas` in the state graph, but so far, we have not rendered the graph aka. the `ui`? 

Let's do it! Update the `main()` function in `main.rs` with the following code: 

```rust
// ...

// Create an image map for conrod
let image_map = conrod_core::image::Map::new();

// Create a texture to use for efficiently caching text on the GPU.
let mut text_vertex_data = Vec::new();

while let Some(event) = window.next_event(&mut events) {
	// update our game state
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
                text_vertex_data.extend(data.iter()         .flat_map(|&b| vec![255, 255, 255, b]));
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
```

That is an epic amount of code. Never fear though, a lot of this code is simply required as parameters to the last method `conrod_piston::draw` to keep things performant. Lets step through and take stock of what we have just added.

The `image_map` and `text_vertex_data` variables we created allow piston to perform optimisation and caching on our UI data. This is an internal feature which we won't dwell on here, but more information is available [here](https://docs.rs/conrod_piston/0.67.0/conrod_piston/draw/fn.primitives.html) if you're the curious type. The same is true for the closure we defined named `cache_queued_glyphs`. Again, we don't need to concern ourselves with this at this point. The same is true of our next internal method `texture_from_image`.

At the end of our new code, beyond the caching calls, we call `conrod_piston::draw::primitives()` which is the call which renders our widget primitives. In the initial call to our `logic::update` method, we are updating the data state, which is read by our widgets when they are updated by the call to `conrod_piston::draw::primitives()`.

If you run our game now, you'll see a window popping up filled with white. So the canvas actually fills out the window, huh great! Let's test resizing the window. Oooops, you quickly realize that it's not working properly. Also, where's the `caption`?

##### Resize:

The canvas has the initial width and height of the window, and it doesn't grow beyound. This is because `conrod` supports multiple backends: we have to tell it what kind of event convertions should be done.

Since the conversion and handling of such events is relatively isolated, let's create one new file to manage events. Create the file `event.rs` and add the following code:

```rust
//! A backend for converting src events to conrod's `Input` type.

use conrod_core::{event, input, Point, Scalar};
use piston_window::GenericEvent;

/// Converts any `GenericEvent` to an `Input` event for conrod.
///
/// The given `width` and `height` must be `Scalar` (DPI agnostic) values.
pub fn convert<E>(event: E, win_w: Scalar, win_h: Scalar) -> Option<event::Input>
where
  E: GenericEvent,
{
  // Translate the coordinates from top-left-origin-with-y-down to centre-origin-with-y-up.
  let translate_coords = |xy: Point| (xy[0] - win_w / 2.0, -(xy[1] - win_h / 2.0));

  if let Some(xy) = event.mouse_cursor_args() {
    let (x, y) = translate_coords(xy);
    return Some(event::Input::Motion(input::Motion::MouseCursor {
      x: x,
      y: y,
    }));
  }

  if let Some(rel_xy) = event.mouse_relative_args() {
    let (rel_x, rel_y) = translate_coords(rel_xy);
    return Some(event::Input::Motion(input::Motion::MouseRelative {
      x: rel_x,
      y: rel_y,
    }));
  }

  if let Some(xy) = event.mouse_scroll_args() {
    // Invert the scrolling of the *y* axis as *y* is up in conrod.
    let (x, y) = (xy[0], -xy[1]);
    return Some(event::Input::Motion(input::Motion::Scroll { x: x, y: y }));
  }

  if let Some(button) = event.press_args() {
    return Some(event::Input::Press(button));
  }

  if let Some(button) = event.release_args() {
    return Some(event::Input::Release(button));
  }

  if let Some(text) = event.text_args() {
    return Some(event::Input::Text(text));
  }

  if let Some(dim) = event.resize_args() {
    return Some(event::Input::Resize(dim[0], dim[1]));
  }

  if let Some(b) = event.focus_args() {
    return Some(event::Input::Focus(b));
  }

  None
}
```

As you can see, we are accepting generic `piston::GenericEvent` events, and returning `conrod_core::event` events, which our UI knows how to handle.

Our newly added update code will also captures the Window resize event. We can now let `piston` handle the resize events which were not being captured when we resied earlier. Add the following to the beginning of your `main()` function in `main.rs`:

```rust
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

    // ...
```

Rebuild your application now and test resizing the application window. you should see the application `redraw` as the event is handled at the end of the mouse drag.

#### Fonts:

So text isn't rendered as there's no font defined in `ui` by default. I would say fonts are also part of **data**, because of that write a helper function to load fonts in `app.rs`:

```rust
pub fn load_font(font: &str) -> PathBuf {
    use super::find_folder::Search::KidsThenParents;

    let fonts_dir = KidsThenParents(3, 5)
        .for_folder("fonts")
        .expect("`fonts/` not found!");
    let font_path = fonts_dir.join(font);

    font_path
}
```

It assumes that you have a the font located in the project directory under the folder `*/fonts/`. You can grab the required font files from the source of this repository if required.

Back to our game code, let's load the font!

```rust
/* data and game inits above, just to organize data in one place */
let font_path = app::load_font("UbuntuMono-R.ttf");

//stuff

let mut ui = conrod::UiBuilder::new([data.width as f64, data.height as f64]).build();
ui.fonts.insert_from_file(font_path).unwrap();
let ids = Ids::new(ui.widget_id_generator());
```

If you run the application now, you'll see the title displays correctly.

### 5. Logic

I already asked you to skip ahead and write this code in `logic.rs`, the only thing left is to walk you through!

```rust
use app::{GameData, AppData, Ids};
use super::conrod;

pub fn update(ref mut ui: conrod::UiCell, ids: &Ids, game: &mut GameData, data: &mut AppData) {
    use conrod::{Colorable,Labelable, Positionable, Sizeable};
    use conrod::Widget;
    use conrod::widget::text_box;
    use conrod::widget::{Canvas, Button, Text, TextBox};

    let caption = format!("Guess number between {}", game.show_range());

    Canvas::new()
        .color(conrod::color::WHITE)
        .title_bar(&caption)
        .pad(40.0)
        .set(ids.canvas, ui);
}
```

Q: **Why do I define the `caption` here?**
A: Because it's the combination of the two states, it does not belongs to any, only to the canvas.

So after all this code, I'm going to start with `use ...`. Let's breakdown each:
* [Colorable](http://docs.piston.rs/conrod/conrod/color/trait.Colorable.html) - let's you use colors on widgets, and defines default colors, like `WHITE` in the canvas definition, also has ways of creating colors from RGB and HSL.
* [Labelable](http://docs.piston.rs/conrod/conrod/trait.Labelable.html) - let's you use functions to place labels on widgets like a `Button`
* [Positionable](http://docs.piston.rs/conrod/conrod/trait.Positionable.html) - let's you use functions to move around widgets, align them to others, etc.
* [Sizeable](http://docs.piston.rs/conrod/conrod/trait.Sizeable.html) - let's you get and set the size of widgets at the given update cycle
* the rest is just imports for default and specific widget behaviours

What you see in the type signature is that we interact with the `ui` through something called the `UiCell`. The `UiCell` restricts you from mutating the `ui` in ways you shouldn't do, and provides methods on how you should. You can dig through all you can do through the `UiCell` [here](http://docs.piston.rs/conrod/conrod/struct.UiCell.html).

Let's look at the `Canvas` for a second! As you can see we're not binding a canvas object to a variable, we simply declare what kind of canvas we want. I say, let's make it white, I want the padding from all edges to be 40, and this is going to live under the name `canvas` with a title defined by `caption`. You can do [many more](http://docs.piston.rs/conrod/conrod/widget/canvas/struct.Canvas.html).

The most important bit is `set()`. It creates the widget, adds it to the `ui` graph with the given id.

You can look up `Positionable` to see all the functions available, and to have a better understanding of what the code below means. I'll give you the basic syntax for each widget used, and then the final logic and one more 'homework' to do.

#### The Guess Button:

```rust
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
```

It reads quite well: 'I have a button at the top left corner of the canvas, with size of 100 by 40. It has a label. For each click this button takes a new guess and based on that updates the info.' 

Before you start screaming, that `button` isn't at the top left, remember we have an edge padding of 40 set for the `canvas`. `_` in `_click` is used to tell the compiler we named the iterator, but not going to use it, and we will get no warnings.

#### Text:

Add some text feedback with the following snippet:

```rust
Text::new(&(game.get_no_guess().to_string()))
    .middle_of(ids.guess_button)
    .down_from(ids.guess_button, 10.0)
    .set(ids.count_text, ui);
```

#### TextBox:

...and a text box for entry of our guess:

```rust
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
```

The complete update logic should look like the following:

```rust
pub fn update(/* ... */) {
    /* imports, caption and canvas */

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
    }
}
```

I'd like you to have some fun with positioning the elements, come up with your own layout.

If you run your game at this point, you should be able to play! You can guess a number and the logic will determine whether you either higher, lower or correct.

However, we need to implement the logic for starting a new game, as well as let our user determine the number range for generating a new random number:

#### More UI

We will add some new UI elements to show these options to our user. Here's how that screen will look:

![Guessing Game UI Updated](/illustration/app_ui_updated.png)

ur game logic is already accounting for these new additions, all we need to do at this point is update our UI code by extending the main conditional `if !game.end()`. Add the following `else` statement to account for our new game state.

```rust

if !game.end() {
    // ... Existing code
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
```

Here, we've introduced two new widgets:

- A `RangeSlider` which will allow the player to choose a number range for generating a new random number for the next game.
- A new `Button` to begin the game.

When running the game now, the logic loop will detect when the game has completed, and present this new UI to the player.

#### The end

We've completed what we set out to do, build something simple with conrod. 

Have fun playing the guessing game. 