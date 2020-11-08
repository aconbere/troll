use std::io::{stdin,stdout,Write};

enum State {
    Start,
    Cavern,
    Troll,
    Die
}

struct Game {
    state: State
}

fn write(msg: &str) {
    println!("{}", msg);
    stdout().flush().unwrap();
}


fn read() -> String {
    print!("> ");
    stdout().flush().unwrap();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let mut game = Game { state: State::Start };

    loop {
        match game.state {
            State::Start => {
                write("You are standing in a dark cavern. The walls drip over moss and mildew and a stench emanates from a doorway in the north wall.");
                game.state = State::Cavern;
            }
            State::Cavern => {
                let command = read();
                match command.as_str() {
                    "walk north" => {
                        write("You head north towards the doorway and fear fills your heart as the stench grows ever stronger.");
                        game.state = State::Troll;
                    }
                    "walk east" => {
                        write("The walls block your way.\n");
                    }
                    "walk south" => {
                        write("That's where you came from! There's no sense retreating.\n");
                    }
                    "walk west" => {
                        write("The walls block your way.\n");
                    }
                    _ => {
                        write("I don't understand.\n");
                    }
                }
            }
            State::Troll => {
                write("A TROLL stands in the center of a large room! You can attack or run away. What will you do?!");

                let command = read();

                match command.as_str() {
                    "attack" => {
                        write("You battled the troll but the troll fights back. It is much strong than you and it kills you easily."); 
                        game.state = State::Die;
                    }
                    "run away" => {
                        write("The Troll chases after you. It's much faster than you and it kills you easily."); 
                        game.state = State::Die;
                    }
                    _ => {
                        write("Your indecision is kills you.");
                        game.state = State::Die;
                    }
                }
            }

            State::Die => {
                write("You die, you are stoopid");
                break;
            }
        }
    }
}

