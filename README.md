##CURRENT CODE DOES NOT RESEMBELS THE TUTORIAL - WORK IN PROGRESS

##Using Conrod - Let's build something simple!

We're about to make a simple application step by step. The example app will be a graphical version of the Guessing Game. I'm pretty sure you came across this one in the Rust Book. This time, we're going to make it look pretty!

###Setting up the project:
Type `cargo new --bin guessing_game` into your terminal to get the skeleton of the project. First we need to add the dependencies for our application, so open up the `Cargo.toml` file in the freshly created directory and lets add a few lines to it!

```toml
[dependencies]
conrod = "*"
rand="*"
```

We need to fetch conrod from `crates.io`, and we're going to do that by adding it to the dependencies. `"*"` marks we want to get the latest version. The same applies for `rand`, it'll be used to generate the 'secret' number, with constant number it whould be a silly game, don't you think?

As it can take a while to download, let's do the heavy lifting before we start coding, so type `cargo run` into your terminal to fetch all necessary packages. If everything is fine, you'll see `Hello, world!` printed out on the last line.

All cool? Great! Let's move on!

###Basic window
In this section we are about to write all the code needed to get an empty window show up. For now we'll use `piston_window` as the backend.

**Imports:**
Open up your `main.rs` file and add these lines to the top:
```rust
#[macro_use] extern crate conrod;

use conrod::backend::piston::{self,Window,WindowEvents,OpenGL};
```

Eventhough we're not goin to use any macros in this section, add `#[macro_use]` also and I'll tell you later why it is a 'thing' you'll need!

**Definitions:**
I'm not sure about you, but I prefer to have my initial window dimensions being simple to refer to, that's why I put them up top as `const`.
```rust
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
```
Next thing should be to define your prefered version of OpenGL. It's not mandatory, but a good practice to do so! In Conrod OpenGL is an `enum` type so you have a predefined set of versions that can be used. To check out from what you can choose from, click [here](http://docs.piston.rs/conrod/conrod/backend/piston/enum.OpenGL.html).
```rust
let opengl = OpenGL::V3_2;
```
To get a nice window we are going to use something called 'method chaining' or 'builder pattern'. Think about it like ordering some food! You first tell the cashier you want pizza, than you say "Yo, I love spicey food, add some pepperoni, and coke too".
```rust
let my_dinner = Restaurant::Order::new()
	.pizza()
	.with_extra("pepperoni")
	.drink("coke")
	.done();
```

This way we're going to tell Conrod that the window should
* have the dimensions specified,
* use the OpenGL version specified,
* quit on pressing the 'esc' key.

```rust
let mut window: Window = piston::window::WindowSettings::new("Guessing Game",[WIDTH,HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();
```
As you can see, `new()` takes the title as its first argument and the dimensions as its secound. You can also see, that we indicate the end of the build pattern by invoking the function `build()`, this will actually create the window with the given parameters. There's many more thing you can mess around with, see [this](http://docs.piston.rs/conrod/conrod/backend/piston/window/struct.WindowSettings.html?search=) for more.

We also need to get the events from the window, for that we have to create an event iterator.
```rust
let mut events = WindowEvents::new();
```
The only thing left is to write a loop in which we can check the window events, and it also keeps are window open.
```rust
while let Some(event) = window.next_event(&mut events) {
	// ...
}
```
We are going to use this loop to draw, update and do pretty much everything we need.

**The code so far:**
```rust
#[macro_use] extern crate conrod;
use conrod::backend::piston::{self,Window,WindowEvents,OpenGL};

fn main() {
	const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

	// Specify OpenGL version
    let opengl = OpenGL::V3_2;

	// Create window using the 'builder pattern'
    let mut window: Window = piston::window::WindowSettings::new("Guessing Game",[WIDTH,HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

	// Create an event loop iterator
    let mut events = WindowEvents::new();

    while let Some(event) = window.next_event(&mut events) {
		// ...
    }
}
```

###Application data
In this section our goal is to define what kind of data are we going to track. This is something you may not have to do in traditional, retained mode GUI systems as widgets store many of that data. However, Conrod is an IMGUI system of which the most mentionable is that IMGUI widgets does not store state. It also has a bright side, you never have to syncronise data between your applications inner state and the data shown to the user.

In my oppinion, we only need to take care of a few things in this case, like:
* what is the secret number
* how many guesses left
* game ended

