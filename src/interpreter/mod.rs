use std::io::{Read, Write, stdin, stdout};

mod tokenizer;
use tokenizer::Data;

struct State {
    cursor: usize,
    cells: Vec<u8>,
}

pub fn run(input: &str) {
    let mut state = State {
        cursor: 0,
        cells: vec![0; 1000],
    };

    let instructions: Vec<Data> = tokenizer::to_data(input);
    run_instructions(&instructions, &mut state);
}

impl State {
    fn add(&mut self) {
        self.cells[self.cursor] = self.cells[self.cursor].wrapping_add(1)
    }

    fn substract(&mut self) {
        self.cells[self.cursor] = self.cells[self.cursor].wrapping_sub(1)
    }

    fn move_l(&mut self) {
        self.cursor = match self.cursor {
            0 => self.cells.len() - 1,
            n => n - 1,
        };
    }

    fn move_r(&mut self) {
        self.cursor = match self.cursor + 1 {
            n if n == self.cells.len() => 0,
            n => n,
        };
    }
}

fn run_instructions(instructions: &Vec<Data>, state: &mut State) {
    for i in instructions {
        match i {
            Data::Add => state.add(),
            Data::Substract => state.substract(),
            Data::MoveRight => state.move_r(),
            Data::MoveLeft => state.move_l(),
            Data::Loop(i) => loop_instructions(i, state),
            Data::Print => print_char(state.cells[state.cursor]),
            Data::Read => state.cells[state.cursor] = read_char(),
        }
    }
}

fn loop_instructions(instructions: &Vec<Data>, state: &mut State) {
    while state.cells[state.cursor] != 0 {
        run_instructions(instructions, state)
    }
}

fn print_char(out: u8) {
    print!("{}", out as char);
    _ = stdout().flush();
}

fn read_char() -> u8 {
    stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .unwrap()
}
