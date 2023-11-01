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