Let's create a separate file, which I'm going to call `app_data.rs`. The secret number is going to be stored here, so let's import `rand` here quickly, and define data for all the bullet points above!
```rust
extern crate rand;
use self::rand::Rng;

pub struct GuessingGame {
    secret_num:i32,
    no_guess:i32,
    win:bool,
}
```
Now, let's think about all the logic we'll need to interact with our data... I would say we need these functions:
```rust
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
```
I think it's quite self-explanatory which function does what, so let's move on to showing the data to the user.
Also before you move on to the next section don't forget to add the `app_data` module to your `main.rs` file like so:
```rust
mod app_data;
use app_data::GuessingGame;
```

###Setting up the UI
Allright, go back to the `main.rs` file as the next step is going to be creating the `Ui`. This is special as `Ui` in Conrod is the data structure that keeps track of the state of every `Widget`, the `Theme` you use globally (this is for in app themeing, does not support OS theme at the moment) and many more. If your curious go look at [here](http://docs.piston.rs/conrod/conrod/struct.Ui.html)!

We're not going to create a `Ui` directly, we have the `UiBuilder` at hand to make things easy. `UiBuilder` let's you specify the ui's dimensions, theme (if none than the default is used) and widget capacity.
As the `Ui` track widgets in a graph, it can grow organically or you can help it predefineing how many widgets you're about to instantiate, this could help app launch times, so you're not expand the graph dynamically in the very first draw cycle.

Right now, lets stick with the dimensions only, as we don't know how many widgets we're about to use.

```rust
let mut ui = conrod::UiBuilder::new([WIDTH as f64,HEIGHT as f64]).build();
```

As the widget states are store inside the graph, we use `Id`s to acces specific ones.
Conrod provides an easy to use macro for instantiating all your `Ids`. You remember `#[macro_use]`, right? This macro not only creates the definition, but some functions needed to get going. Write this snippet outside of your `main`.
```rust
widget_ids! {
    struct Ids {
		// id names goes here, like:
		canvas,
	}
}
```
So, the macro will create the `Ids` struct for us, with fields with the name you write inside, in this case it's `canvas`. Each field going to represent unique numbers for all widgets, but how? Well we need to add a one more line to `main()`.
```rust
let mut ids = Ids::new(ui.widget_id_generator());
```
Let's create the functions which is going to be used to draw widgets! For simplicity, let's just put a `Canvas` on screen, it's a widget that can be used as a container for other widgets.

```rust
fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids) {
    use conrod::Widget;
    use conrod::Colorable;
    use conrod::widget::{Canvas};

    Canvas::new().color(conrod::color::WHITE).pad(50.0).set(ids.canvas, ui);
}
```
What you see here, is that we interact with the ui through something called the `UiCell`. The `UiCell` restricts you from mutating the ui in ways you shouldn't do, and provides methods on how you can mutate the ui in certain contexts. You can dig through all you can do through the `UiCell` [here](http://docs.piston.rs/conrod/conrod/struct.UiCell.html).

Let's look at the `Canvas` for a second! As you can see we're not binding a canvas object to a variable, we simply declare what kind of canvas do we want. Like I say let's make it white, I want the padding from all edges to be 50, and this is going to live under the name `canvas`. You can do [many more](http://docs.piston.rs/conrod/conrod/widget/canvas/struct.Canvas.html) things to the canvas, like giving it a title, managing the flow of child canvases and such.

We want to update the ui, so let's import one more thing!
```rust
use conrod::backend::piston::event::UpdateEvent;
```
Next step is to finally add something to our draw loop!
```rust
while let Some(event) = window.next_event(&mut events) {
        event.update(|_| set_ui(ui.set_widgets(),&ids));
}
```
If you run it, you see that it's still a black window, well... Every cycle you tell Conrod to update the canvas in the state graph, but have you ever rendered the graph aka the ui? Nope, so let's do it!

Right now we have no text to draw, but we need at least an empty text texture cache. The secound item on our list is an image map which describes the widget-image relationships. Confused? No worries, right now none of those will be used, they're just empty shells to feed them into the `draw()` function of `piston` backend.
```rust
let mut text_texture_cache = piston::window::GlyphCache::new(&mut window,0,0);
let image_map = conrod::image::Map::new();

while let Some(event) = window.next_event(&mut events) {
	// update ui first
	event.update(|_| set_ui(ui.set_widgets(),&ids));
	// then display
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
```
If you run the application now, you'll see a window popping up filled with white. So the canvas actually fills out the window, huh great! Let's test resizing the window. Oooops, you quickly realize that it's not working properly. The canvas can resizes itself up to the windows initial width and height. It's because Conrod supports multiple backends, so you have to tell it what kind of event convertions should it do. Add 3 more lines right above `event.update()`.
```rust
if let Some(e) = piston::window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
}
```
Right before you move on, play a little bit with the canvas, its color, title!

**The code so far:**
```rust
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
```
