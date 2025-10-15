use std::iter::Peekable;

pub enum Data {
    MoveRight,
    MoveLeft,
    Add,
    Substract,
    Print,
    Read,
    Loop(Vec<Data>),
}

pub fn to_data(input: &str) -> Vec<Data> {
    let mut instructions = input.chars().peekable();
    parse_tokens(&mut instructions)
}

fn parse_tokens(instructions: &mut Peekable<std::str::Chars>) -> Vec<Data> {
    let mut tokens: Vec<Data> = Vec::new();

    while let Some(c) = instructions.next() {
        match c {
            '+' => tokens.push(Data::Add),
            '-' => tokens.push(Data::Substract),
            '>' => tokens.push(Data::MoveRight),
            '<' => tokens.push(Data::MoveLeft),
            '.' => tokens.push(Data::Print),
            ',' => tokens.push(Data::Read),
            '[' => tokens.push(Data::Loop(parse_tokens(instructions))),
            ']' => break,
            '\n' | ' ' | '\t' => continue, // so it doesn't break with indent/whitespaces
            _ => break,                    // ignore contents if there's a comment
        }
    }

    tokens
}
