use clap::Parser;
use guessing_game::guessing_game_function;

mod guessing_game;

#[derive(Parser, Debug)]
#[clap(author = "Peter Storm", version, about)]

struct Args {
    #[arg(short, long)]
    program: Option<String>,
}

fn main() {
    let args = Args::parse();
    match args.program {
        Some(program) => {
            match program.as_str() {
                "guessing_game" => guessing_game_function(),
                _ => println!("Unknown program: {}", program)
            }
        }
        None => println!("Hello world! No program specified.")
    }
}
