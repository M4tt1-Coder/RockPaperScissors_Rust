# Rock-Paper-Scissors Rust

This is a Rust UI application to play Rock-Paper-Scissors on the desktop.

## Function, Technologies and Challenges

The program opens a new window, and that very fast thanks to Rust :smile:, and provides the playing field and dashboard for the status of the game.

To develop a good, reliable solution for the user interface I found [egui](https://docs.rs/egui/latest/egui/) an incredible GUI framework written in Rust. Furthermore I included [eframe](https://docs.rs/eframe/latest/eframe/), thats an egui framework crate. And last but not least I added the [rand](https://docs.rs/rand/latest/rand/) for generating random numbers.

As I said earlier, egui is a great framework for UI components and applications but Rust combined with a new framework took some time to get used to it.

## How do I get it?

The *first* step ever when you want with a repository from GitHub, you clone it!
    - So head to the folder where you want to have your copied repository situated and then use this command in an opened command in your chosen destination: 
```
    git clone https://github.com/M4tt1-Coder/RockPaperScissors_Rust.git
```

- There some other options to clone it. For that please check it on GitHub in my repository!

Now if you have Rust already installed you start immediately, if not than go [here](https://www.rust-lang.org/tools/install),  install it and come 
back afterwards.

To check if Rust is already installed enter this command in the command line:
```
    cargo version
```
It should popup an information about the version of Rust and cargo the tool for it!
For me it looks like this:
```
    cargo 1.73.0
```

To run the game write this command in the terminal: 
```
    cargo run 
```

## And now what ... ?

I think you know how to play Rock-Paper-Scissors but just to make sure before you play, you can check the rules [here](https://www.wikihow.com/Play-Rock,-Paper,-Scissors).

Now after you ran ... :
```
    cargo run
```
... you should see something like this: 
![show the start screen for the game](pictures/start-page.png)

Another page looks like this:
![give an example of the game](pictures/result-screen.png)

### First own custumization
Now you want to create your own game or app based on the existing repository?
Then take this few hints:
- Set your applications name:
```rust
    let app_name = "Your app name";
    //...
    //pass it in the running endpoint
    eframe::run_native(
        app_name,
        //...
    )
    
```
- When you want to create a component use an own function for it!
    - ... call it where you set your implementation of the App trait of egui
    ```rust
        fn display_your_content(ui: &mut Ui, app: &mut App){
            ui.horizontal(|ui| {
                    // do something
                }
            )
        }

        //then in the update function
        fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
            display_your_content(ui, app);
        }
    ```

- Work with properties of the app struct you implemented to handle logic in the program:
```rust
    //if you look in my code there are more examples
    //...
    struct MyApp{
        user: Vec<User>,
        id: i8,
        //...
    }
```
