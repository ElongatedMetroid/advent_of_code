use std::{process, fs::File, io::{BufReader, BufRead}};

pub struct Submarine {
    horizontal_pos: isize,
    depth: isize,
    aim: isize,
}

impl Submarine {
    pub fn new() -> Self {
        Submarine {
            horizontal_pos: 0,
            depth: 0,
            aim: 0,
        }
    }
    pub fn execute(&mut self, command: &Command) {
        match command.kind {
            CommandKind::Forward => {
                self.forward(command.x);
            }
            CommandKind::Up => {
                self.up(command.x);
            }
            CommandKind::Down => {
                self.down(command.x);
            }
        }
    }
    pub fn forward(&mut self, x: isize) {
        self.horizontal_pos += x;
        self.depth += self.aim * x;
    }
    pub fn up(&mut self, x: isize) {
        self.aim -= x;
    }
    pub fn down(&mut self, x: isize) {
        self.aim += x;
    }

    pub fn solution(&self) -> isize {
        self.horizontal_pos * self.depth
    }
}

enum CommandKind {
    Forward,
    Up,
    Down,
}

pub struct Command {
    kind: CommandKind,
    x: isize,
}

impl Command {
    /// Parse a str into a command
    pub fn parse(command_line: &str) -> Result<Command, Box<dyn std::error::Error>> {
        let mut tokens = command_line.split_whitespace();

        let kind = match tokens.next() {
            Some(kind) => {
                match kind {
                    "forward" => CommandKind::Forward,
                    "up" => CommandKind::Up,
                    "down" => CommandKind::Down,
                    _ => return Err("Invalid command kind!".into())
                }
            }
            None => {
                return Err("Empty command!".into());
            }
        };

        let x: isize = match tokens.next() {
            Some(x) => x.parse()?,
            None => {
                return Err("Argument `x` not provided".into());
            }
        };

        Ok(Command { kind, x })
    }
}

fn main() {
    let f = File::open("input").unwrap_or_else(|error| {
        println!("Could not open file: {error}");
        process::exit(1);
    });
    let reader = BufReader::new(f);

    let mut submarine = Submarine::new();
    for line in reader.lines() {
        let command = line.unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(1);
        });

        let command = Command::parse(command.as_str()).unwrap_or_else(|error| {
            eprintln!("Could not parse command: {error}");
            process::exit(1);
        });

        submarine.execute(&command);
    }

    println!("{}", submarine.solution());
}
