use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct BinOpApp {
    #[clap(subcommand)]
    command: BinOpCommand,
}

#[derive(Subcommand, Debug)]
enum BinOpCommand {
    #[clap(about = "Add two numbers")]
    Add {
        #[clap(short = 'n', help = "The numbers to add", )]
        numbers: Option<String>,
    },
    #[clap(about = "Subtract two numbers")]
    Sub {
        #[clap(short = 'n', help = "The numbers to subtract")]
        numbers: Option<String>,
    },
}

fn main() {
    let app = BinOpApp::parse();
    match app.command {
        BinOpCommand::Add { numbers } => {
            let numbers = numbers.unwrap_or(String::from("0"));
            let sum = string_to_numbers(numbers).into_iter().sum::<i32>();

            println!("{}", sum);
        }
        BinOpCommand::Sub { numbers } => {
            let numbers = numbers.unwrap_or(String::from("0"));
            let num_vec = string_to_numbers(numbers);

            let mut sub = num_vec[0];
            for n in num_vec[1..].iter() {
                sub -= n;
            }

            println!("{}", sub);
        }
    }
}

fn string_to_numbers(numbers: String) -> Vec<i32> {
    numbers
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
