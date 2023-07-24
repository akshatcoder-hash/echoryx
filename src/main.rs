use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn main() {
    loop {
        // using '>' as the prompt
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after whitespace is passedd as arguments

        let mut parts = input.trim().split_whitespace();

        let command = parts.next().unwrap();
        let args = parts;

        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();

        child.wait();
    }
}
