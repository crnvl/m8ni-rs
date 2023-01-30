use std::fs;
use std::str::Lines;

fn main() {
    let script = fs::read_to_string("scripts/tests/jump.m8").expect("File couldn't be read");

    let mut lines: Lines = script.lines();

    let mut memory = [0; 16];
    let mut iter = 0;
    while let Some(line) = lines.next() {
        iter += 1;
        let mut words = line.split_whitespace();
        let command = words.next().unwrap();
        let pos = words.next().unwrap().parse::<usize>().unwrap();
        match command.to_lowercase().as_str() {
            "inc" => inc(&mut memory, pos),
            "dec" => dec(&mut memory, pos),
            "is_zero" => is_zero(&mut memory, pos, iter, &mut lines),
            "jump" => jump(pos, iter, &mut lines),
            _ => println!("Command not found"),
        }
    }
    print!("{:?}", memory)
}

// instructions
fn inc(memory: &mut [u8], pos: usize) {
    memory[pos] += 1;
}

fn dec(memory: &mut [u8], pos: usize) {
    memory[pos] -= 1;
}

fn is_zero(memory: &mut [u8], pos: usize, iter: usize, lines: &mut Lines) {
    if memory[pos] == 0 {
        println!("Memory is zero");
        jump(pos, iter, lines);
    } else {
        println!("Memory is not zero");
    }
}

fn jump(line: usize, iter: usize, lines: &mut Lines) {
    let n = iter - line;
    println!("Jumping to line {}", iter - n);
    if line > iter {
        for _ in 0..n {
            lines.next();
        }
    } else {
        for _ in 0..n {
            lines.next_back();
        }
    }
}
