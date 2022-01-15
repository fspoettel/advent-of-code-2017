use itertools::Itertools;
use std::collections::HashMap;

struct State {
    state: char,
    falsy: Rules,
    truthy: Rules,
}

struct Rules {
    write: bool,
    direction: Direction,
    target_state: char,
}

enum Direction {
    Left,
    Right,
}

struct Program {
    default_state: char,
    checksum_at: usize,
    states: HashMap<char, State>,
}

type Tape = HashMap<isize, bool>;

fn parse(input: &str) -> Option<Program> {
    let mut lines = input.lines();

    let default_state = lines.next()?.chars().nth_back(1)?;
    let checksum_at = lines.next()?.split(' ').nth_back(1)?.parse().ok()?;

    let states = lines.chunks(10)
        .into_iter()
        .filter_map(|mut lines| {
            let state = lines.nth(1)?.chars().nth_back(1)?;

            let rules_chunks = lines.chunks(4);

            let mut rules = rules_chunks.into_iter().filter_map(|mut lines| {
                let write = lines.nth(1)?.chars().nth_back(1)? == '1';
                let direction = match lines.next()?.split(' ').last()? {
                    "right." => Direction::Right,
                    "left." => Direction::Left,
                    s => panic!("unexpected direction: {}", s),
                };
                let target_state = lines.next()?.chars().nth_back(1)?;
                Some(Rules {
                    write,
                    direction,
                    target_state,
                })
            });

            Some(State {
                state,
                falsy: rules.next()?,
                truthy: rules.next()?,
            })
        })
        .fold(HashMap::new(), |mut acc, s| {
            acc.insert(s.state, s);
            acc
        });

    Some(Program {
        default_state,
        checksum_at,
        states,
    })
}

pub fn part_one(input: &str) -> usize {
    let program = parse(input).unwrap();

    let mut tape: Tape = HashMap::new();
    let mut state = program.default_state;
    let mut cursor = 0;

    for _ in 0..program.checksum_at {
        let current_state = program.states.get(&state).unwrap();
        let current = tape.entry(cursor).or_default();

        let rules = match current {
            false => &current_state.falsy,
            true => &current_state.truthy,
        };

        *current = rules.write;
        state = rules.target_state;
        cursor = match rules.direction {
            Direction::Left => cursor - 1,
            Direction::Right => cursor + 1,
        }
    }

    tape.values().filter(|s| **s).count()
}

pub fn part_two(_: &str) -> u32 {
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 25), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 25);
        assert_eq!(part_one(&input), 3);
    }
}
