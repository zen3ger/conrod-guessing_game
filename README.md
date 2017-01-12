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

**Imports**
Open up your `main.rs` file and add these lines to the top:
```rust
#[macro_use] extern crate conrod;

use conrod::backend::piston::{self,Window,WindowEvents,OpenGL};
```

Eventhough we're not goin to use any macros in this section, add `#[macro_use]` also and I'll tell you later why it is a 'thing' you'll need!

**Definitions**
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
In this section are goal is to define what kind of data are we going to track. This is something you may not have to do in traditional, retained mode GUI systems as widgets store many of that data. However, Conrod is an IMGUI system of which the most mentionable is that IMGUI widgets does not store state. It also has a bright side, you never have to syncronise data between your applications inner state and the data shown to the user.

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
