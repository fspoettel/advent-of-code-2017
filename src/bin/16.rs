use itertools::Itertools;
use std::collections::VecDeque;

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .filter_map(|s| match s.chars().next()? {
            's' => Some(Instruction::Spin(s[1..].parse().unwrap())),
            'x' => {
                let mut x = s[1..].split('/');
                Some(Instruction::Exchange(
                    x.next()?.parse().unwrap(),
                    x.next()?.parse().unwrap(),
                ))
            }
            'p' => {
                let mut x = s[1..].split('/');
                Some(Instruction::Partner(
                    x.next()?.chars().next()?,
                    x.next()?.chars().next()?,
                ))
            }
            _ => None,
        })
        .collect()
}

fn a_to_p() -> VecDeque<char> {
    (0..16).map(|i| (i as u8 + 97) as char).collect()
}

fn dance(position: &mut VecDeque<char>, instructions: &[Instruction]) {
    for instruction in instructions {
        match instruction {
            Instruction::Spin(s) => position.rotate_right(*s),
            Instruction::Exchange(i, j) => position.swap(*i, *j),
            Instruction::Partner(a, b) => {
                let (i, _) = position.iter().find_position(|x| *x == a).unwrap();
                let (j, _) = position.iter().find_position(|x| *x == b).unwrap();
                position.swap(i, j);
            }
        }
    }
}

pub fn part_one(input: &str) -> String {
    let instructions = parse(input);
    let mut position = a_to_p();

    dance(&mut position, &instructions);
    String::from_iter(position)
}

pub fn part_two(input: &str) -> String {
    let instructions = parse(input);
    let initial_position = a_to_p();

    let mut current_position = initial_position.clone();
    let mut i: usize = 0;

    loop {
        dance(&mut current_position, &instructions);
        i += 1;
        if current_position == initial_position {
            break;
        }
    }

    for _ in 0..((1000000000 - i) % i) {
        dance(&mut current_position, &instructions);
    }

    String::from_iter(current_position)
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 16), part_one, part_two)
}
