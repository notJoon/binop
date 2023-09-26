use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct BinOpApp {
    #[clap(subcommand)]
    command: BinOpCommand,
}

#[derive(Subcommand, Debug, PartialEq)]
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

#[cfg(test)]
mod binop_test {
    use super::*;

    #[test]
    fn test_add() {
        let app = BinOpApp::parse_from(&["binop", "add", "-n", "1 2 3 4 5"]);
        assert_eq!(app.command, BinOpCommand::Add { numbers: Some(String::from("1 2 3 4 5")) });
    }

    #[test]
    fn test_sub() {
        let app = BinOpApp::parse_from(&["binop", "sub", "-n", "1 2 3 4 5"]);
        assert_eq!(app.command, BinOpCommand::Sub { numbers: Some(String::from("1 2 3 4 5")) });
    }

    #[test]
    fn test_add_no_numbers() {
        let app = BinOpApp::parse_from(&["binop", "add"]);
        assert_eq!(app.command, BinOpCommand::Add { numbers: None });
    }

    #[test]
    fn test_string_to_numbers() {
        let numbers = String::from("1 2 3 4 5");
        let num_vec = string_to_numbers(numbers);

        assert_eq!(num_vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sum_parsed_numbers() {
        let numbers = String::from("1 2 3 4 5");
        let num_vec = string_to_numbers(numbers);

        let sum = num_vec.into_iter().sum::<i32>();

        assert_eq!(sum, 15);
    }
}