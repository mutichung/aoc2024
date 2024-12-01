use clap::{Parser, ValueEnum};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum TestCase {
    Example,
    Test,
}

impl FromStr for TestCase {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "example" => Ok(TestCase::Example),
            "test" => Ok(TestCase::Test),
            _ => Err(format!("Invalid test case type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Date of the program to run
    #[arg(short, long)]
    day: u32,

    /// Mode to run. Choose from example or test.
    #[arg(short, long, value_enum)]
    mode: TestCase,
}

fn main() {
    let args = Args::parse();
    println!("Running day {} on test case {:?}", args.day, args.mode);
}
